#ifndef rfe_h
#define rfe_h

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define ScreenData_WIDTH_PX 128

#define ScreenData_HEIGHT_PX 64

enum Attenuation
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  ATTENUATION_ON = 0,
  ATTENUATION_OFF,
};
#ifndef __cplusplus
typedef uint8_t Attenuation;
#endif // __cplusplus

enum CalcMode
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  CALC_MODE_NORMAL = 0,
  CALC_MODE_MAX,
  CALC_MODE_AVG,
  CALC_MODE_OVERWRITE,
  CALC_MODE_MAX_HOLD,
  CALC_MODE_MAX_HISTORICAL,
  CALC_MODE_UNKNOWN = 255,
};
#ifndef __cplusplus
typedef uint8_t CalcMode;
#endif // __cplusplus

enum DspMode
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  DSP_MODE_AUTO = 0,
  DSP_MODE_FILTER,
  DSP_MODE_FAST,
  DSP_MODE_NO_IMG,
};
#ifndef __cplusplus
typedef uint8_t DspMode;
#endif // __cplusplus

enum InputStage
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  INPUT_STAGE_DIRECT = 48,
  INPUT_STAGE_ATTENUATOR30D_B = 49,
  INPUT_STAGE_LNA25D_B = 50,
  INPUT_STAGE_ATTENUATOR60D_B = 51,
  INPUT_STAGE_LNA12D_B = 52,
};
#ifndef __cplusplus
typedef uint8_t InputStage;
#endif // __cplusplus

enum Mode
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  MODE_SPECTRUM_ANALYZER = 0,
  MODE_RF_GENERATOR = 1,
  MODE_WIFI_ANALYZER = 2,
  MODE_ANALYZER_TRACKING = 5,
  MODE_RF_SNIFFER = 6,
  MODE_CW_TRANSMITTER = 60,
  MODE_SWEEP_FREQUENCY = 61,
  MODE_SWEEP_AMPLITUDE = 62,
  MODE_GENERATOR_TRACKING = 63,
  MODE_UNKNOWN = 255,
};
#ifndef __cplusplus
typedef uint8_t Mode;
#endif // __cplusplus

enum PowerLevel
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  POWER_LEVEL_LOWEST = 0,
  POWER_LEVEL_LOW,
  POWER_LEVEL_HIGH,
  POWER_LEVEL_HIGHEST,
};
#ifndef __cplusplus
typedef uint8_t PowerLevel;
#endif // __cplusplus

typedef enum Result {
  RESULT_SUCCESS = 0,
  RESULT_INCOMPATIBLE_FIRMWARE_ERROR,
  RESULT_INVALID_INPUT_ERROR,
  RESULT_INVALID_OPERATION_ERROR,
  RESULT_IO_ERROR,
  RESULT_NO_DATA,
  RESULT_NULL_PTR_ERROR,
  RESULT_TIMEOUT_ERROR,
} Result;

enum RfPower
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  RF_POWER_ON = 0,
  RF_POWER_OFF,
};
#ifndef __cplusplus
typedef uint8_t RfPower;
#endif // __cplusplus

enum SignalGeneratorModel
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  SIGNAL_GENERATOR_MODEL_RFE6_GEN = 60,
  SIGNAL_GENERATOR_MODEL_RFE6_GEN_EXPANSION = 61,
};
#ifndef __cplusplus
typedef uint8_t SignalGeneratorModel;
#endif // __cplusplus

