"""Selection behavior for resource/performance trade-offs, without mocked results."""
import sys
from pathlib import Path
import unittest
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'scripts'))
from tool import pareto

class ToolSelectionTests(unittest.TestCase):
    def test_frontier_matches_real_measured_job(self):
        import json
        root=Path(__file__).resolve().parents[1]
        measured=json.loads((root/'results/hardware-v4/qemu-comparison.json').read_text())
        # Use real observed instructions/text to verify non-dominated membership.
        rows=[dict(name=r['name'],instructions=r['mean_instructions_raw'],flash_bytes=r['text_bytes']) for r in measured]
        names={r['name'] for r in pareto(rows)}
        self.assertIn('muriscv-fixed-asm',names)
        self.assertIn('tile16_b2p4',names)
        self.assertNotIn('tile16_b2p4_block_sram',names)
        self.assertNotIn('generated-0',names)
