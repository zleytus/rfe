# `rfe-c`

C example programs that use the `rfe-ffi` C API.

## Requirements

- CMake
- C compiler
- Rust compiler

## Build

Build the examples against the dynamic library:

```bash
cmake -S . -B build -DBUILD_SHARED_LIBS=ON
cmake --build build
```

Build the examples against the static library:

```bash
cmake -S . -B build -DBUILD_SHARED_LIBS=OFF
cmake --build build
```