enum SpectrumAnalyzerModel
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  SPECTRUM_ANALYZER_MODEL_RFE433_M = 0,
  SPECTRUM_ANALYZER_MODEL_RFE868_M = 1,
  SPECTRUM_ANALYZER_MODEL_RFE915_M = 2,
  SPECTRUM_ANALYZER_MODEL_RFE_W_SUB1_G = 3,
  SPECTRUM_ANALYZER_MODEL_RFE24_G = 4,
  SPECTRUM_ANALYZER_MODEL_RFE_W_SUB3_G = 5,
  SPECTRUM_ANALYZER_MODEL_RFE6_G = 6,
  SPECTRUM_ANALYZER_MODEL_RFE_W_SUB1_G_PLUS = 10,
  SPECTRUM_ANALYZER_MODEL_RFE_PRO_AUDIO = 11,
  SPECTRUM_ANALYZER_MODEL_RFE24_G_PLUS = 12,
  SPECTRUM_ANALYZER_MODEL_RFE4_G_PLUS = 13,
  SPECTRUM_ANALYZER_MODEL_RFE6_G_PLUS = 14,
  SPECTRUM_ANALYZER_MODEL_RFE_MW5G3G = 16,
  SPECTRUM_ANALYZER_MODEL_RFE_MW5G4G = 17,
  SPECTRUM_ANALYZER_MODEL_RFE_MW5G5G = 18,
  SPECTRUM_ANALYZER_MODEL_UNKNOWN = 19,
};
#ifndef __cplusplus
typedef uint8_t SpectrumAnalyzerModel;
#endif // __cplusplus

enum Temperature
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  TEMPERATURE_MINUS_TEN_TO_ZERO = 48,
  TEMPERATURE_ZERO_TO_TEN = 49,
  TEMPERATURE_TEN_TO_TWENTY = 50,
  TEMPERATURE_TWENTY_TO_THIRTY = 51,
  TEMPERATURE_THIRTY_TO_FORTY = 52,
  TEMPERATURE_FORTY_TO_FIFTY = 53,
  TEMPERATURE_FIFTY_TO_SIXTY = 54,
};
#ifndef __cplusplus
typedef uint8_t Temperature;
#endif // __cplusplus

enum TrackingStatus
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  TRACKING_STATUS_DISABLED = 0,
  TRACKING_STATUS_ENABLED,
};
#ifndef __cplusplus
typedef uint8_t TrackingStatus;
#endif // __cplusplus

enum WifiBand
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  WIFI_BAND_TWO_POINT_FOUR_GHZ = 1,
  WIFI_BAND_FIVE_GHZ,
};
#ifndef __cplusplus
typedef uint8_t WifiBand;
#endif // __cplusplus

typedef struct ScreenData ScreenData;

typedef struct SignalGenerator SignalGenerator;

typedef struct SpectrumAnalyzer SpectrumAnalyzer;

typedef struct SignalGeneratorConfig {
  uint64_t start_hz;
  uint64_t cw_hz;
  uint32_t total_steps;
  uint64_t step_hz;
  Attenuation attenuation;
  PowerLevel power_level;
  uint16_t sweep_power_steps;
  Attenuation start_attenuation;
  PowerLevel start_power_level;
  Attenuation stop_attenuation;
  PowerLevel stop_power_level;
  RfPower rf_power;
  uint64_t sweep_delay_ms;
} SignalGeneratorConfig;

typedef struct SignalGeneratorConfigAmpSweep {
  uint64_t cw_hz;
  uint16_t sweep_power_steps;
  Attenuation start_attenuation;
  PowerLevel start_power_level;
  Attenuation stop_attenuation;
  PowerLevel stop_power_level;
  RfPower rf_power;
  uint64_t sweep_delay_ms;
} SignalGeneratorConfigAmpSweep;

typedef struct SignalGeneratorConfigCw {
  uint64_t cw_hz;
  uint32_t total_steps;
  uint64_t step_freq_hz;
  Attenuation attenuation;
  PowerLevel power_level;
  RfPower rf_power;
} SignalGeneratorConfigCw;

typedef struct SignalGeneratorConfigFreqSweep {
  uint64_t start_hz;
  uint32_t total_steps;
  uint64_t step_hz;
  Attenuation attenuation;
  PowerLevel power_level;
  RfPower rf_power;
  uint64_t sweep_delay_ms;
} SignalGeneratorConfigFreqSweep;

