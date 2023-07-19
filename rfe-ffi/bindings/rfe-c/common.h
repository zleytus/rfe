#ifndef COMMON_H
#define COMMON_H

#include "rfe.h"
#include <inttypes.h>
#include <stdio.h>

void print_sweep(Sweep *sweep) {
    printf("Sweep { ");
    printf("amplitudes: [");
    for (uintptr_t i = 0; i < sweep->len; ++i) {
        if (i != sweep->len - 1) {
            printf("%.1f, ", sweep->amplitudes_dbm[i]);
        } else {
            printf("%.1f", sweep->amplitudes_dbm[i]);
        }
    }
    printf("], ");
    printf("timestamp_ms: %" PRId64 " }\n", sweep->timestamp);
}

#endif // COMMON_H
