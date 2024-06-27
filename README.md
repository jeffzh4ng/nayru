# nayru
![](./nayru.gif)

**nayru: software 2.0 compiler: PyTorch -> CUDA**

- [models](./models/README)
- [rstorch](./rstorch/README)

Deep learning compiler that implements a Torch-like API for both training and
inference. The first goal is to bootstrap `rstorch`'s `tensor` and `autograd`
correctness by reproducing SD and GPT2. The second goal is to improve training
and inference speeds by implementing SIMD and CUDA support. The third goal is
to support different backend targets, like TT and ONNX. For more details on
supervised learning, check out README.tex

### References
- Shalev-Shwartz,Ben-David
- Hardt, Recht
- MacKay