typedef struct SpectrumAnalyzerConfig {
  uint64_t start_freq_hz;
  uint64_t step_size_hz;
  uint64_t stop_freq_hz;
  uint64_t center_freq_hz;
  uint64_t span_hz;
  int16_t max_amp_dbm;
  int16_t min_amp_dbm;
  uint16_t sweep_len;
  bool is_expansion_radio_module_active;
  Mode mode;
  uint64_t min_freq_hz;
  uint64_t max_freq_hz;
  uint64_t max_span_hz;
  uint64_t rbw_hz;
  int8_t amp_offset_db;
  CalcMode calc_mode;
} SpectrumAnalyzerConfig;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#if (defined(_WIN32) || defined(__APPLE__) || defined(__linux__))
bool rfe_is_driver_installed(void);
#endif

char *const *rfe_port_names(uintptr_t *len);

void rfe_free_port_names(char **port_names_ptr, uintptr_t len);

enum Result rfe_screen_data_get_pixel(const struct ScreenData *screen_data,
                                      uint8_t x,
                                      uint8_t y,
                                      bool *pixel);

enum Result rfe_screen_data_get_pixel_checked(const struct ScreenData *screen_data,
                                              uint8_t x,
                                              uint8_t y,
                                              bool *pixel);

enum Result rfe_screen_data_timestamp(const struct ScreenData *screen_data, int64_t *timestamp);

void rfe_screen_data_free(struct ScreenData *screen_data);

enum Result rfe_signal_generator_model_name(SignalGeneratorModel model,
                                            char *name_buf,
                                            uintptr_t len);

uint64_t rfe_signal_generator_model_min_freq_hz(SignalGeneratorModel model);

uint64_t rfe_signal_generator_model_max_freq_hz(SignalGeneratorModel model);

struct SignalGenerator *rfe_signal_generator_connect(void);

struct SignalGenerator *rfe_signal_generator_connect_with_name_and_baud_rate(const char *name,
                                                                             uint32_t baud_rate);

void rfe_signal_generator_free(struct SignalGenerator *rfe);

enum Result rfe_signal_generator_send_bytes(const struct SignalGenerator *rfe,
                                            const uint8_t *bytes,
                                            uintptr_t len);

enum Result rfe_signal_generator_port_name(const struct SignalGenerator *rfe,
                                           char *port_name_buf,
                                           uintptr_t buf_len);

enum Result rfe_signal_generator_firmware_version(const struct SignalGenerator *rfe,
                                                  char *firmware_version_buf,
                                                  uintptr_t buf_len);

uintptr_t rfe_signal_generator_firmware_version_len(const struct SignalGenerator *rfe);

enum Result rfe_signal_generator_serial_number(const struct SignalGenerator *rfe,
                                               char *serial_number_buf,
                                               uintptr_t buf_len);

uintptr_t rfe_signal_generator_serial_number_len(const struct SignalGenerator *rfe);

enum Result rfe_signal_generator_lcd_on(const struct SignalGenerator *rfe);

enum Result rfe_signal_generator_lcd_off(const struct SignalGenerator *rfe);

enum Result rfe_signal_generator_enable_dump_screen(const struct SignalGenerator *rfe);

enum Result rfe_signal_generator_disable_dump_screen(const struct SignalGenerator *rfe);

enum Result rfe_signal_generator_hold(const struct SignalGenerator *rfe);

enum Result rfe_signal_generator_reboot(struct SignalGenerator *rfe);

enum Result rfe_signal_generator_power_off(struct SignalGenerator *rfe);

enum Result rfe_signal_generator_config(const struct SignalGenerator *rfe,
                                        struct SignalGeneratorConfig *config);

enum Result rfe_signal_generator_config_amp_sweep(const struct SignalGenerator *rfe,
                                                  struct SignalGeneratorConfigAmpSweep *config);

enum Result rfe_signal_generator_config_cw(const struct SignalGenerator *rfe,
                                           struct SignalGeneratorConfigCw *config);

