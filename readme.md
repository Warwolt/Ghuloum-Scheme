# Compiler Project
This is a small project for following along the "An Incremental Approach to
Compiler Construction" paper by Abdulaziz Ghuloum.

The compiler implementation is written in Rust here instead of in Scheme, with
a driver program in C.

## Dependencies
- Cargo (for building Rust)
- GCC C compiler
- Make

# Build
To build the compiler, simply run

```
cargo build
```

To run the output from the compiler, there exists a driver C program. After the
compiler has run and the file `program.s` has been produced, the driver program
can be built using the Makefile:

```
make
```

An example of a single line for regular compilation and execution:

```
cargo run -- <args> && make && ./build/driver
```
