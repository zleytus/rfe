#include "rfe.h"
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

void print_spectrum_analyzer_info(const SpectrumAnalyzer *spectrum_analyzer) {
    char port_name[100];
    rfe_spectrum_analyzer_port_name(spectrum_analyzer, port_name, 100);
    printf("Spectrum Analyzer (%s):\n", port_name);

    char firmware_version[50];
    rfe_spectrum_analyzer_firmware_version(spectrum_analyzer, firmware_version, 50);
    printf("\tFirmware version: %s\n", firmware_version);

    char serial_number[50];
    rfe_spectrum_analyzer_serial_number(spectrum_analyzer, serial_number, 50);
    printf("\tSerial number: %s\n", serial_number);

    SpectrumAnalyzerConfig config;
    if (rfe_spectrum_analyzer_config(spectrum_analyzer, &config) == RESULT_SUCCESS) {
        printf("\tCenter: %" PRIu64 " Hz\n", config.center_hz);
        printf("\tSpan: %" PRIu64 " Hz\n", config.span_hz);
        printf("\tStart: %" PRIu64 " Hz\n", config.start_hz);
        printf("\tStop: %" PRIu64 " Hz\n", config.stop_hz);
        printf("\tStep: %" PRIu64 " Hz\n", config.step_hz);
        if (config.rbw_hz > 0) {
            printf("\tRBW: %" PRIu64 " Hz\n", config.rbw_hz);
        }
        printf("\tSweep points: %u\n", config.sweep_points);
        printf("\tAmp offset: %d dB\n", config.amp_offset_db);
        printf("\tMode: %d\n", config.mode);
        printf("\tCalc mode: %d\n", config.calc_mode);
        printf("\tMin freq: %" PRIu64 " Hz\n", config.min_freq_hz);
        printf("\tMax freq: %" PRIu64 " Hz\n", config.max_freq_hz);
        printf("\tMax span: %" PRIu64 " Hz\n", config.max_span_hz);
        printf("\tMin amp: %d dBm\n", config.min_amp_dbm);
        printf("\tMax amp: %d dBm\n", config.max_amp_dbm);
    }

    SpectrumAnalyzerRadioModule active_radio_module;
    if (rfe_spectrum_analyzer_active_radio_module(spectrum_analyzer, &active_radio_module) ==
        RESULT_SUCCESS) {
        char model_name[100];
        rfe_spectrum_analyzer_model_name(active_radio_module.model, model_name, 100);
        printf("\tActive radio module model: %s\n", model_name);
    }

    SpectrumAnalyzerRadioModule inactive_radio_module;
    if (rfe_spectrum_analyzer_inactive_radio_module(spectrum_analyzer, &inactive_radio_module) ==
        RESULT_SUCCESS) {
        char model_name[100];
        rfe_spectrum_analyzer_model_name(inactive_radio_module.model, model_name, 100);
        printf("\tInactive radio module model: %s\n", model_name);
    }

    printf("\n");
}

void print_signal_generator_info(const SignalGenerator *signal_generator) {
    char port_name[100];
    rfe_signal_generator_port_name(signal_generator, port_name, 100);
    printf("Signal Generator (%s):\n", port_name);

    char firmware_version[50];
    rfe_signal_generator_firmware_version(signal_generator, firmware_version, 50);
    printf("\tFirmware version: %s\n", firmware_version);

    char serial_number[50];
    rfe_signal_generator_serial_number(signal_generator, serial_number, 50);
    printf("\tSerial number: %s\n", serial_number);

    SignalGeneratorConfig config;
    if (rfe_signal_generator_config(signal_generator, &config) != RESULT_SUCCESS) {
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
    SpectrumAnalyzerList *spectrum_analyzers = rfe_spectrum_analyzer_connect_all();
    if (spectrum_analyzers) {
        for (uintptr_t i = 0; i < rfe_spectrum_analyzer_list_len(spectrum_analyzers); ++i) {
            const SpectrumAnalyzer *spectrum_analyzer =
                rfe_spectrum_analyzer_list_get(spectrum_analyzers, i);
            print_spectrum_analyzer_info(spectrum_analyzer);
        }
        rfe_spectrum_analyzer_list_free(spectrum_analyzers);
    }

    SignalGeneratorList *signal_generators = rfe_signal_generator_connect_all();
    if (signal_generators) {
        for (uintptr_t i = 0; i < rfe_signal_generator_list_len(signal_generators); ++i) {
            const SignalGenerator *signal_generator =
                rfe_signal_generator_list_get(signal_generators, i);
            print_signal_generator_info(signal_generator);
        }
        rfe_signal_generator_list_free(signal_generators);
    }

    return EXIT_SUCCESS;
}