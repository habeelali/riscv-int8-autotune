"""Strict importer for the pinned TFLM person detector, not a general TFLite engine."""
import hashlib
import json
import math
import struct
from pathlib import Path

import numpy as np
import tflite
from ai_edge_litert.interpreter import Interpreter, OpResolverType
from PIL import Image

from runner import ROOT

MODEL = ROOT / 'models/person-detection'


def quant_multiplier(value):
    fraction, shift = math.frexp(value)
    multiplier = math.floor(fraction * (1 << 31) + 0.5)
    if multiplier == 1 << 31:
        multiplier //= 2
        shift += 1
    assert multiplier >= 0 and -30 <= shift <= 30
    return multiplier, shift


def array_decl(name, values, ctype='int8_t'):
    values = np.asarray(values).reshape(-1)
    return f'static const {ctype} {name}[{len(values)}]={{' + ','.join(str(int(x)) for x in values) + '};\n'


def load_model(destination):
    destination = Path(destination)
    destination.mkdir(parents=True, exist_ok=True)
    lock = json.loads((MODEL / 'source.json').read_text())
    for entry in lock['files']:
        assert hashlib.sha256((MODEL / entry['file']).read_bytes()).hexdigest() == entry['sha256']
    original = (MODEL / 'person_detect.tflite').read_bytes()
    data = bytearray(original)
    graph = tflite.Model.GetRootAsModel(data, 0).Subgraphs(0)
    fixes = []
    # The upstream 2019 converter put depthwise weight axis=3 on rank-1 biases.
    # Axis metadata alone is corrected; tensor values/scales and all ops are unchanged.
    for index in range(graph.TensorsLength()):
        tensor = graph.Tensors(index)
        q = tensor.Quantization()
        if q is not None and q.QuantizedDimension() >= tensor.ShapeLength():
            assert tensor.Type() == tflite.TensorType.INT32 and tensor.ShapeLength() == 1
            position = q._tab.Pos + q._tab.Offset(16)
            assert q._tab.Offset(16) != 0 and q.QuantizedDimension() == 3
            fixes.append(dict(tensor=index, byte_offset=position, old_axis=3, new_axis=0))
            struct.pack_into('<i', data, position, 0)
    (destination / 'interpreter-compatible.tflite').write_bytes(data)
    model = tflite.Model.GetRootAsModel(data, 0)
    graph = model.Subgraphs(0)
    interpreter = Interpreter(model_content=bytes(data), experimental_preserve_all_tensors=True,
                              experimental_op_resolver_type=OpResolverType.BUILTIN_REF,
                              num_threads=1)
    interpreter.allocate_tensors()
    names = {v: k for k, v in vars(tflite.BuiltinOperator).items() if isinstance(v, int)}
    layers = []
    constants = '#include <stdint.h>\n'
    for index in range(graph.OperatorsLength()):
        op = graph.Operators(index)
        name = names[model.OperatorCodes(op.OpcodeIndex()).BuiltinCode()]
        inputs = list(map(int, op.InputsAsNumpy()))
        output = int(op.Outputs(0))
        x, y = graph.Tensors(inputs[0]), graph.Tensors(output)
        shape, outshape = x.ShapeAsNumpy().tolist(), y.ShapeAsNumpy().tolist()
        layer = dict(index=index, op=name, input=inputs[0], output=output, shape=shape,
                     outshape=outshape, input_zero=int(x.Quantization().ZeroPoint(0)))
        if name in ('CONV_2D', 'DEPTHWISE_CONV_2D'):
            options = tflite.Conv2DOptions() if name == 'CONV_2D' else tflite.DepthwiseConv2DOptions()
            options.Init(op.BuiltinOptions().Bytes, op.BuiltinOptions().Pos)
            wt, bt = graph.Tensors(inputs[1]), graph.Tensors(inputs[2])
            weights = np.frombuffer(model.Buffers(wt.Buffer()).DataAsNumpy(), dtype=np.int8).reshape(wt.ShapeAsNumpy())
            bias = np.frombuffer(model.Buffers(bt.Buffer()).DataAsNumpy(), dtype='<i4')
            assert np.all(wt.Quantization().ZeroPointAsNumpy() == 0)
            assert options.StrideH() == options.StrideW()
            assert options.DilationHFactor() == options.DilationWFactor() == 1
            kernel = weights.shape[1]
            assert kernel == weights.shape[2] and kernel in (1, 3)
            stride = options.StrideH()
            total = max(0, (outshape[1] - 1) * stride + kernel - shape[1]) if options.Padding() == tflite.Padding.SAME else 0
            assert shape[1] == shape[2] and outshape[1] == outshape[2]
            pad = total // 2
            # Odd SAME padding is materialized explicitly, identically for every backend.
            explicit = total % 2 != 0
            operation = 'conv' if name == 'CONV_2D' else 'depthwise'
            if name == 'DEPTHWISE_CONV_2D' and options.DepthMultiplier() != 1:
                assert shape[3] == 1
                constants += array_decl(f'weights_native_{index}', weights, 'int8_t')
                operation = 'conv'
                weights = weights.transpose(3, 1, 2, 0).copy()
                layer['lowering'] = 'depth-multiplier-8 with one input channel equals ordinary convolution'
            oc = outshape[-1]
            yscale = float(y.Quantization().Scale(0))
            effective = float(x.Quantization().Scale(0)) * wt.Quantization().ScaleAsNumpy().astype(np.float64) / yscale
            quant = [quant_multiplier(float(v)) for v in effective]
            assert len(quant) == oc and all(s <= 0 for _, s in quant)
            activation = options.FusedActivationFunction()
            assert activation in (tflite.ActivationFunctionType.NONE, tflite.ActivationFunctionType.RELU6)
            yz = int(y.Quantization().ZeroPoint(0))
            low, high = (-128, 127) if activation == 0 else (max(-128, yz), min(127, yz + math.floor(6 / yscale + 0.5)))
            w = dict(name=f'person_layer_{index}', operator=operation,
                     height=shape[1] + (total if explicit else 0), width=shape[2] + (total if explicit else 0),
                     input_channels=shape[3], output_channels=oc, kernel=kernel, stride=stride,
                     padding=0 if explicit else pad, input_offset=-layer['input_zero'], output_offset=yz,
                     quant_shift=-8, activation_min=low, activation_max=high, seed=42,
                     multipliers=[m for m, _ in quant], shifts=[s for _, s in quant],
                     isa='rv32imc', flash_budget=4000000, sram_budget=4000000)
            assert (w['height'] + 2*w['padding'] - kernel)//stride+1 == outshape[1]
            layer.update(workload=w, explicit_padding=explicit, pad_before=pad, pad_total=total)
            for key, val, typ in [('weights', weights, 'int8_t'), ('bias', bias, 'int32_t'),
                                  ('mult', w['multipliers'], 'int32_t'), ('shift', w['shifts'], 'int32_t')]:
                constants += array_decl(f'{key}_{index}', val, typ)
        elif name == 'AVERAGE_POOL_2D':
            options = tflite.Pool2DOptions()
            options.Init(op.BuiltinOptions().Bytes, op.BuiltinOptions().Pos)
            assert options.FilterHeight() == options.FilterWidth() == 3
            assert shape[1:3] == [3, 3] and outshape[1:3] == [1, 1]
            assert options.FusedActivationFunction() == 0
            assert x.Quantization().Scale(0) == y.Quantization().Scale(0)
            assert x.Quantization().ZeroPoint(0) == y.Quantization().ZeroPoint(0)
        elif name == 'RESHAPE':
            assert math.prod(shape) == math.prod(outshape)
        elif name == 'SOFTMAX':
            options = tflite.SoftmaxOptions()
            options.Init(op.BuiltinOptions().Bytes, op.BuiltinOptions().Pos)
            assert options.Beta() == 1 and outshape == [1, 2]
            multiplier, shift = quant_multiplier(min(float(x.Quantization().Scale(0)) * (1 << 26), (1 << 31)-1))
            layer.update(multiplier=multiplier, shift=shift, diff_min=-math.floor(31 * (1 << 26) / (1 << shift)))
        else:
            raise ValueError(f'Unsupported operator: {name}')
        layers.append(layer)
    (destination / 'constants.h').write_text(constants)
    vectors = '#include <stdint.h>\n'
    samples = []
    # Two upstream images plus horizontal flips held out from schedule selection.
    # No dataset accuracy estimate is inferred from these tiny fixtures.
    cases = [('person', False), ('no_person', False), ('person', True), ('no_person', True)]
    for sample_index, (name, mirrored) in enumerate(cases):
        pixels = np.asarray(Image.open(MODEL / f'{name}.bmp').convert('L'), dtype=np.int16)
        assert pixels.shape == (96, 96)
        if mirrored:
            pixels = pixels[:, ::-1].copy()
        image = (pixels - 128).astype(np.int8).reshape(1, 96, 96, 1)
        interpreter.set_tensor(int(graph.Inputs(0)), image)
        interpreter.invoke()
        vectors += array_decl(f'image_{sample_index}', image)
        expected = []
        for layer in layers:
            value = interpreter.get_tensor(layer['output'])
            vectors += array_decl(f'expected_{sample_index}_{layer["index"]}', value)
            expected.append(f'expected_{sample_index}_{layer["index"]}')
        vectors += f'static const int8_t *expected_{sample_index}[]={{' + ','.join(expected) + '};\n'
        final = interpreter.get_tensor(int(graph.Outputs(0))).reshape(-1).tolist()
        samples.append(dict(name=name+('_mirrored' if mirrored else ''), held_out=mirrored, output=final, prediction=('no_person', 'person')[int(np.argmax(final))],
                            probabilities=[(v+128)/256 for v in final]))
    vectors += 'static const int8_t *images[]={' + ','.join(f'image_{j}' for j in range(len(cases))) + '};\n'
    vectors += 'static const int8_t **oracles[]={' + ','.join(f'expected_{j}' for j in range(len(cases))) + '};\n'
    (destination / 'vectors.h').write_text(vectors)
    manifest = dict(model='TFLM MobileNet person detection', input=[1,96,96,1],
                    original_sha256=hashlib.sha256(original).hexdigest(),
                    compatible_sha256=hashlib.sha256(data).hexdigest(), metadata_corrections=fixes,
                    layers=layers, samples=samples, reference='LiteRT 2.1.0 BUILTIN_REF (no delegates)')
    (destination / 'manifest.json').write_text(json.dumps(manifest, indent=2)+'\n')
    return manifest


if __name__ == '__main__':
    manifest = load_model(ROOT / 'results/model/person-detection')
    print(json.dumps(manifest['samples'], indent=2))
