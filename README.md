# nayru
![](./nayru.gif)

**a software 2.0 compiler: rstorch -> {CUDA, TT, MLIR}**

- [models](./models/README)
- [rstorch](./rstorch/README)

The first goal is to bootstrap `rstorch.tensor` and `rstorch.autograd` correctness
by 1. creating trusted implementations through SD and GPT2 weight reproduction
in `pytorch` and 2. reproducing the weights once again with `rstorch`. The second
goal is to improve single and distributed system training performance with CUDA,
NCCL, and MPI. The third and final goal is to support two new players to the ml
compiler/chip scene: MLIR and TT.