enum Result rfe_signal_generator_config_freq_sweep(const struct SignalGenerator *rfe,
                                                   struct SignalGeneratorConfigFreqSweep *config);

enum Result rfe_signal_generator_screen_data(const struct SignalGenerator *rfe,
                                             const struct ScreenData **screen_data);

enum Result rfe_signal_generator_wait_for_next_screen_data(const struct SignalGenerator *rfe,
                                                           const struct ScreenData **screen_data);

enum Result rfe_signal_generator_wait_for_next_screen_data_with_timeout(const struct SignalGenerator *rfe,
                                                                        uint64_t timeout_secs,
                                                                        const struct ScreenData **screen_data);

enum Result rfe_signal_generator_temperature(const struct SignalGenerator *rfe,
                                             Temperature *temperature);

enum Result rfe_signal_generator_main_radio_model(const struct SignalGenerator *rfe,
                                                  SignalGeneratorModel *model);

enum Result rfe_signal_generator_expansion_radio_model(const struct SignalGenerator *rfe,
                                                       SignalGeneratorModel *model);

enum Result rfe_signal_generator_active_radio_model(const struct SignalGenerator *rfe,
                                                    SignalGeneratorModel *model);

enum Result rfe_signal_generator_inactive_radio_model(const struct SignalGenerator *rfe,
                                                      SignalGeneratorModel *model);

enum Result rfe_signal_generator_start_amp_sweep(const struct SignalGenerator *rfe,
                                                 uint64_t cw_hz,
                                                 Attenuation start_attenuation,
                                                 PowerLevel start_power_level,
                                                 Attenuation stop_attenuation,
                                                 PowerLevel stop_power_level,
                                                 uint8_t step_delay_sec);

enum Result rfe_signal_generator_start_amp_sweep_exp(const struct SignalGenerator *rfe,
                                                     uint64_t cw_hz,
                                                     double start_power_dbm,
                                                     double step_power_db,
                                                     double stop_power_dbm,
                                                     uint8_t step_delay_sec);

enum Result rfe_signal_generator_start_cw(const struct SignalGenerator *rfe,
                                          uint64_t cw_hz,
                                          Attenuation attenuation,
                                          PowerLevel power_level);

enum Result rfe_signal_generator_start_cw_exp(const struct SignalGenerator *rfe,
                                              uint64_t cw_hz,
                                              double power_dbm);

enum Result rfe_signal_generator_start_freq_sweep(const struct SignalGenerator *rfe,
                                                  uint64_t start_hz,
                                                  Attenuation attenuation,
                                                  PowerLevel power_level,
                                                  uint16_t sweep_steps,
                                                  uint64_t step_hz,
                                                  uint8_t step_delay_sec);

enum Result rfe_signal_generator_start_freq_sweep_exp(const struct SignalGenerator *rfe,
                                                      uint64_t start_hz,
                                                      double power_dbm,
                                                      uint16_t sweep_steps,
                                                      uint64_t step_hz,
                                                      uint8_t step_delay_sec);

enum Result rfe_signal_generator_start_tracking(const struct SignalGenerator *rfe,
                                                uint64_t start_hz,
                                                Attenuation attenuation,
                                                PowerLevel power_level,
                                                uint16_t sweep_steps,
                                                uint64_t step_hz);

enum Result rfe_signal_generator_start_tracking_exp(const struct SignalGenerator *rfe,
                                                    uint64_t start_hz,
                                                    double power_dbm,
                                                    uint16_t sweep_steps,
                                                    uint64_t step_hz);

enum Result rfe_signal_generator_tracking_step(const struct SignalGenerator *rfe, uint16_t steps);

void rfe_signal_generator_set_config_callback(const struct SignalGenerator *rfe,
                                              void (*callback)(struct SignalGeneratorConfig config,
                                                               void *user_data),
                                              void *user_data);

void rfe_signal_generator_remove_config_callback(const struct SignalGenerator *rfe);

