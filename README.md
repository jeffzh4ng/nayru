# nayru
![](./nayru.gif)

**a software 2.0 compiler: rstorch -> CUDA**

- [models](./models/README)
- [rstorch](./rstorch/README)

The first goal is to bootstrap `rstorch.tensor` and `rstorch.autograd` correctness
by 1. creating a trusted implementation through SD/GPT2 weight reproduction with
`pytorch` and 2. reproducing the weights once again with `rstorch`. The second
goal is to saturate the 1/10th of a petaflop that is available in dual 3090s
bridged with nvlink. Similar to the software 1.0 compiler [din](https://github.com/jeffzh4ng/din/),
20% of the optimizations will be implemented to unlock 80% of the performance
improvements. The final goal is to improve multi machine perf with NCCL and MPI.