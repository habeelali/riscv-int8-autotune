"""Fail-closed selection tests; these synthetic records are never benchmark data."""
import copy
import importlib.util
from pathlib import Path
import sys
import unittest

sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'scripts'))
AVAILABLE = all(importlib.util.find_spec(m) for m in ('tflite','ai_edge_litert','numpy','PIL'))


@unittest.skipUnless(AVAILABLE,'install requirements-model.txt for full-model flash tests')
class FlashConstraints(unittest.TestCase):
    def setUp(self):
        from tune_flash import constraints
        self.check = constraints
        self.reference = dict(correct=True,text_bytes=100,rodata_bytes=200,static_ram_bytes=80,
                              samples=[dict(name='fixture',output=[1,2],mismatches=0,instructions=2000,stack_watermark_bytes=32)],
                              layers=[dict(sample=0,layer=0,instructions=1000,mismatches=0)])
        self.good = copy.deepcopy(self.reference)
        self.good['text_bytes'] -= 10

    def test_smaller_identical_performance_is_eligible(self):
        self.assertEqual(self.check(self.good,self.reference),[])

    def test_rejects_single_instruction_ram_stack_or_correctness_regressions(self):
        changes = [lambda r:r['samples'][0].update(instructions=2001),
                   lambda r:r['layers'][0].update(instructions=1001),
                   lambda r:r['samples'][0].update(stack_watermark_bytes=33),
                   lambda r:r.update(static_ram_bytes=81),
                   lambda r:r['samples'][0].update(output=[1,3]),
                   lambda r:r['layers'][0].update(mismatches=1),
                   lambda r:r['layers'][0].update(layer=1),
                   lambda r:r.update(layers=[]),
                   lambda r:r.update(rodata_bytes=211)]
        for change in changes:
            bad=copy.deepcopy(self.good);change(bad)
            self.assertTrue(self.check(bad,self.reference))


if __name__ == '__main__':
    unittest.main()
