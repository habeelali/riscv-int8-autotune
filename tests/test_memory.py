import ctypes,subprocess,tempfile,unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
class MemoryTests(unittest.TestCase):
    def test_alignment_tails_and_guards(self):
        with tempfile.TemporaryDirectory() as d:
            lib=Path(d)/'memory.so'
            subprocess.run(['cc','-O3','-fno-builtin','-shared','-fPIC',str(ROOT/'qemu/memory.c'),'-o',lib],check=True)
            dll=ctypes.CDLL(str(lib));dll.memcpy.argtypes=[ctypes.c_void_p,ctypes.c_void_p,ctypes.c_size_t];dll.memset.argtypes=[ctypes.c_void_p,ctypes.c_int,ctypes.c_size_t]
            for n in range(130):
                for a in range(8):
                    for b in range(8):
                        src=(ctypes.c_ubyte*160)(*[(i*31+7)%256 for i in range(160)])
                        dst=(ctypes.c_ubyte*160)(*[0xa5]*160)
                        dll.memcpy(ctypes.byref(dst,a),ctypes.byref(src,b),n)
                        self.assertEqual(bytes(dst),bytes([0xa5]*a)+bytes(src)[b:b+n]+bytes([0xa5]*(160-a-n)))
                    for v in (-1,0,0xa5,0x123):
                        dst=(ctypes.c_ubyte*160)(*[0xa5]*160)
                        dll.memset(ctypes.byref(dst,a),v,n)
                        self.assertEqual(bytes(dst),bytes([0xa5]*a)+bytes([v&255]*n)+bytes([0xa5]*(160-a-n)))
if __name__=='__main__':unittest.main()
