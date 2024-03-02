#include "rfe.h"
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

void print_spectrum_analyzer_info(const SpectrumAnalyzer *rfe) {
    uintptr_t port_name_len = rfe_spectrum_analyzer_port_name_len(rfe) + 1;
    char *port_name = malloc(sizeof(char) * port_name_len);
    rfe_spectrum_analyzer_port_name(rfe, port_name, port_name_len);
    printf("Spectrum Analyzer (%s):\n", port_name);

    char firmware_version[50];
    rfe_spectrum_analyzer_firmware_version(rfe, firmware_version, 50);
    printf("\tFirmware version: %s\n", firmware_version);

    char serial_number[50];
    rfe_spectrum_analyzer_serial_number(rfe, serial_number, 50);
    printf("\tSerial number: %s\n", serial_number);

    printf("\tCenter: %" PRIu64 " Hz\n", rfe_spectrum_analyzer_center_freq_hz(rfe));
    printf("\tSpan: %" PRIu64 " Hz\n", rfe_spectrum_analyzer_span_hz(rfe));
    printf("\tStart: %" PRIu64 " Hz\n", rfe_spectrum_analyzer_start_freq_hz(rfe));
    printf("\tStop: %" PRIu64 " Hz\n", rfe_spectrum_analyzer_stop_freq_hz(rfe));
    printf("\tStep: %" PRIu64 " Hz\n", rfe_spectrum_analyzer_step_size_hz(rfe));
    printf("\tRBW: %" PRIu64 " Hz\n", rfe_spectrum_analyzer_rbw_hz(rfe));
    printf("\tSweep points: %u\n", rfe_spectrum_analyzer_sweep_len(rfe));
    printf("\tAmp offset: %d dB\n", rfe_spectrum_analyzer_amp_offset_db(rfe));
    printf("\tMode: %d\n", rfe_spectrum_analyzer_mode(rfe));
    printf("\tCalc mode: %d\n", rfe_spectrum_analyzer_calc_mode(rfe));
    printf("\tMin freq: %" PRIu64 " Hz\n", rfe_spectrum_analyzer_min_freq_hz(rfe));
    printf("\tMax freq: %" PRIu64 " Hz\n", rfe_spectrum_analyzer_max_freq_hz(rfe));
    printf("\tMax span: %" PRIu64 " Hz\n", rfe_spectrum_analyzer_max_span_hz(rfe));
    printf("\tMin amp: %d dBm\n", rfe_spectrum_analyzer_min_amp_dbm(rfe));
    printf("\tMax amp: %d dBm\n", rfe_spectrum_analyzer_max_amp_dbm(rfe));

    SpectrumAnalyzerModel active_radio_model = rfe_spectrum_analyzer_active_radio_model(rfe);
    char active_model_name[100];
    rfe_spectrum_analyzer_model_name(active_radio_model, active_model_name, 100);
    printf("\tActive radio module model: %s\n", active_model_name);

    SpectrumAnalyzerModel inactive_radio_model = rfe_spectrum_analyzer_inactive_radio_model(rfe);
    char inactive_model_name[100];
    rfe_spectrum_analyzer_model_name(inactive_radio_model, inactive_model_name, 100);
    printf("\tInactive radio module model: %s\n", inactive_model_name);

    printf("\n");
}

void print_signal_generator_info(const SignalGenerator *rfe) {
    char port_name[100];
    rfe_signal_generator_port_name(rfe, port_name, 100);
    printf("Signal Generator (%s):\n", port_name);

    char firmware_version[50];
    rfe_signal_generator_firmware_version(rfe, firmware_version, 50);
    printf("\tFirmware version: %s\n", firmware_version);

    char serial_number[50];
    rfe_signal_generator_serial_number(rfe, serial_number, 50);
    printf("\tSerial number: %s\n", serial_number);

    SignalGeneratorConfig config;
    if (rfe_signal_generator_config(rfe, &config) != RESULT_SUCCESS) {
        printf("\tStart: %" PRIu64 " Hz\n", config.start_hz);
        printf("\tCW: %" PRIu64 " Hz\n", config.cw_hz);
        printf("\tTotal steps: %" PRIu32 " Hz\n", config.total_steps);
        printf("\tStep: %" PRIu64 " Hz\n", config.step_hz);
        printf("\tAttenuation: %d\n", config.attenuation);
        printf("\tPower level: %d\n", config.power_level);
        printf("\tSweep power steps: %" PRIu16 "\n", config.sweep_power_steps);
        printf("\tStart attenuation: %d\n", config.start_attenuation);
        printf("\tStart power level: %d\n", config.start_power_level);
        printf("\tStop attenuation: %d\n", config.stop_attenuation);
        printf("\tStop power level: %d\n", config.stop_power_level);
        printf("\tRF power: %d\n", config.rf_power);
        printf("\tSweep delay: %" PRIu64 " ms\n", config.sweep_delay_ms);
    }

    printf("\n");
}

int main() {
    SpectrumAnalyzer *spectrum_analyzer = rfe_spectrum_analyzer_connect();
    if (spectrum_analyzer) {
        print_spectrum_analyzer_info(spectrum_analyzer);
        rfe_spectrum_analyzer_free(spectrum_analyzer);
    }

    SignalGenerator *signal_generator = rfe_signal_generator_connect();
    if (signal_generator) {
        print_signal_generator_info(signal_generator);
        rfe_signal_generator_free(signal_generator);
    }

    return EXIT_SUCCESS;
}
