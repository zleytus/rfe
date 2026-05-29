# rfe-ffi

`rfe-ffi` exposes the Rust [`rfe`](../lib/) library through a C-compatible API. It builds static and dynamic libraries named `rfe` and provides the generated C header [`include/rfe.h`](include/rfe.h).

## Usage

```c
#include "rfe.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

int main(void) {
    SpectrumAnalyzer *rfe = rfe_spectrum_analyzer_connect();
    if (!rfe) {
        fprintf(stderr, "Failed to connect to an RF Explorer\n");
        return EXIT_FAILURE;
    }

    uint16_t sweep_len = rfe_spectrum_analyzer_sweep_len(rfe);
    float *sweep = malloc(sizeof(float) * sweep_len);
    Result rc = rfe_spectrum_analyzer_wait_for_next_sweep(rfe, sweep, sweep_len, NULL);

    free(sweep);
    rfe_spectrum_analyzer_free(rfe);
    return (rc == RESULT_SUCCESS) ? EXIT_SUCCESS : EXIT_FAILURE;
}
```

## Memory Ownership

Some FFI functions return heap-allocated objects or arrays. The caller owns those values and must release them with the matching free function.

| Allocate | Free |
| --- | --- |
| `rfe_spectrum_analyzer_connect` | `rfe_spectrum_analyzer_free` |
| `rfe_signal_generator_connect` | `rfe_signal_generator_free` |
| `rfe_port_names` | `rfe_free_port_names` |
| `rfe_*_screen_data` | `rfe_screen_data_free` |

## Strings

String getters write into caller-provided buffers. Length helpers return the required buffer size including the terminating null byte.

```c
uintptr_t len = rfe_spectrum_analyzer_port_name_len(rfe);
char *port_name = malloc(len);
rfe_spectrum_analyzer_port_name(rfe, port_name, len);
free(port_name);
```

## Build

Build the FFI library from the workspace root:

```bash
cargo build -p rfe-ffi --release
```

Common outputs:

| Artifact | Location |
| --- | --- |
| Header | `ffi/include/rfe.h` |
| Linux dynamic library | `target/release/librfe.so` |
| macOS dynamic library | `target/release/librfe.dylib` |
| Windows dynamic library | `target/release/rfe.dll` |
| Unix static library | `target/release/librfe.a` |
| Windows static library | `target/release/rfe.lib` |

## C Examples

C examples are available in [`bindings/rfe-c`](bindings/rfe-c/).

Build them against the dynamic library:

```bash
cd ffi/bindings/rfe-c
cmake -S . -B build -DBUILD_SHARED_LIBS=ON
cmake --build build
```

Build them against the static library:

```bash
cd ffi/bindings/rfe-c
cmake -S . -B build -DBUILD_SHARED_LIBS=OFF
cmake --build build
```
