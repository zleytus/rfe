#include "common.h"
#include "rfe.h"
#include <stdatomic.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

void sweep_callback(Sweep sweep, void *received_sweep) {
    atomic_store_explicit((atomic_bool *)received_sweep, true, memory_order_relaxed);
    print_sweep(&sweep);
}

int main() {
    SpectrumAnalyzer *rfe = rfe_spectrum_analyzer_connect();
    if (!rfe) {
        fprintf(stderr, "Failed to connect to an RF Explorer\n");
        return EXIT_FAILURE;
    }

    atomic_bool received_sweep = false;
    rfe_spectrum_analyzer_set_sweep_callback(rfe, sweep_callback, (void *)&received_sweep);

    // Wait to receive a sweep before exiting
    while (!atomic_load_explicit(&received_sweep, memory_order_relaxed)) {
    }

    rfe_spectrum_analyzer_free(rfe);
    return EXIT_SUCCESS;
}