void rfe_signal_generator_set_config_amp_sweep_callback(const struct SignalGenerator *rfe,
                                                        void (*callback)(struct SignalGeneratorConfigAmpSweep config,
                                                                         void *user_data),
                                                        void *user_data);

void rfe_signal_generator_remove_config_amp_sweep_callback(const struct SignalGenerator *rfe);

void rfe_signal_generator_set_config_cw_callback(const struct SignalGenerator *rfe,
                                                 void (*callback)(struct SignalGeneratorConfigCw config,
                                                                  void *user_data),
                                                 void *user_data);

void rfe_signal_generator_remove_config_cw_callback(const struct SignalGenerator *rfe);

void rfe_signal_generator_set_config_freq_sweep_callback(const struct SignalGenerator *rfe,
                                                         void (*callback)(struct SignalGeneratorConfigFreqSweep config,
                                                                          void *user_data),
                                                         void *user_data);

void rfe_signal_generator_remove_config_freq_sweep_callback(const struct SignalGenerator *rfe);

enum Result rfe_signal_generator_rf_power_on(const struct SignalGenerator *rfe);

enum Result rfe_signal_generator_rf_power_off(const struct SignalGenerator *rfe);

enum Result rfe_spectrum_analyzer_model_name(SpectrumAnalyzerModel model,
                                             char *name_buf,
                                             uintptr_t len);

bool rfe_spectrum_analyzer_model_is_plus_model(SpectrumAnalyzerModel model);

bool rfe_spectrum_analyzer_model_has_wifi_analyzer(SpectrumAnalyzerModel model);

uint64_t rfe_spectrum_analyzer_model_min_freq_hz(SpectrumAnalyzerModel model);

uint64_t rfe_spectrum_analyzer_model_max_freq_hz(SpectrumAnalyzerModel model);

uint64_t rfe_spectrum_analyzer_model_min_span_hz(SpectrumAnalyzerModel model);

uint64_t rfe_spectrum_analyzer_model_max_span_hz(SpectrumAnalyzerModel model);

struct SpectrumAnalyzer *rfe_spectrum_analyzer_connect(void);

struct SpectrumAnalyzer *rfe_spectrum_analyzer_connect_with_name_and_baud_rate(const char *name,
                                                                               uint32_t baud_rate);

void rfe_spectrum_analyzer_free(struct SpectrumAnalyzer *rfe);

enum Result rfe_spectrum_analyzer_send_bytes(const struct SpectrumAnalyzer *rfe,
                                             const uint8_t *bytes,
                                             uintptr_t len);

enum Result rfe_spectrum_analyzer_port_name(const struct SpectrumAnalyzer *rfe,
                                            char *port_name_buf,
                                            uintptr_t buf_len);

uintptr_t rfe_spectrum_analyzer_port_name_len(const struct SpectrumAnalyzer *rfe);

enum Result rfe_spectrum_analyzer_firmware_version(const struct SpectrumAnalyzer *rfe,
                                                   char *firmware_version_buf,
                                                   uintptr_t buf_len);

uintptr_t rfe_spectrum_analyzer_firmware_version_len(const struct SpectrumAnalyzer *rfe);

enum Result rfe_spectrum_analyzer_serial_number(const struct SpectrumAnalyzer *rfe,
                                                char *serial_number_buf,
                                                uintptr_t buf_len);

uintptr_t rfe_spectrum_analyzer_serial_number_len(const struct SpectrumAnalyzer *rfe);

enum Result rfe_spectrum_analyzer_lcd_on(const struct SpectrumAnalyzer *rfe);

enum Result rfe_spectrum_analyzer_lcd_off(const struct SpectrumAnalyzer *rfe);

enum Result rfe_spectrum_analyzer_enable_dump_screen(const struct SpectrumAnalyzer *rfe);

enum Result rfe_spectrum_analyzer_disable_dump_screen(const struct SpectrumAnalyzer *rfe);

