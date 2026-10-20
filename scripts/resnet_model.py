#!/usr/bin/env python3
"""Pinned MLPerf Tiny ResNet importer and shared scalar graph, with LiteRT oracle."""
import hashlib,json,math
from pathlib import Path
import numpy as np
import tflite
from PIL import Image
from ai_edge_litert.interpreter import Interpreter,OpResolverType
from runner import ROOT
from model_import import quant_multiplier,array_decl

SRC=ROOT/'models/cifar10-resnet'
DEST=ROOT/'results/model/cifar10-resnet'

def load():
    DEST.mkdir(parents=True,exist_ok=True)
    lock=json.loads((SRC/'source.json').read_text())
    for f in lock['files']:assert hashlib.sha256((SRC/f['file']).read_bytes()).hexdigest()==f['sha256']
    data=(SRC/'model.tflite').read_bytes();model=tflite.Model.GetRootAsModel(data,0);g=model.Subgraphs(0)
    it=Interpreter(model_content=data,experimental_preserve_all_tensors=True,experimental_op_resolver_type=OpResolverType.BUILTIN_REF,num_threads=1);it.allocate_tensors()
    names={v:k for k,v in vars(tflite.BuiltinOperator).items() if isinstance(v,int)}
    layers=[];constants='#include <stdint.h>\n'
    scale=lambda t:float(t.Quantization().Scale(0))
    zero=lambda t:int(t.Quantization().ZeroPoint(0))
    for j in range(g.OperatorsLength()):
        op=g.Operators(j);name=names[model.OperatorCodes(op.OpcodeIndex()).BuiltinCode()]
        ins=list(map(int,op.InputsAsNumpy()));out=int(op.Outputs(0));x=g.Tensors(ins[0]);y=g.Tensors(out)
        shape=x.ShapeAsNumpy().tolist();oshape=y.ShapeAsNumpy().tolist()
        l=dict(index=j,op=name,inputs=ins,output=out,shape=shape,outshape=oshape,input_zero=zero(x),output_zero=zero(y))
        if name in ['CONV_2D','FULLY_CONNECTED']:
            fc=name=='FULLY_CONNECTED';options=tflite.FullyConnectedOptions() if fc else tflite.Conv2DOptions();options.Init(op.BuiltinOptions().Bytes,op.BuiltinOptions().Pos)
            wt,bt=[g.Tensors(i) for i in ins[1:3]]
            weights=np.frombuffer(model.Buffers(wt.Buffer()).DataAsNumpy(),dtype=np.int8).reshape(wt.ShapeAsNumpy());bias=np.frombuffer(model.Buffers(bt.Buffer()).DataAsNumpy(),dtype='<i4')
            assert np.all(wt.Quantization().ZeroPointAsNumpy()==0)
            oc=oshape[-1];ic=shape[-1];h=1 if fc else shape[1];wd=1 if fc else shape[2];k=1 if fc else int(weights.shape[1]);stride=1 if fc else options.StrideH()
            assert h==wd and (fc or (k==weights.shape[2] and options.StrideH()==options.StrideW() and options.DilationHFactor()==options.DilationWFactor()==1))
            total=0 if fc or options.Padding()!=tflite.Padding.SAME else max(0,(oshape[1]-1)*stride+k-h)
            explicit=total%2!=0;pad=total//2
            q=[quant_multiplier(float(v)) for v in scale(x)*wt.Quantization().ScaleAsNumpy().astype(np.float64)/scale(y)]
            if len(q)==1:q*=oc
            assert len(q)==oc and all(-30<=s<=0 for _,s in q)
            activation=options.FusedActivationFunction();assert activation in [0,tflite.ActivationFunctionType.RELU,tflite.ActivationFunctionType.RELU6]
            lo=-128 if activation==0 else max(-128,zero(y));hi=127 if activation!=tflite.ActivationFunctionType.RELU6 else min(127,zero(y)+math.floor(6/scale(y)+.5))
            w=dict(name=f'resnet_layer_{j}',operator='fc' if fc else 'conv',height=h+(total if explicit else 0),width=wd+(total if explicit else 0),input_channels=ic,output_channels=oc,kernel=k,stride=stride,padding=0 if explicit else pad,input_offset=-zero(x),output_offset=zero(y),quant_shift=-8,activation_min=lo,activation_max=hi,seed=42,multipliers=[m for m,s in q],shifts=[s for m,s in q],isa='rv32imc',flash_budget=4000000,sram_budget=112144)
            l.update(workload=w,explicit_padding=explicit,pad_before=pad,pad_total=total)
            for key,val,typ in [('weights',weights,'int8_t'),('bias',bias,'int32_t'),('mult',w['multipliers'],'int32_t'),('shift',w['shifts'],'int32_t')]:constants+=array_decl(f'r_{key}_{j}',val,typ)
        elif name=='ADD':
            options=tflite.AddOptions();options.Init(op.BuiltinOptions().Bytes,op.BuiltinOptions().Pos)
            a,b=[g.Tensors(i) for i in ins];assert list(a.ShapeAsNumpy())==list(b.ShapeAsNumpy())==oshape
            twice=2*max(scale(a),scale(b));ls=20
            am,ass=quant_multiplier(scale(a)/twice);bm,bs=quant_multiplier(scale(b)/twice);om,os=quant_multiplier(twice/((1<<ls)*scale(y)))
            act=options.FusedActivationFunction();assert act in [0,tflite.ActivationFunctionType.RELU]
            l['add']=dict(a_offset=-zero(a),a_mult=am,a_shift=ass,b_offset=-zero(b),b_mult=bm,b_shift=bs,left_shift=ls,out_offset=zero(y),out_mult=om,out_shift=os,lo=-128 if act==0 else max(-128,zero(y)),hi=127)
        elif name=='AVERAGE_POOL_2D':
            options=tflite.Pool2DOptions();options.Init(op.BuiltinOptions().Bytes,op.BuiltinOptions().Pos)
            assert shape[1:3]==[8,8] and oshape[1:3]==[1,1] and options.FilterHeight()==options.FilterWidth()==8 and options.FusedActivationFunction()==0
            assert scale(x)==scale(y) and zero(x)==zero(y)
        elif name=='RESHAPE':assert math.prod(shape)==math.prod(oshape)
        elif name=='SOFTMAX':
            options=tflite.SoftmaxOptions();options.Init(op.BuiltinOptions().Bytes,op.BuiltinOptions().Pos);assert options.Beta()==1
            m,s=quant_multiplier(min(scale(x)*(1<<26),(1<<31)-1));l.update(multiplier=m,shift=s,diff_min=-math.floor(31*(1<<26)/(1<<s)))
        else:raise ValueError(name)
        layers.append(l)
    # Allocate activations with first-fit liveness. An output never aliases a live input.
    produced={l['output'] for l in layers};last={t:max(l['index'] for l in layers if t in l['inputs']) if any(t in l['inputs'] for l in layers) else len(layers) for t in produced}
    allocations={};active={};peak=0
    for l in layers:
        length=math.prod(l['outshape']);size=(length+15)//16*16;offset=0
        for off,n in sorted(active.values()):
            if offset+size<=off:break
            offset=max(offset,off+n)
        allocations[str(l['output'])]=offset;active[l['output']]=(offset,size);peak=max(peak,offset+size)
        for t in list(active):
            if last[t]<=l['index']:del active[t]
    assert peak<=73728
    vectors='#include <stdint.h>\n';samples=[]
    for s in range(4):
        pixels=np.asarray(Image.open(SRC/f'cifar-test-{s}.jpg').convert('RGB'),dtype=np.int16);assert pixels.shape==(32,32,3)
        image=(pixels-128).astype(np.int8)[None];it.set_tensor(int(g.Inputs(0)),image);it.invoke()
        vectors+=array_decl(f'r_image_{s}',image)
        refs=[]
        for l in layers:
            v=it.get_tensor(l['output']);assert v.dtype==np.int8
            vectors+=array_decl(f'r_expected_{s}_{l["index"]}',v);refs.append(f'r_expected_{s}_{l["index"]}')
        vectors+=f'static const int8_t *const r_expected_{s}[]={{'+','.join(refs)+'};\n'
        out=it.get_tensor(int(g.Outputs(0))).reshape(-1).tolist();samples.append(dict(index=s,name=f'CIFAR10_test_{s}',label=lock['files'][s+2]['label'],prediction=int(np.argmax(out)),output=out,input_sha256=hashlib.sha256(image.tobytes()).hexdigest()))
    vectors+='static const int8_t *const r_images[]={'+','.join(f'r_image_{s}' for s in range(4))+'};\n'
    vectors+='static const int8_t *const *const r_oracles[]={'+','.join(f'r_expected_{s}' for s in range(4))+'};\n'
    m=dict(model='MLPerf Tiny CIFAR-10 ResNet',model_sha256=hashlib.sha256(data).hexdigest(),input_tensor=int(g.Inputs(0)),output_tensor=int(g.Outputs(0)),input=[1,32,32,3],layers=layers,samples=samples,activation_offsets=allocations,activation_peak_bytes=peak,reference='LiteRT 2.1.0 BUILTIN_REF, delegates disabled, all tensors preserved')
    (DEST/'manifest.json').write_text(json.dumps(m,indent=2)+'\n');(DEST/'constants.h').write_text(constants);(DEST/'vectors.h').write_text(vectors)
    return m

