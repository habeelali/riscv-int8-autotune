# Figure gallery

All numeric figures are rendered from saved benchmark data. Regenerate with
`python3 scripts/presentation.py` after installing `requirements.txt`. Each chart
has SVG, PNG and PDF versions in `docs/figures/project/`. Input file SHA-256 hashes
are retained in [data-provenance.json](figures/project/data-provenance.json).

## Physical inference timing

![Complete-model hardware timing](figures/project/hardware.svg)

Actual V5 board measurements, four input medians per model and three hard-reset
sessions. C means the fastest of three tested C variants per input.

## Every Rust timing observation

![Physical timing repeatability](figures/project/repeatability.svg)

The vertical scales are deliberately zoomed to reveal variation. Every one of
the 120 Rust RUNs is visible; no jitter or confidence interval was fabricated.

## Generic Rust versus generated loops

![QEMU instruction measurements](figures/project/instructions.svg)

Logarithmic scale; executed guest instructions, not hardware latency. The generic
Rust baseline is retained even though it loses.

## Archived flash reduction

![Flash before and after](figures/project/flash.svg)

This earlier model configuration is separate from V5 hardware; its instruction
counts remain identical while function sharing saves generated code.

## Code/performance trade-off

![Cache-tile trade-offs](figures/project/tradeoffs.svg)

Candidate measurements include staging regressions. Simulated points do not
establish physical reliability of the corresponding firmware.

## Social assets

[LinkedIn cover](../media/linkedin/linkedin-cover.png) ·
[Four-slide PDF](../media/linkedin/carousel.pdf) ·
[Post draft](../media/linkedin/POST.md).
The social kit carries the same one-board scope and unresolved-failure caveats.