enum Result rfe_spectrum_analyzer_hold(const struct SpectrumAnalyzer *rfe);

enum Result rfe_spectrum_analyzer_reboot(struct SpectrumAnalyzer *rfe);

enum Result rfe_spectrum_analyzer_power_off(struct SpectrumAnalyzer *rfe);

uint64_t rfe_spectrum_analyzer_start_freq_hz(const struct SpectrumAnalyzer *rfe);

uint64_t rfe_spectrum_analyzer_step_size_hz(const struct SpectrumAnalyzer *rfe);

uint64_t rfe_spectrum_analyzer_stop_freq_hz(const struct SpectrumAnalyzer *rfe);

uint64_t rfe_spectrum_analyzer_center_freq_hz(const struct SpectrumAnalyzer *rfe);

uint64_t rfe_spectrum_analyzer_span_hz(const struct SpectrumAnalyzer *rfe);

uint64_t rfe_spectrum_analyzer_min_freq_hz(const struct SpectrumAnalyzer *rfe);

uint64_t rfe_spectrum_analyzer_max_freq_hz(const struct SpectrumAnalyzer *rfe);

uint64_t rfe_spectrum_analyzer_max_span_hz(const struct SpectrumAnalyzer *rfe);

uint64_t rfe_spectrum_analyzer_rbw_hz(const struct SpectrumAnalyzer *rfe);

int16_t rfe_spectrum_analyzer_min_amp_dbm(const struct SpectrumAnalyzer *rfe);

int16_t rfe_spectrum_analyzer_max_amp_dbm(const struct SpectrumAnalyzer *rfe);

int8_t rfe_spectrum_analyzer_amp_offset_db(const struct SpectrumAnalyzer *rfe);

uint16_t rfe_spectrum_analyzer_sweep_len(const struct SpectrumAnalyzer *rfe);

Mode rfe_spectrum_analyzer_mode(const struct SpectrumAnalyzer *rfe);

CalcMode rfe_spectrum_analyzer_calc_mode(const struct SpectrumAnalyzer *rfe);

enum Result rfe_spectrum_analyzer_sweep(const struct SpectrumAnalyzer *rfe,
                                        float *sweep_buf,
                                        uintptr_t buf_len,
                                        uintptr_t *sweep_len);

enum Result rfe_spectrum_analyzer_wait_for_next_sweep(const struct SpectrumAnalyzer *rfe,
                                                      float *sweep_buf,
                                                      uintptr_t buf_len,
                                                      uintptr_t *sweep_len);

enum Result rfe_spectrum_analyzer_wait_for_next_sweep_with_timeout(const struct SpectrumAnalyzer *rfe,
                                                                   uint64_t timeout_secs,
                                                                   float *sweep_buf,
                                                                   uintptr_t buf_len,
                                                                   uintptr_t *sweep_len);

enum Result rfe_spectrum_analyzer_screen_data(const struct SpectrumAnalyzer *rfe,
                                              const struct ScreenData **screen_data);

enum Result rfe_spectrum_analyzer_wait_for_next_screen_data(const struct SpectrumAnalyzer *rfe,
                                                            const struct ScreenData **screen_data);

enum Result rfe_spectrum_analyzer_wait_for_next_screen_data_with_timeout(const struct SpectrumAnalyzer *rfe,
                                                                         uint64_t timeout_secs,
                                                                         const struct ScreenData **screen_data);

enum Result rfe_spectrum_analyzer_dsp_mode(const struct SpectrumAnalyzer *rfe, DspMode *dsp_mode);

enum Result rfe_spectrum_analyzer_tracking_status(const struct SpectrumAnalyzer *rfe,
                                                  TrackingStatus *tracking_status);

enum Result rfe_spectrum_analyzer_input_stage(const struct SpectrumAnalyzer *rfe,
                                              InputStage *input_stage);

SpectrumAnalyzerModel rfe_spectrum_analyzer_main_radio_model(const struct SpectrumAnalyzer *rfe);

