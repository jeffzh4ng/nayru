# nayru
![](./nayru.gif)

**a software 2.0 compiler: rstorch -> {CUDA, TT, MLIR}**

- [models](./models/README)
- [rstorch](./rstorch/README)

Deep learning compiler that implements a Torch-like API for both training and
inference.

The first goal is to bootstrap `rstorch.tensor` and `rstorch.autograd` correctness.
This will be done by:

1. creating trusted implementations through SD and GPT2 weight reproduction in `pytorch`
2. reproducing the weights once again with `rstorch`.

The second goal is to improve speeds by implementing SIMD and CUDA support.

The third goal is to support different backend targets, namely MLIR and TT.

### References
- Shalev-Shwartz,Ben-David
- Mohri, Rostamizadeh ,Talwalkar
- Hardt, Recht
- MacKay