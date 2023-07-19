# `rfe-c`

C programs using the Rust `rfe` library.

## Requirements

* CMake
* C and Rust compiler

## Build

### Statically link to `librfe.a` / `rfe.lib`

```
cmake -S . -B build
cmake --build build
```

### Dynamically link to `librfe.so` / `librfe.dylib` / `rfe.dll`

```
cmake -S . -B build -DBUILD_SHARED_LIBS=ON
cmake --build build
```
