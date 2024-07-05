# nayru
![](./nayru.gif)

**a software 2.0 compiler: rstorch -> CUDA**

- [models](./models/README)
- [rstorch](./rstorch/README)

The first goal is to bootstrap `rstorch.tensor` and `rstorch.autograd` correctness
by reproducing SD/GPT2 weights. The second goal is to saturate model training on
the the 1/10th petaflop available with two nvlinked 3090s via CUDA and Triton
kernels — 20% of the optimizations will be implemented to unlock 80% of the perf[0].
The third goal is to improve multi machine perf with NCCL and MPI.

[0]: Similar to the software 1.0 compiler [din](https://github.com/jeffzh4ng/din/),

### References
**Non-linear Optimization**
- Farina
- Aggarwal
- Peyré
- Strang

**Online Optimization**
- Hazan
- Shwartz

**Compiler Architectures**
- TVM
- XLA