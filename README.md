# intel-tsx-rtm

This crates provides a simple set of wrappers around Intel's TSX RTM instructions and associated intrinsics. It needs a C compiler to create a small shim. This is important because Rust's compiler does not like code with multiple returns. It does not depend on your compiler having the necessary headers (`<immintrin.h>`), and so can work with older compilers and other Operating Systems.

It uses third-party self-modifying code (Andi Kleen's `tsx-tools`) to provide runtime detection of CPUs without TSX and fallback to non-hardware paths.


## Licensing

The license for this project is MIT.

[intel-tsx-rtm]: https://github.com/lemonrock/intel-tsx-rtm "intel-tsx-rtm GitHub page"
