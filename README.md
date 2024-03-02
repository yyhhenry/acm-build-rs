# ACM Build (Rust)

`acm-build-rs` is a tool designed to construct data for regular ACM competition problems.

You can use Rust to describe the input, and use your own C++ code to solve the problem.

The tool takes your Rust input generator, the `std.cpp` file and some fixed input files. It then automatically compiles the C++ code and gives you a `data.zip` file, containing all the input and output files.

A sample problem is put in the `problems/sample` directory. See also `src/bin/build_sample.rs`.
