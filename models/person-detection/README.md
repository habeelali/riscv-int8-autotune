# Pretrained TFLM person detector

Original pretrained INT8 model and two grayscale example images from TensorFlow
Lite Micro. `source.json` records pinned upstream URLs, commit and SHA-256 hashes.
Upstream Apache-2.0 license is retained in `LICENSE.tflite-micro`.

The original model is preserved unchanged. Its 2019 converter emitted invalid
axis=3 metadata on 14 rank-one depthwise bias tensors. `scripts/model_import.py`
produces a separately named compatible copy with those axes set to zero, as
required by current LiteRT. No tensor values, scales or operators change.

This is an explicit graph importer for this model, not a general TFLite runtime.
It supports all 31 model operators, lowering the first depth-multiplier-8 layer
to equivalent standard convolution for Rust. Both native depthwise and equivalent
convolution are tested for every C backend. The report uses the faster complete
model as comparator. Shared C pooling, softmax and graph dispatch are included
in measurements; this is not an entirely Rust inference runtime.

```sh
python3 -m pip install -r requirements-model.txt -r requirements.txt
bash scripts/reproduce-model.sh
```

The two original images are used to select among generated schedules; their
horizontal flips are held out. This is a correctness/performance fixture corpus,
not enough examples to assess dataset accuracy. The reference itself predicts
`no_person` for the supplied person fixture, and this error is preserved.

See [the complete evaluation](../../docs/model-evaluation.md).
