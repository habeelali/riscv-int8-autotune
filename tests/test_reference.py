"""Independent oracle differential tests, including signed rounding boundaries."""
import ctypes,random,subprocess,sys,tempfile,unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'scripts'))
from reference import requantize,vectors
from runner import load
class ReferenceTests(unittest.TestCase):
    def test_quantization_against_upstream_c_and_rust(self):
        rng=random.Random(20261018)
        cases=[(x,m,s) for x in [-(1<<31),-(1<<30),-9,-5,-3,-1,0,1,3,5,9,(1<<30),(1<<31)-1] for m in [-(1<<31),-(1<<30),0,1,1<<30,(1<<31)-1] for s in [-30,-15,-3,-1,0]]
        cases += [(rng.randrange(-(1<<28),1<<28),rng.randrange(-(1<<31),1<<31),rng.randrange(-25,4)) for _ in range(3000)]
        with tempfile.TemporaryDirectory() as d:
            src=Path(d)/'quant.c';lib=Path(d)/'quant.so'
            src.write_text('#include "common_functions.h"\nint32_t q(int32_t x,int32_t m,int32_t s){return esp_nn_requantize(x,m,s);}\n')
            subprocess.run(['cc','-O3','-shared','-fPIC','-I'+str(ROOT/'baselines/esp-nn/src/common'),str(src),'-o',str(lib)],check=True)
            c=ctypes.CDLL(str(lib)).q;c.argtypes=[ctypes.c_int32]*3;c.restype=ctypes.c_int32
            text=''.join(f'{x} {m} {s}\n' for x,m,s in cases)
            p=subprocess.run(['cargo','run','--quiet','--locked','-p','reference'],input=text,text=True,capture_output=True,cwd=ROOT,check=True)
            actual=list(map(int,p.stdout.splitlines()));self.assertEqual(len(actual),len(cases))
            for case,r in zip(cases,actual):
                expected=c(*case)
                self.assertEqual(requantize(*case),expected,case)
                self.assertEqual(r,expected,case)
    def test_fixture_is_deterministic_and_not_all_saturated(self):
        for path in (ROOT/'workloads').glob('*.toml'):
            w=load(path);a=vectors(w);self.assertEqual(a,vectors(w))
            self.assertGreater(len(set(a[-1])),min(8,len(a[-1])))
            self.assertTrue(any(w['activation_min']<v<w['activation_max'] for v in a[-1]))
if __name__=='__main__':unittest.main()
