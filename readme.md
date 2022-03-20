# Compiler Project
This is a small project for following along the "An Incremental Approach to
Compiler Construction" paper by Abdulaziz Ghuloum.

http://scheme2006.cs.uchicago.edu/11-ghuloum.pdf

The compiler implementation is written in Rust here instead of in Scheme, with
a driver program in C.

## Dependencies
- [Cargo](https://www.rust-lang.org/learn/get-started)
- [GCC](https://gcc.gnu.org/)
- [GNU Make](https://www.gnu.org/software/make/)

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

# Run

The compiler can be run from cargo, with arguments fed after a double dash `--`:

```
cargo run -- <args>
```

The `program.s` output from the compiler is linked directly into the runtime
C-program, so to run the program after compiling with Make just start the
driver:

```
./build/driver
```

An example of a single line for regular compilation and execution:

```
cargo run -- <args> && make && ./build/driver
```
