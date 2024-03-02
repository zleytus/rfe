#ifndef COMMON_H
#define COMMON_H

#include <stdint.h>
#include <stdio.h>

void print_sweep(const float *sweep, uintptr_t sweep_len) {
    printf("[");
    for (uintptr_t i = 0; i < sweep_len; ++i) {
        if (i != sweep_len - 1) {
            printf("%.1f, ", sweep[i]);
        } else {
            printf("%.1f", sweep[i]);
        }
    }
    printf("]");
}

#endif // COMMON_H
