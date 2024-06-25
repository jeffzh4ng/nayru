# nayru
![](./nayru.gif)

**nayru: software 2.0 compiler**

- [models](./models/README)
- [rstorch](./rstorch/README)
    - [autograd]()
    - [tensor]()
    - [nayruc]()

Deep learning compiler that implements a Torch-like API for both training and
inference. Targets CUDA, TT, and ONNX. The first goal is to bootstrap correctness
of `rstorch`'s `tensor` and `autograd` library by reproducing SD and GPT2. The
second goal is to improve training and inference speeds by implementing CUDA
support. The third goal is to support different backend targets, like TT and ONNX.

### References
Starting points to refresh foundations. Recurse to other textbooks on an as-needed basis.

- The Princeton Companion to Mathematics
- Garrity's All the Mathematics You Missed