# nayru
![](./nayru.gif)

**a software 2.0 compiler: rstorch -> {CUDA, TT, MLIR}**

- [models](./models/README)
- [rstorch](./rstorch/README)

The first goal is to bootstrap `rstorch.tensor` and `rstorch.autograd` correctness
by 1. creating a trusted implementation by reproducing SD/GPT2 weights with `pytorch`
and 2. reproducing the weights once again with `rstorch`. The second goal is to
improve single and dual gpu perf with CUDA. The final goal is to improve distributed
(across NIC) perf with NCCL and MPI. A stretch goal is to support new players
to the ml infrastructure scene, such as MLIR and TT.