def graph_source(m,backend_prefixes,parameter_header='constants.h',tuned_header=None):
    layers=m['layers'];prefixes=backend_prefixes
    s='#include <stdint.h>\n#include <string.h>\n#include "'+parameter_header+'"\n#include "vectors.h"\n'
    if tuned_header:s+='#include "'+tuned_header+'"\n'
    s+='extern int8_t activations[73728],padded[38416];\nextern void record_check(unsigned,unsigned,unsigned,unsigned,const int8_t*,const int8_t*,unsigned);\n'
    s+='extern void muriscv_nn_softmax_s8(const int8_t*,int32_t,int32_t,int32_t,int32_t,int32_t,int8_t*);\n'
    s+='extern int muriscv_nn_elementwise_add_s8(const int8_t*,const int8_t*,int,int,int,int,int,int,int,int8_t*,int,int,int,int,int,int);\n'
    def ptr(t):return 'r_images[sample]' if t==m['input_tensor'] else f'activations+{m["activation_offsets"][str(t)]}'
    for l in layers:
        if 'workload' in l:
            for p in prefixes:s+=f'extern void {p}_layer_{l["index"]}(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);\n'
    s+='typedef void (*r_fn)(const int8_t*,const int8_t*,const int32_t*,const int32_t*,const int32_t*,int8_t*);\n'
    s+='static const r_fn r_kernels['+str(len(prefixes))+'][16]={'+','.join('{'+','.join(f'{p}_layer_{l["index"]}' if 'workload'in l else '0' for l in layers)+'}' for p in prefixes)+'};\n'
    s+='void resnet_execute(unsigned backend,unsigned sample,unsigned check){\n'
    for l in layers:
        j=l['index'];length=math.prod(l['outshape']);s+='{\nconst int8_t*i='+ptr(l['inputs'][0])+';int8_t*o='+ptr(l['output'])+';\n'
        if 'workload' in l:
            w=l['workload']
            if l['explicit_padding']:
                h,wd,ic=l['shape'][1:];ph,pw=w['height'],w['width'];pad=l['pad_before']
                s+=f'for(unsigned y=0;y<{ph};y++)for(unsigned x=0;x<{pw};x++)for(unsigned c=0;c<{ic};c++){{unsigned a=y-{pad},b=x-{pad};padded[(y*{pw}+x)*{ic}+c]=(a<{h}&&b<{wd})?i[(a*{wd}+b)*{ic}+c]:{l["input_zero"]};}}\ni=padded;\n'
            parms=[]
            for key in ['weights','bias','mult','shift']:
                p=f'r_{key}_{j}'
                if tuned_header:p=f'(backend=={len(prefixes)-1}?t_{key}_{j}:r_{key}_{j})'
                parms.append(p)
            if l.get('stage_all_weights'):
                # Offline liveness proof: graph padding occupies the beginning;
                # this layer's weights occupy the otherwise unused tail.
                n=w['kernel']**2*w['input_channels']*w['output_channels']
                live=w['height']*w['width']*w['input_channels'] if l['explicit_padding'] else 0
                assert live+n<=38416, 'Staged weights overlap live padded input'
                assert len(prefixes)==1, 'Multi-backend staging must use explicit dispatch'
                s+=f'memcpy(padded+{38416-n},{parms[0]},{n});\n'
                parms[0]=f'padded+{38416-n}'
            s+=f'r_kernels[backend][{j}](i,'+','.join(parms)+',o);\n'
        elif l['op']=='ADD':
            a=l['add'];s+='muriscv_nn_elementwise_add_s8(i,'+ptr(l['inputs'][1])+','+','.join(str(a[k]) for k in ['a_offset','a_mult','a_shift','b_offset','b_mult','b_shift','left_shift'])+',o,'+','.join(str(a[k]) for k in ['out_offset','out_mult','out_shift','lo','hi'])+f',{length});\n'
        elif l['op']=='AVERAGE_POOL_2D':
            c=l['outshape'][-1];s+=f'for(unsigned c=0;c<{c};c++){{int sum=0;for(unsigned p=0;p<64;p++)sum+=i[p*{c}+c];o[c]=(sum+(sum>0?32:-32))/64;}}\n'
        elif l['op']=='RESHAPE':s+=f'memcpy(o,i,{length});\n'
        elif l['op']=='SOFTMAX':s+=f'muriscv_nn_softmax_s8(i,1,{length},{l["multiplier"]},{l["shift"]},{l["diff_min"]},o);\n'
        s+=f'if(check)record_check(1,backend,sample,{j},o,r_oracles[sample][{j}],{length});\n}}\n'
    s+='}\nconst int8_t*resnet_output(void){return activations+'+str(m['activation_offsets'][str(m['output_tensor'])])+';}\n'
    return s

if __name__=='__main__':
    m=load();print('Imported',m['model'],'activation peak',m['activation_peak_bytes']);print(json.dumps(m['samples'],indent=2))