SpectrumAnalyzerModel rfe_spectrum_analyzer_expansion_radio_model(const struct SpectrumAnalyzer *rfe);

SpectrumAnalyzerModel rfe_spectrum_analyzer_active_radio_model(const struct SpectrumAnalyzer *rfe);

SpectrumAnalyzerModel rfe_spectrum_analyzer_inactive_radio_model(const struct SpectrumAnalyzer *rfe);

enum Result rfe_spectrum_analyzer_start_wifi_analyzer(const struct SpectrumAnalyzer *rfe,
                                                      WifiBand wifi_band);

enum Result rfe_spectrum_analyzer_stop_wifi_analyzer(const struct SpectrumAnalyzer *rfe);

enum Result rfe_spectrum_analyzer_request_tracking(const struct SpectrumAnalyzer *rfe,
                                                   uint64_t start_hz,
                                                   uint64_t step_hz);

enum Result rfe_spectrum_analyzer_tracking_step(const struct SpectrumAnalyzer *rfe, uint16_t step);

enum Result rfe_spectrum_analyzer_set_start_stop(const struct SpectrumAnalyzer *rfe,
                                                 uint64_t start_hz,
                                                 uint64_t stop_hz);

enum Result rfe_spectrum_analyzer_set_start_stop_sweep_len(const struct SpectrumAnalyzer *rfe,
                                                           uint64_t start_hz,
                                                           uint64_t stop_hz,
                                                           uint16_t sweep_len);

enum Result rfe_spectrum_analyzer_set_center_span(const struct SpectrumAnalyzer *rfe,
                                                  uint64_t center_hz,
                                                  uint64_t span_hz);

enum Result rfe_spectrum_analyzer_set_center_span_sweep_len(const struct SpectrumAnalyzer *rfe,
                                                            uint64_t center_hz,
                                                            uint64_t span_hz,
                                                            uint16_t sweep_len);

enum Result rfe_spectrum_analyzer_set_min_max_amps(const struct SpectrumAnalyzer *rfe,
                                                   int16_t min_amp_dbm,
                                                   int16_t max_amp_dbm);

void rfe_spectrum_analyzer_set_sweep_callback(const struct SpectrumAnalyzer *rfe,
                                              void (*callback)(const float *sweep,
                                                               uintptr_t sweep_len,
                                                               uint64_t start_hz,
                                                               uint64_t stop_hz,
                                                               void *user_data),
                                              void *user_data);

void rfe_spectrum_analyzer_remove_sweep_callback(const struct SpectrumAnalyzer *rfe);

void rfe_spectrum_analyzer_set_config_callback(const struct SpectrumAnalyzer *rfe,
                                               void (*callback)(struct SpectrumAnalyzerConfig config,
                                                                void *user_data),
                                               void *user_data);

void rfe_spectrum_analyzer_remove_config_callback(const struct SpectrumAnalyzer *rfe);

enum Result rfe_spectrum_analyzer_set_sweep_len(const struct SpectrumAnalyzer *rfe,
                                                uint16_t sweep_len);

enum Result rfe_spectrum_analyzer_set_calc_mode(const struct SpectrumAnalyzer *rfe,
                                                CalcMode calc_mode);

enum Result rfe_spectrum_analyzer_activate_main_radio(const struct SpectrumAnalyzer *rfe);

enum Result rfe_spectrum_analyzer_activate_expansion_radio(const struct SpectrumAnalyzer *rfe);

enum Result rfe_spectrum_analyzer_set_input_stage(const struct SpectrumAnalyzer *rfe,
                                                  InputStage input_stage);

enum Result rfe_spectrum_analyzer_set_offset_db(const struct SpectrumAnalyzer *rfe,
                                                int8_t offset_db);

enum Result rfe_spectrum_analyzer_set_dsp_mode(const struct SpectrumAnalyzer *rfe,
                                               DspMode dsp_mode);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* rfe_h */
