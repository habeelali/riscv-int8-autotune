# Returned ESP32-C3 measurement audit

ESP32-C3 SuperMini, chip revision 0.4, 160 MHz, USB-powered, physical flash 4 MB; supplied image configured for 2 MB. Identification and flashing transcripts are retained in `../physical-return/`.

Boot 3 is the first complete capture: 868 unique layer checks and 196 unique timed runs, all zero mismatches, expected firmware build ID. Boots 4 and 5 also contain complete correct records. Boot 1 lacks early records and boot 2 is empty; neither is used.

Boots 3 and 4 have identical SHA-256 hashes. Boot 5 differs in only one timing record. Boot 1 also shares 1052 of 1054 comparable cycle counts with boot 3. These captures are not treated as independent repetitions. This is a provenance question to resolve before making repeated-boot stability claims; it does not identify the cause.

| Implementation | Mean sample-median cycles | Equivalent ms at 160 MHz |
|---|---:|---:|
| muRISCV-NN scalar | 58,511,834.0 | 365.70 |
| muRISCV-NN fixed | 58,621,555.5 | 366.38 |
| muRISCV-NN portable | 59,141,114.0 | 369.63 |
| muRISCV-NN portable fixed | 59,487,083.5 | 371.79 |
| New Rust | 73,108,929.5 | 456.93 |
| Previous Rust | 84,341,099.5 | 527.13 |
| ESP-NN | 122,521,083.0 | 765.76 |

Geometric mean across four fixtures: previous Rust / candidate = 1.1536×; fastest C / candidate = 0.8003×.

The candidate reduces cycles versus previous Rust by 13.32%, but takes 24.95% more cycles than fastest C. Thus the hardware target of 1.2× versus C has not been met.

These are reported board cycle-counter measurements, not QEMU instruction counts. Time is converted from cycles at 160 MHz; logged wall-clock values agree with that frequency. QEMU measured instruction reductions do not establish hardware cycle reductions: instruction latency, cache behavior and code placement can change ranking. A controlled hardware ablation is needed to attribute the gap.

Correctness means bit-exact agreement with the stored model oracle on four fixtures and all intermediate tensors. It does not establish classification accuracy or correctness for arbitrary inputs. Interrupts remain enabled; no confidence intervals are claimed. The combined seven-backend diagnostic image cannot establish standalone deployment flash savings.

Reproduce: `python3 scripts/analyze_hardware_return.py` (requires matplotlib). Raw uploaded logs are retained without editing.
