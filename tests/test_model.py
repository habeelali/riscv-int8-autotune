"""Optional real-model importer checks; QEMU integration is benchmark-model."""
import importlib.util
import json
from pathlib import Path
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'scripts'))
AVAILABLE = all(importlib.util.find_spec(m) for m in ('tflite', 'ai_edge_litert', 'numpy', 'PIL'))


@unittest.skipUnless(AVAILABLE, 'install requirements-model.txt for full-model tests')
class ModelTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        from model_import import load_model
        cls.temp = tempfile.TemporaryDirectory()
        cls.dest = Path(cls.temp.name)
        cls.manifest = load_model(cls.dest)

    @classmethod
    def tearDownClass(cls):
        cls.temp.cleanup()

    def test_only_invalid_bias_axis_metadata_changes(self):
        original = (ROOT/'models/person-detection/person_detect.tflite').read_bytes()
        compatible = (self.dest/'interpreter-compatible.tflite').read_bytes()
        self.assertEqual(len(original), len(compatible))
        fixes = self.manifest['metadata_corrections']
        self.assertEqual(len(fixes), 14)
        changed = {i for i,(a,b) in enumerate(zip(original,compatible)) if a != b}
        self.assertEqual(changed, {f['byte_offset'] for f in fixes})
        for fix in fixes:
            p = fix['byte_offset']
            self.assertEqual(original[p:p+4], bytes([3,0,0,0]))
            self.assertEqual(compatible[p:p+4], bytes(4))

    def test_all_operators_are_present_and_geometry_is_exact(self):
        layers = self.manifest['layers']
        self.assertEqual(len(layers), 31)
        self.assertEqual(sum('workload' in l for l in layers), 28)
        self.assertEqual([l['op'] for l in layers[-4:]],
                         ['AVERAGE_POOL_2D','CONV_2D','RESHAPE','SOFTMAX'])
        for layer in layers:
            if 'workload' in layer:
                w = layer['workload']
                self.assertEqual((w['height']+2*w['padding']-w['kernel'])//w['stride']+1,
                                 layer['outshape'][1])
                self.assertEqual(len(w['multipliers']), layer['outshape'][-1])

    def test_reference_includes_held_out_inputs_and_intermediate_oracles(self):
        samples = self.manifest['samples']
        self.assertEqual(len(samples), 4)
        self.assertEqual([s['held_out'] for s in samples], [False,False,True,True])
        text = (self.dest/'vectors.h').read_text()
        for sample in range(4):
            for layer in range(31):
                self.assertIn(f'int8_t expected_{sample}_{layer}[',text)
        # Preserve the reference model's error on this fixture instead of rewriting labels.
        self.assertEqual(samples[0]['prediction'], 'no_person')


if __name__ == '__main__':
    unittest.main()
