#include "common.h"
#include "rfe.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
    SpectrumAnalyzer *rfe = rfe_spectrum_analyzer_connect();
    if (!rfe) {
        fprintf(stderr, "Failed to connect to an RF Explorer\n");
        return EXIT_FAILURE;
    }

    uint16_t sweep_len = rfe_spectrum_analyzer_sweep_len(rfe);
    float *sweep_buf = malloc(sizeof(float) * sweep_len);
    Result rc = rfe_spectrum_analyzer_wait_for_next_sweep(rfe, sweep_buf, sweep_len, NULL);
    if (rc == RESULT_SUCCESS) {
        uint64_t start_hz = rfe_spectrum_analyzer_start_freq_hz(rfe);
        uint64_t stop_hz = rfe_spectrum_analyzer_stop_freq_hz(rfe);
        print_sweep(sweep_buf, sweep_len, start_hz, stop_hz);
    } else {
        fprintf(stderr, "Failed to wait for next RF Explorer sweep\n");
    }

    free(sweep_buf);
    rfe_spectrum_analyzer_free(rfe);
    return (rc == RESULT_SUCCESS) ? EXIT_SUCCESS : EXIT_FAILURE;
}
