# MLPerf Tiny CIFAR-10 ResNet

The unmodified pretrained INT8 model comes from mlcommons/tiny at the commit
and URL in source.json. Upstream licensing is retained in LICENSE.mlcommons.
The four 32×32 RGB fixtures are rows 0–3 of the CIFAR-10 test split served by
uoft-cs/cifar10. The returned files are JPEG previews provided by the dataset server; they may
differ from original dataset pixels. The importer decodes them via Pillow. Hashes and source references
are recorded in source.json. CIFAR-10 images originate from the dataset by
Alex Krizhevsky, Vinod Nair and Geoffrey Hinton; see
https://www.cs.toronto.edu/~kriz/cifar.html. Fixture attribution is separate from
the project's MIT source license. Four fixtures are a correctness/performance
sample, not a classification accuracy or official MLPerf submission.

The model has 16 operators: nine convolutions, three residual adds, global
average pooling, reshape, fully connected and softmax (operator count and exact
layer list are authoritative in the generated manifest). The importer uses
LiteRT BUILTIN_REF without delegates and retains every intermediate tensor.
