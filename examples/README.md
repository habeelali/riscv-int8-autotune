# Examples

[measured-dense](measured-dense/) is an actual `riscv-int8 build` result for a
256 → 128 INT8 fully connected layer, not a hand-written export or invented
performance example. Its 12 candidates were executed in QEMU; the selected
kernel uses 124,485 measured guest instructions versus 139,788 for the fastest
tested C implementation, a 1.1229× instruction-count ratio. It is below 1.2× and
is retained as measured.

The [offline report](measured-dense/report.html), [source crate](measured-dense/export/),
[all candidate records](measured-dense/candidates.json) and exact
[workload](measured-dense/workload.toml) are included. The compile cache is omitted
because it can be rebuilt. No physical latency is claimed for this example.

```bash
riscv-int8 build examples/measured-dense/workload.toml \
 --out artifacts/dense-demo --max-candidates 12
cargo check --manifest-path artifacts/dense-demo/export/Cargo.toml \
 --target riscv32imc-unknown-none-elf
```

For new shapes, use `riscv-int8 init custom.toml --operator conv` and edit the
specification. See [the tool guide](../docs/tool.md) for integration and contracts.
