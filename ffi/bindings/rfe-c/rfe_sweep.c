#include "common.h"
#include "rfe.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
    SpectrumAnalyzer *rfe = rfe_spectrum_analyzer_connect();
    if (!rfe) {
        fprintf(stderr, "Failed to connect to an RF Explorer\n");
        return EXIT_FAILURE;
    }

    Sweep sweep;
    if (rfe_spectrum_analyzer_wait_for_next_sweep(rfe, &sweep) != RESULT_SUCCESS) {
        rfe_spectrum_analyzer_free(rfe);
        fprintf(stderr, "Failed to wait for next RF Explorer sweep\n");
        return EXIT_FAILURE;
    }
    print_sweep(&sweep);

    rfe_spectrum_analyzer_sweep_free(sweep);
    rfe_spectrum_analyzer_free(rfe);
    return EXIT_SUCCESS;
}