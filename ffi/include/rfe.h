#ifndef rfe_h
#define rfe_h

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Screen width in pixels.
 */
#define ScreenData_WIDTH_PX 128

/**
 * Screen height in pixels.
 */
#define ScreenData_HEIGHT_PX 64

/**
 * Result code returned by fallible `rfe-ffi` functions.
 */
typedef enum Result {
  /**
   * The function completed successfully.
   */
  RESULT_SUCCESS = 0,
  /**
   * The connected device reported unsupported or incompatible firmware.
   */
  RESULT_INCOMPATIBLE_FIRMWARE_ERROR,
  /**
   * An argument was invalid, such as an out-of-range value or undersized buffer.
   */
  RESULT_INVALID_INPUT_ERROR,
  /**
   * The requested operation is not valid for the current device state.
   */
  RESULT_INVALID_OPERATION_ERROR,
  /**
   * A serial port or operating system I/O error occurred.
   */
  RESULT_IO_ERROR,
  /**
   * The requested data has not been received from the device.
   */
  RESULT_NO_DATA,
  /**
   * A required pointer argument was `NULL`.
   */
  RESULT_NULL_PTR_ERROR,
  /**
   * The device did not respond before the operation timed out.
   */
  RESULT_TIMEOUT_ERROR,
} Result;

/**
 * Signal generator model reported by the RF Explorer.
 */
enum SignalGeneratorModel
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  /**
   * Main 6 GHz signal generator module.
   */
  SIGNAL_GENERATOR_MODEL_RFE6_GEN = 60,
  /**
   * Expansion 6 GHz signal generator module.
   */
  SIGNAL_GENERATOR_MODEL_RFE6_GEN_EXPANSION = 61,
};
#ifndef __cplusplus
typedef uint8_t SignalGeneratorModel;
#endif // __cplusplus

/**
 * RF output attenuation state.
 */
enum Attenuation
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  /**
   * Attenuation is enabled.
   */
  ATTENUATION_ON = 0,
  /**
   * Attenuation is disabled.
   */
  ATTENUATION_OFF,
};
#ifndef __cplusplus
typedef uint8_t Attenuation;
#endif // __cplusplus

/**
 * Discrete RF output power level.
 */
enum PowerLevel
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  /**
   * Lowest output power.
   */
  POWER_LEVEL_LOWEST = 0,
  /**
   * Low output power.
   */
  POWER_LEVEL_LOW,
  /**
   * High output power.
   */
  POWER_LEVEL_HIGH,
  /**
   * Highest output power.
   */
  POWER_LEVEL_HIGHEST,
};
#ifndef __cplusplus
typedef uint8_t PowerLevel;
#endif // __cplusplus

/**
 * RF output power state.
 */
enum RfPower
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  /**
   * RF output is enabled.
   */
  RF_POWER_ON = 0,
  /**
   * RF output is disabled.
   */
  RF_POWER_OFF,
};
#ifndef __cplusplus
typedef uint8_t RfPower;
#endif // __cplusplus

/**
 * Temperature range reported by the signal generator.
 */
enum Temperature
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  /**
   * Temperature is between -10 C and 0 C.
   */
  TEMPERATURE_MINUS_TEN_TO_ZERO = 48,
  /**
   * Temperature is between 0 C and 10 C.
   */
  TEMPERATURE_ZERO_TO_TEN = 49,
  /**
   * Temperature is between 10 C and 20 C.
   */
  TEMPERATURE_TEN_TO_TWENTY = 50,
  /**
   * Temperature is between 20 C and 30 C.
   */
  TEMPERATURE_TWENTY_TO_THIRTY = 51,
  /**
   * Temperature is between 30 C and 40 C.
   */
  TEMPERATURE_THIRTY_TO_FORTY = 52,
  /**
   * Temperature is between 40 C and 50 C.
   */
  TEMPERATURE_FORTY_TO_FIFTY = 53,
  /**
   * Temperature is between 50 C and 60 C.
   */
  TEMPERATURE_FIFTY_TO_SIXTY = 54,
};
#ifndef __cplusplus
typedef uint8_t Temperature;
#endif // __cplusplus

/**
 * RF Explorer spectrum analyzer model.
 */
enum SpectrumAnalyzerModel
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  /**
   * 433M model.
   */
  SPECTRUM_ANALYZER_MODEL_RFE433_M = 0,
  /**
   * 868M model.
   */
  SPECTRUM_ANALYZER_MODEL_RFE868_M = 1,
  /**
   * 915M model.
   */
  SPECTRUM_ANALYZER_MODEL_RFE915_M = 2,
  /**
   * WSUB1G model.
   */
  SPECTRUM_ANALYZER_MODEL_RFE_W_SUB1_G = 3,
  /**
   * 2.4G model.
   */
  SPECTRUM_ANALYZER_MODEL_RFE24_G = 4,
  /**
   * WSUB3G model.
   */
  SPECTRUM_ANALYZER_MODEL_RFE_W_SUB3_G = 5,
  /**
   * 6G model.
   */
  SPECTRUM_ANALYZER_MODEL_RFE6_G = 6,
  /**
   * WSUB1G+ model.
   */
  SPECTRUM_ANALYZER_MODEL_RFE_W_SUB1_G_PLUS = 10,
  /**
   * Pro Audio model.
   */
  SPECTRUM_ANALYZER_MODEL_RFE_PRO_AUDIO = 11,
  /**
   * 2.4G+ model.
   */
  SPECTRUM_ANALYZER_MODEL_RFE24_G_PLUS = 12,
  /**
   * 4G+ model.
   */
  SPECTRUM_ANALYZER_MODEL_RFE4_G_PLUS = 13,
  /**
   * 6G+ model.
   */
  SPECTRUM_ANALYZER_MODEL_RFE6_G_PLUS = 14,
  /**
   * MW5G 3 GHz model.
   */
  SPECTRUM_ANALYZER_MODEL_RFE_MW5G3G = 16,
  /**
   * MW5G 4 GHz model.
   */
  SPECTRUM_ANALYZER_MODEL_RFE_MW5G4G = 17,
  /**
   * MW5G 5 GHz model.
   */
  SPECTRUM_ANALYZER_MODEL_RFE_MW5G5G = 18,
  /**
   * Unknown or unsupported model.
   */
  SPECTRUM_ANALYZER_MODEL_UNKNOWN = 19,
};
#ifndef __cplusplus
typedef uint8_t SpectrumAnalyzerModel;
#endif // __cplusplus

/**
 * Operating mode reported by an RF Explorer device.
 */
enum Mode
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  /**
   * Spectrum analyzer mode.
   */
  MODE_SPECTRUM_ANALYZER = 0,
  /**
   * RF generator mode.
   */
  MODE_RF_GENERATOR = 1,
  /**
   * Wi-Fi analyzer mode.
   */
  MODE_WIFI_ANALYZER = 2,
  /**
   * Analyzer tracking mode.
   */
  MODE_ANALYZER_TRACKING = 5,
  /**
   * RF sniffer mode.
   */
  MODE_RF_SNIFFER = 6,
  /**
   * CW transmitter mode.
   */
  MODE_CW_TRANSMITTER = 60,
  /**
   * Frequency sweep mode.
   */
  MODE_SWEEP_FREQUENCY = 61,
  /**
   * Amplitude sweep mode.
   */
  MODE_SWEEP_AMPLITUDE = 62,
  /**
   * Generator tracking mode.
   */
  MODE_GENERATOR_TRACKING = 63,
  /**
   * Unknown or unsupported mode.
   */
  MODE_UNKNOWN = 255,
};
#ifndef __cplusplus
typedef uint8_t Mode;
#endif // __cplusplus

/**
 * Sweep calculator mode used by the spectrum analyzer.
 */
enum CalcMode
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  /**
   * Normal sweep display.
   */
  CALC_MODE_NORMAL = 0,
  /**
   * Maximum value mode.
   */
  CALC_MODE_MAX,
  /**
   * Average value mode.
   */
  CALC_MODE_AVG,
  /**
   * Overwrite mode.
   */
  CALC_MODE_OVERWRITE,
  /**
   * Maximum hold mode.
   */
  CALC_MODE_MAX_HOLD,
  /**
   * Historical maximum mode.
   */
  CALC_MODE_MAX_HISTORICAL,
  /**
   * Unknown or unsupported calculator mode.
   */
  CALC_MODE_UNKNOWN = 255,
};
#ifndef __cplusplus
typedef uint8_t CalcMode;
#endif // __cplusplus

/**
 * Digital signal processing mode used by the spectrum analyzer.
 */
enum DspMode
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  /**
   * Automatically select the DSP mode.
   */
  DSP_MODE_AUTO = 0,
  /**
   * Filtered DSP mode.
   */
  DSP_MODE_FILTER,
  /**
   * Fast DSP mode.
   */
  DSP_MODE_FAST,
  /**
   * No image rejection DSP mode.
   */
  DSP_MODE_NO_IMG,
};
#ifndef __cplusplus
typedef uint8_t DspMode;
#endif // __cplusplus

/**
 * Status of analyzer tracking mode.
 */
enum TrackingStatus
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  /**
   * Tracking mode is disabled.
   */
  TRACKING_STATUS_DISABLED = 0,
  /**
   * Tracking mode is enabled.
   */
  TRACKING_STATUS_ENABLED,
};
#ifndef __cplusplus
typedef uint8_t TrackingStatus;
#endif // __cplusplus

/**
 * RF input stage selected on supported spectrum analyzer models.
 */
enum InputStage
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  /**
   * Direct input path.
   */
  INPUT_STAGE_DIRECT = 48,
  /**
   * 30 dB attenuator input path.
   */
  INPUT_STAGE_ATTENUATOR30D_B = 49,
  /**
   * 25 dB low-noise amplifier input path.
   */
  INPUT_STAGE_LNA25D_B = 50,
  /**
   * 60 dB attenuator input path.
   */
  INPUT_STAGE_ATTENUATOR60D_B = 51,
  /**
   * 12 dB low-noise amplifier input path.
   */
  INPUT_STAGE_LNA12D_B = 52,
};
#ifndef __cplusplus
typedef uint8_t InputStage;
#endif // __cplusplus

/**
 * Wi-Fi band used by Wi-Fi analyzer mode.
 */
enum WifiBand
#ifdef __cplusplus
  : uint8_t
#endif // __cplusplus
 {
  /**
   * 2.4 GHz Wi-Fi band.
   */
  WIFI_BAND_TWO_POINT_FOUR_GHZ = 1,
  /**
   * 5 GHz Wi-Fi band.
   */
  WIFI_BAND_FIVE_GHZ,
};
#ifndef __cplusplus
typedef uint8_t WifiBand;
#endif // __cplusplus

/**
 * Monochrome LCD screen capture from an RF Explorer device.
 */
typedef struct ScreenData ScreenData;

/**
 * RF Explorer signal generator device.
 */
typedef struct SignalGenerator SignalGenerator;

/**
 * RF Explorer spectrum analyzer device.
 */
typedef struct SpectrumAnalyzer SpectrumAnalyzer;

/**
 * Signal generator configuration.
 *
 * Frequencies are represented in hertz. Durations are represented in
 * milliseconds.
 */
typedef struct SignalGeneratorConfig {
  /**
   * Start frequency for frequency sweep and tracking modes.
   */
  uint64_t start_hz;
  /**
   * CW frequency.
   */
  uint64_t cw_hz;
  /**
   * Total number of sweep or tracking steps.
   */
  uint32_t total_steps;
  /**
   * Frequency increment per step.
   */
  uint64_t step_hz;
  /**
   * CW and frequency sweep attenuation setting.
   */
  Attenuation attenuation;
  /**
   * CW and frequency sweep power level.
   */
  PowerLevel power_level;
  /**
   * Number of amplitude sweep power steps.
   */
  uint16_t sweep_power_steps;
  /**
   * Amplitude sweep start attenuation setting.
   */
  Attenuation start_attenuation;
  /**
   * Amplitude sweep start power level.
   */
  PowerLevel start_power_level;
  /**
   * Amplitude sweep stop attenuation setting.
   */
  Attenuation stop_attenuation;
  /**
   * Amplitude sweep stop power level.
   */
  PowerLevel stop_power_level;
  /**
   * RF output power state.
   */
  RfPower rf_power;
  /**
   * Delay between sweep steps.
   */
  uint64_t sweep_delay_ms;
} SignalGeneratorConfig;

/**
 * Signal generator amplitude sweep configuration.
 *
 * Frequencies are represented in hertz. Durations are represented in
 * milliseconds.
 */
typedef struct SignalGeneratorConfigAmpSweep {
  /**
   * CW frequency used during the amplitude sweep.
   */
  uint64_t cw_hz;
  /**
   * Number of power steps in the sweep.
   */
  uint16_t sweep_power_steps;
  /**
   * Starting attenuation setting.
   */
  Attenuation start_attenuation;
  /**
   * Starting output power level.
   */
  PowerLevel start_power_level;
  /**
   * Stopping attenuation setting.
   */
  Attenuation stop_attenuation;
  /**
   * Stopping output power level.
   */
  PowerLevel stop_power_level;
  /**
   * RF output power state.
   */
  RfPower rf_power;
  /**
   * Delay between amplitude sweep steps.
   */
  uint64_t sweep_delay_ms;
} SignalGeneratorConfigAmpSweep;

/**
 * Signal generator CW configuration.
 *
 * Frequencies are represented in hertz.
 */
typedef struct SignalGeneratorConfigCw {
  /**
   * CW frequency.
   */
  uint64_t cw_hz;
  /**
   * Total number of configured steps.
   */
  uint32_t total_steps;
  /**
   * Frequency increment per step.
   */
  uint64_t step_freq_hz;
  /**
   * RF output attenuation setting.
   */
  Attenuation attenuation;
  /**
   * RF output power level.
   */
  PowerLevel power_level;
  /**
   * RF output power state.
   */
  RfPower rf_power;
} SignalGeneratorConfigCw;

/**
 * Signal generator frequency sweep configuration.
 *
 * Frequencies are represented in hertz. Durations are represented in
 * milliseconds.
 */
typedef struct SignalGeneratorConfigFreqSweep {
  /**
   * Start frequency.
   */
  uint64_t start_hz;
  /**
   * Total number of sweep steps.
   */
  uint32_t total_steps;
  /**
   * Frequency increment per step.
   */
  uint64_t step_hz;
  /**
   * RF output attenuation setting.
   */
  Attenuation attenuation;
  /**
   * RF output power level.
   */
  PowerLevel power_level;
  /**
   * RF output power state.
   */
  RfPower rf_power;
  /**
   * Delay between sweep steps.
   */
  uint64_t sweep_delay_ms;
} SignalGeneratorConfigFreqSweep;

/**
 * Spectrum analyzer configuration.
 *
 * Frequencies are represented in hertz. Fields that are optional in the Rust
 * API use zero or the enum default when the device has not reported a value.
 */
typedef struct SpectrumAnalyzerConfig {
  /**
   * Sweep start frequency.
   */
  uint64_t start_freq_hz;
  /**
   * Frequency step between sweep points.
   */
  uint64_t step_size_hz;
  /**
   * Sweep stop frequency.
   */
  uint64_t stop_freq_hz;
  /**
   * Sweep center frequency.
   */
  uint64_t center_freq_hz;
  /**
   * Sweep span.
   */
  uint64_t span_hz;
  /**
   * Top displayed amplitude in dBm.
   */
  int16_t max_amp_dbm;
  /**
   * Bottom displayed amplitude in dBm.
   */
  int16_t min_amp_dbm;
  /**
   * Number of points in each sweep.
   */
  uint16_t sweep_len;
  /**
   * Whether the expansion radio module is active.
   */
  bool is_expansion_radio_module_active;
  /**
   * Current operating mode.
   */
  Mode mode;
  /**
   * Minimum supported frequency.
   */
  uint64_t min_freq_hz;
  /**
   * Maximum supported frequency.
   */
  uint64_t max_freq_hz;
  /**
   * Maximum supported span.
   */
  uint64_t max_span_hz;
  /**
   * Resolution bandwidth, or zero if it has not been reported by the device.
   */
  uint64_t rbw_hz;
  /**
   * Amplitude offset in dB, or zero if it has not been reported by the device.
   */
  int8_t amp_offset_db;
  /**
   * Calculator mode, or the default value if it has not been reported by the device.
   */
  CalcMode calc_mode;
} SpectrumAnalyzerConfig;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#if (defined(_WIN32) || defined(__APPLE__) || defined(__linux__))
/**
 * Returns whether the platform RF Explorer USB serial driver appears to be installed.
 */
bool rfe_is_driver_installed(void);
#endif

/**
 * Returns a heap-allocated array of RF Explorer serial port names.
 *
 * If `len` is non-NULL, it is set to the number of returned names. The returned
 * array and each string in it are owned by the caller and must be released with
 * `rfe_free_port_names`.
 */
char **rfe_port_names(uintptr_t *len);

/**
 * Frees an array returned by `rfe_port_names`.
 *
 * `len` must be the same length returned by `rfe_port_names`. Passing `NULL`
 * is allowed and has no effect.
 */
void rfe_free_port_names(char **port_names_ptr, uintptr_t len);

/**
 * Gets one pixel from an RF Explorer LCD screen capture.
 *
 * The top-left pixel is `(0, 0)` and the bottom-right pixel is `(127, 63)`.
 * On success, `pixel` is set to `true` for an enabled pixel and `false` for a
 * disabled pixel. Returns `RESULT_INVALID_INPUT_ERROR` if the coordinates are
 * out of range.
 */
enum Result rfe_screen_data_get_pixel(const struct ScreenData *screen_data,
                                      uint8_t x,
                                      uint8_t y,
                                      bool *pixel);

/**
 * Gets one pixel from an RF Explorer LCD screen capture with bounds checking.
 *
 * This is equivalent to `rfe_screen_data_get_pixel`; both functions return
 * `RESULT_INVALID_INPUT_ERROR` for out-of-range coordinates.
 */
enum Result rfe_screen_data_get_pixel_checked(const struct ScreenData *screen_data,
                                              uint8_t x,
                                              uint8_t y,
                                              bool *pixel);

/**
 * Writes the screen capture timestamp as Unix seconds.
 */
enum Result rfe_screen_data_timestamp(const struct ScreenData *screen_data, int64_t *timestamp);

/**
 * Frees screen data returned by an `rfe_*_screen_data` function.
 *
 * Passing `NULL` is allowed and has no effect.
 */
void rfe_screen_data_free(struct ScreenData *screen_data);

/**
 * Writes the display name of a signal generator model.
 *
 * `name_buf` must point to a writable buffer of at least `len` bytes. The
 * buffer receives a null-terminated C string. Returns
 * `RESULT_INVALID_INPUT_ERROR` if `len` is too small.
 */
enum Result rfe_signal_generator_model_name(SignalGeneratorModel model,
                                            char *name_buf,
                                            uintptr_t len);

/**
 * Returns the model's minimum supported output frequency in hertz.
 */
uint64_t rfe_signal_generator_model_min_freq_hz(SignalGeneratorModel model);

/**
 * Returns the model's maximum supported output frequency in hertz.
 */
uint64_t rfe_signal_generator_model_max_freq_hz(SignalGeneratorModel model);

/**
 * Connects to the first RF Explorer signal generator found on a CP210x USB serial port.
 *
 * Returns `NULL` if no compatible device can be opened and initialized. The
 * returned pointer is owned by the caller and must be freed with
 * `rfe_signal_generator_free`.
 */
struct SignalGenerator *rfe_signal_generator_connect(void);

/**
 * Connects to a named serial port using the given baud rate.
 *
 * `name` must be a valid null-terminated UTF-8 string. Returns `NULL` if the
 * pointer is null, the string is invalid, or the device cannot be opened and
 * initialized. The returned pointer is owned by the caller and must be freed
 * with `rfe_signal_generator_free`.
 */
struct SignalGenerator *rfe_signal_generator_connect_with_name_and_baud_rate(const char *name,
                                                                             uint32_t baud_rate);

/**
 * Frees a signal generator returned by `rfe_signal_generator_connect`.
 *
 * Passing `NULL` is allowed and has no effect.
 */
void rfe_signal_generator_free(struct SignalGenerator *rfe);

/**
 * Sends raw bytes to the signal generator.
 *
 * `bytes` must point to at least `len` bytes. This function is primarily for
 * advanced users that need to send RF Explorer commands not wrapped by this API.
 */
enum Result rfe_signal_generator_send_bytes(const struct SignalGenerator *rfe,
                                            const uint8_t *bytes,
                                            uintptr_t len);

/**
 * Writes the connected serial port name to a caller-provided buffer.
 *
 * Use `rfe_signal_generator_port_name_len` to get the required buffer size,
 * including the terminating null byte.
 */
enum Result rfe_signal_generator_port_name(const struct SignalGenerator *rfe,
                                           char *port_name_buf,
                                           uintptr_t buf_len);

/**
 * Returns the buffer size required for `rfe_signal_generator_port_name`.
 *
 * The returned size includes the terminating null byte. Returns zero if `rfe`
 * is `NULL`.
 */
uintptr_t rfe_signal_generator_port_name_len(const struct SignalGenerator *rfe);

/**
 * Writes the firmware version to a caller-provided buffer.
 *
 * Use `rfe_signal_generator_firmware_version_len` to get the required buffer
 * size, including the terminating null byte.
 */
enum Result rfe_signal_generator_firmware_version(const struct SignalGenerator *rfe,
                                                  char *firmware_version_buf,
                                                  uintptr_t buf_len);

/**
 * Returns the buffer size required for `rfe_signal_generator_firmware_version`.
 *
 * The returned size includes the terminating null byte. Returns zero if `rfe`
 * is `NULL`.
 */
uintptr_t rfe_signal_generator_firmware_version_len(const struct SignalGenerator *rfe);

/**
 * Writes the device serial number to a caller-provided buffer.
 *
 * Use `rfe_signal_generator_serial_number_len` to get the required buffer
 * size, including the terminating null byte. Returns `RESULT_NO_DATA` if the
 * device does not report a serial number.
 */
enum Result rfe_signal_generator_serial_number(const struct SignalGenerator *rfe,
                                               char *serial_number_buf,
                                               uintptr_t buf_len);

/**
 * Returns the buffer size required for `rfe_signal_generator_serial_number`.
 *
 * The returned size includes the terminating null byte. Returns zero if `rfe`
 * is `NULL` or no serial number has been received.
 */
uintptr_t rfe_signal_generator_serial_number_len(const struct SignalGenerator *rfe);

/**
 * Turns the signal generator LCD on.
 */
enum Result rfe_signal_generator_lcd_on(const struct SignalGenerator *rfe);

/**
 * Turns the signal generator LCD off.
 */
enum Result rfe_signal_generator_lcd_off(const struct SignalGenerator *rfe);

/**
 * Enables screen dump messages from the signal generator.
 */
enum Result rfe_signal_generator_enable_dump_screen(const struct SignalGenerator *rfe);

/**
 * Disables screen dump messages from the signal generator.
 */
enum Result rfe_signal_generator_disable_dump_screen(const struct SignalGenerator *rfe);

/**
 * Holds the current signal generator operation.
 */
enum Result rfe_signal_generator_hold(const struct SignalGenerator *rfe);

/**
 * Reboots the signal generator.
 *
 * The `rfe` pointer must not be used after a successful reboot unless the
 * device is reconnected.
 */
enum Result rfe_signal_generator_reboot(struct SignalGenerator *rfe);

/**
 * Powers off the signal generator.
 *
 * The `rfe` pointer must not be used after a successful power-off unless the
 * device is reconnected.
 */
enum Result rfe_signal_generator_power_off(struct SignalGenerator *rfe);

/**
 * Writes the most recent main signal generator configuration to `config`.
 *
 * Returns `RESULT_NO_DATA` if no matching configuration has been received.
 */
enum Result rfe_signal_generator_config(const struct SignalGenerator *rfe,
                                        struct SignalGeneratorConfig *config);

/**
 * Writes the most recent amplitude sweep configuration to `config`.
 *
 * Returns `RESULT_NO_DATA` if no matching configuration has been received.
 */
enum Result rfe_signal_generator_config_amp_sweep(const struct SignalGenerator *rfe,
                                                  struct SignalGeneratorConfigAmpSweep *config);

/**
 * Writes the most recent CW configuration to `config`.
 *
 * Returns `RESULT_NO_DATA` if no matching configuration has been received.
 */
enum Result rfe_signal_generator_config_cw(const struct SignalGenerator *rfe,
                                           struct SignalGeneratorConfigCw *config);

/**
 * Writes the most recent frequency sweep configuration to `config`.
 *
 * Returns `RESULT_NO_DATA` if no matching configuration has been received.
 */
enum Result rfe_signal_generator_config_freq_sweep(const struct SignalGenerator *rfe,
                                                   struct SignalGeneratorConfigFreqSweep *config);

/**
 * Returns the most recent LCD screen capture.
 *
 * On success, `screen_data` receives a heap-allocated `ScreenData` pointer
 * owned by the caller. Free it with `rfe_screen_data_free`.
 */
enum Result rfe_signal_generator_screen_data(const struct SignalGenerator *rfe,
                                             const struct ScreenData **screen_data);

/**
 * Waits for the next LCD screen capture.
 *
 * On success, `screen_data` receives a heap-allocated `ScreenData` pointer
 * owned by the caller. Free it with `rfe_screen_data_free`.
 */
enum Result rfe_signal_generator_wait_for_next_screen_data(const struct SignalGenerator *rfe,
                                                           const struct ScreenData **screen_data);

/**
 * Waits up to `timeout_secs` seconds for the next LCD screen capture.
 *
 * On success, `screen_data` receives a heap-allocated `ScreenData` pointer
 * owned by the caller. Free it with `rfe_screen_data_free`.
 */
enum Result rfe_signal_generator_wait_for_next_screen_data_with_timeout(const struct SignalGenerator *rfe,
                                                                        uint64_t timeout_secs,
                                                                        const struct ScreenData **screen_data);

/**
 * Writes the most recent temperature range to `temperature`.
 *
 * Returns `RESULT_NO_DATA` if the device has not reported a temperature range.
 */
enum Result rfe_signal_generator_temperature(const struct SignalGenerator *rfe,
                                             Temperature *temperature);

/**
 * Writes the main radio module model to `model`.
 *
 * Returns `RESULT_NO_DATA` if no main model has been reported.
 */
enum Result rfe_signal_generator_main_radio_model(const struct SignalGenerator *rfe,
                                                  SignalGeneratorModel *model);

/**
 * Writes the expansion radio module model to `model`.
 *
 * Returns `RESULT_NO_DATA` if no expansion model has been reported.
 */
enum Result rfe_signal_generator_expansion_radio_model(const struct SignalGenerator *rfe,
                                                       SignalGeneratorModel *model);

/**
 * Writes the currently active radio module model to `model`.
 */
enum Result rfe_signal_generator_active_radio_model(const struct SignalGenerator *rfe,
                                                    SignalGeneratorModel *model);

/**
 * Writes the currently inactive radio module model to `model`.
 *
 * Returns `RESULT_NO_DATA` if no inactive model exists.
 */
enum Result rfe_signal_generator_inactive_radio_model(const struct SignalGenerator *rfe,
                                                      SignalGeneratorModel *model);

/**
 * Starts amplitude sweep mode.
 *
 * `cw_hz` is the CW frequency in hertz and `step_delay_sec` is the delay
 * between amplitude sweep steps in seconds.
 */
enum Result rfe_signal_generator_start_amp_sweep(const struct SignalGenerator *rfe,
                                                 uint64_t cw_hz,
                                                 Attenuation start_attenuation,
                                                 PowerLevel start_power_level,
                                                 Attenuation stop_attenuation,
                                                 PowerLevel stop_power_level,
                                                 uint8_t step_delay_sec);

/**
 * Starts amplitude sweep mode using the expansion module.
 *
 * `cw_hz` is the CW frequency in hertz and `step_delay_sec` is the delay
 * between amplitude sweep steps in seconds.
 */
enum Result rfe_signal_generator_start_amp_sweep_exp(const struct SignalGenerator *rfe,
                                                     uint64_t cw_hz,
                                                     double start_power_dbm,
                                                     double step_power_db,
                                                     double stop_power_dbm,
                                                     uint8_t step_delay_sec);

/**
 * Starts CW mode.
 *
 * `cw_hz` is the CW frequency in hertz.
 */
enum Result rfe_signal_generator_start_cw(const struct SignalGenerator *rfe,
                                          uint64_t cw_hz,
                                          Attenuation attenuation,
                                          PowerLevel power_level);

/**
 * Starts CW mode using the expansion module.
 *
 * `cw_hz` is the CW frequency in hertz.
 */
enum Result rfe_signal_generator_start_cw_exp(const struct SignalGenerator *rfe,
                                              uint64_t cw_hz,
                                              double power_dbm);

/**
 * Starts frequency sweep mode.
 *
 * Frequencies are represented in hertz and `step_delay_sec` is the delay
 * between frequency sweep steps in seconds.
 */
enum Result rfe_signal_generator_start_freq_sweep(const struct SignalGenerator *rfe,
                                                  uint64_t start_hz,
                                                  Attenuation attenuation,
                                                  PowerLevel power_level,
                                                  uint16_t sweep_steps,
                                                  uint64_t step_hz,
                                                  uint8_t step_delay_sec);

/**
 * Starts frequency sweep mode using the expansion module.
 *
 * Frequencies are represented in hertz and `step_delay_sec` is the delay
 * between frequency sweep steps in seconds.
 */
enum Result rfe_signal_generator_start_freq_sweep_exp(const struct SignalGenerator *rfe,
                                                      uint64_t start_hz,
                                                      double power_dbm,
                                                      uint16_t sweep_steps,
                                                      uint64_t step_hz,
                                                      uint8_t step_delay_sec);

/**
 * Starts tracking mode.
 *
 * `start_hz` and `step_hz` are represented in hertz.
 */
enum Result rfe_signal_generator_start_tracking(const struct SignalGenerator *rfe,
                                                uint64_t start_hz,
                                                Attenuation attenuation,
                                                PowerLevel power_level,
                                                uint16_t sweep_steps,
                                                uint64_t step_hz);

/**
 * Starts tracking mode using the expansion module.
 *
 * `start_hz` and `step_hz` are represented in hertz.
 */
enum Result rfe_signal_generator_start_tracking_exp(const struct SignalGenerator *rfe,
                                                    uint64_t start_hz,
                                                    double power_dbm,
                                                    uint16_t sweep_steps,
                                                    uint64_t step_hz);

/**
 * Jumps to a new frequency using the tracking step frequency.
 */
enum Result rfe_signal_generator_tracking_step(const struct SignalGenerator *rfe, uint16_t steps);

/**
 * Sets the callback called when the main signal generator configuration is received.
 *
 * The callback may be invoked from a background thread, and multiple callback
 * invocations may overlap. `user_data`, if non-NULL, must remain valid until
 * the callback is removed or the signal generator is freed.
 */
void rfe_signal_generator_set_config_callback(const struct SignalGenerator *rfe,
                                              void (*callback)(struct SignalGeneratorConfig config,
                                                               void *user_data),
                                              void *user_data);

/**
 * Removes the main configuration callback.
 */
void rfe_signal_generator_remove_config_callback(const struct SignalGenerator *rfe);

/**
 * Sets the callback called when an amplitude sweep configuration is received.
 *
 * The callback may be invoked from a background thread, and multiple callback
 * invocations may overlap. `user_data`, if non-NULL, must remain valid until
 * the callback is removed or the signal generator is freed.
 */
void rfe_signal_generator_set_config_amp_sweep_callback(const struct SignalGenerator *rfe,
                                                        void (*callback)(struct SignalGeneratorConfigAmpSweep config,
                                                                         void *user_data),
                                                        void *user_data);

/**
 * Removes the amplitude sweep configuration callback.
 */
void rfe_signal_generator_remove_config_amp_sweep_callback(const struct SignalGenerator *rfe);

/**
 * Sets the callback called when a CW configuration is received.
 *
 * The callback may be invoked from a background thread, and multiple callback
 * invocations may overlap. `user_data`, if non-NULL, must remain valid until
 * the callback is removed or the signal generator is freed.
 */
void rfe_signal_generator_set_config_cw_callback(const struct SignalGenerator *rfe,
                                                 void (*callback)(struct SignalGeneratorConfigCw config,
                                                                  void *user_data),
                                                 void *user_data);

/**
 * Removes the CW configuration callback.
 */
void rfe_signal_generator_remove_config_cw_callback(const struct SignalGenerator *rfe);

/**
 * Sets the callback called when a frequency sweep configuration is received.
 *
 * The callback may be invoked from a background thread, and multiple callback
 * invocations may overlap. `user_data`, if non-NULL, must remain valid until
 * the callback is removed or the signal generator is freed.
 */
void rfe_signal_generator_set_config_freq_sweep_callback(const struct SignalGenerator *rfe,
                                                         void (*callback)(struct SignalGeneratorConfigFreqSweep config,
                                                                          void *user_data),
                                                         void *user_data);

/**
 * Removes the frequency sweep configuration callback.
 */
void rfe_signal_generator_remove_config_freq_sweep_callback(const struct SignalGenerator *rfe);

/**
 * Turns RF output power on.
 */
enum Result rfe_signal_generator_rf_power_on(const struct SignalGenerator *rfe);

/**
 * Turns RF output power off.
 */
enum Result rfe_signal_generator_rf_power_off(const struct SignalGenerator *rfe);

/**
 * Writes the display name of a spectrum analyzer model.
 *
 * `name_buf` must point to a writable buffer of at least `len` bytes. The
 * buffer receives a null-terminated C string. Returns
 * `RESULT_INVALID_INPUT_ERROR` if `len` is too small or `model` is invalid.
 */
enum Result rfe_spectrum_analyzer_model_name(SpectrumAnalyzerModel model,
                                             char *name_buf,
                                             uintptr_t len);

/**
 * Returns whether the model supports Plus-model features.
 */
bool rfe_spectrum_analyzer_model_is_plus_model(SpectrumAnalyzerModel model);

/**
 * Returns whether the model supports Wi-Fi analyzer mode.
 */
bool rfe_spectrum_analyzer_model_has_wifi_analyzer(SpectrumAnalyzerModel model);

/**
 * Returns the model's minimum supported input frequency in hertz.
 *
 * Returns zero if `model` is invalid.
 */
uint64_t rfe_spectrum_analyzer_model_min_freq_hz(SpectrumAnalyzerModel model);

/**
 * Returns the model's maximum supported input frequency in hertz.
 *
 * Returns zero if `model` is invalid.
 */
uint64_t rfe_spectrum_analyzer_model_max_freq_hz(SpectrumAnalyzerModel model);

/**
 * Returns the model's minimum supported sweep span in hertz.
 *
 * Returns zero if `model` is invalid.
 */
uint64_t rfe_spectrum_analyzer_model_min_span_hz(SpectrumAnalyzerModel model);

/**
 * Returns the model's maximum supported sweep span in hertz.
 *
 * Returns zero if `model` is invalid.
 */
uint64_t rfe_spectrum_analyzer_model_max_span_hz(SpectrumAnalyzerModel model);

/**
 * Connects to the first RF Explorer spectrum analyzer found on a CP210x USB serial port.
 *
 * Returns `NULL` if no compatible device can be opened and initialized. The
 * returned pointer is owned by the caller and must be freed with
 * `rfe_spectrum_analyzer_free`.
 */
struct SpectrumAnalyzer *rfe_spectrum_analyzer_connect(void);

/**
 * Connects to a named serial port using the given baud rate.
 *
 * `name` must be a valid null-terminated UTF-8 string. Returns `NULL` if the
 * pointer is null, the string is invalid, or the device cannot be opened and
 * initialized. The returned pointer is owned by the caller and must be freed
 * with `rfe_spectrum_analyzer_free`.
 */
struct SpectrumAnalyzer *rfe_spectrum_analyzer_connect_with_name_and_baud_rate(const char *name,
                                                                               uint32_t baud_rate);

/**
 * Frees a spectrum analyzer returned by `rfe_spectrum_analyzer_connect`.
 *
 * Passing `NULL` is allowed and has no effect.
 */
void rfe_spectrum_analyzer_free(struct SpectrumAnalyzer *rfe);

/**
 * Sends raw bytes to the spectrum analyzer.
 *
 * `bytes` must point to at least `len` bytes. This function is primarily for
 * advanced users that need to send RF Explorer commands not wrapped by this API.
 */
enum Result rfe_spectrum_analyzer_send_bytes(const struct SpectrumAnalyzer *rfe,
                                             const uint8_t *bytes,
                                             uintptr_t len);

/**
 * Writes the connected serial port name to a caller-provided buffer.
 *
 * Use `rfe_spectrum_analyzer_port_name_len` to get the required buffer size,
 * including the terminating null byte.
 */
enum Result rfe_spectrum_analyzer_port_name(const struct SpectrumAnalyzer *rfe,
                                            char *port_name_buf,
                                            uintptr_t buf_len);

/**
 * Returns the buffer size required for `rfe_spectrum_analyzer_port_name`.
 *
 * The returned size includes the terminating null byte. Returns zero if `rfe`
 * is `NULL`.
 */
uintptr_t rfe_spectrum_analyzer_port_name_len(const struct SpectrumAnalyzer *rfe);

/**
 * Writes the firmware version to a caller-provided buffer.
 *
 * Use `rfe_spectrum_analyzer_firmware_version_len` to get the required buffer
 * size, including the terminating null byte.
 */
enum Result rfe_spectrum_analyzer_firmware_version(const struct SpectrumAnalyzer *rfe,
                                                   char *firmware_version_buf,
                                                   uintptr_t buf_len);

/**
 * Returns the buffer size required for `rfe_spectrum_analyzer_firmware_version`.
 *
 * The returned size includes the terminating null byte. Returns zero if `rfe`
 * is `NULL`.
 */
uintptr_t rfe_spectrum_analyzer_firmware_version_len(const struct SpectrumAnalyzer *rfe);

/**
 * Writes the device serial number to a caller-provided buffer.
 *
 * Use `rfe_spectrum_analyzer_serial_number_len` to get the required buffer
 * size, including the terminating null byte. Returns `RESULT_NO_DATA` if the
 * device does not report a serial number.
 */
enum Result rfe_spectrum_analyzer_serial_number(const struct SpectrumAnalyzer *rfe,
                                                char *serial_number_buf,
                                                uintptr_t buf_len);

/**
 * Returns the buffer size required for `rfe_spectrum_analyzer_serial_number`.
 *
 * The returned size includes the terminating null byte. Returns zero if `rfe`
 * is `NULL` or no serial number has been received.
 */
uintptr_t rfe_spectrum_analyzer_serial_number_len(const struct SpectrumAnalyzer *rfe);

/**
 * Turns the spectrum analyzer LCD on.
 */
enum Result rfe_spectrum_analyzer_lcd_on(const struct SpectrumAnalyzer *rfe);

/**
 * Turns the spectrum analyzer LCD off.
 */
enum Result rfe_spectrum_analyzer_lcd_off(const struct SpectrumAnalyzer *rfe);

/**
 * Enables screen dump messages from the spectrum analyzer.
 */
enum Result rfe_spectrum_analyzer_enable_dump_screen(const struct SpectrumAnalyzer *rfe);

/**
 * Disables screen dump messages from the spectrum analyzer.
 */
enum Result rfe_spectrum_analyzer_disable_dump_screen(const struct SpectrumAnalyzer *rfe);

/**
 * Holds the current spectrum analyzer sweep.
 */
enum Result rfe_spectrum_analyzer_hold(const struct SpectrumAnalyzer *rfe);

/**
 * Reboots the spectrum analyzer.
 *
 * The `rfe` pointer must not be used after a successful reboot unless the
 * device is reconnected.
 */
enum Result rfe_spectrum_analyzer_reboot(struct SpectrumAnalyzer *rfe);

/**
 * Powers off the spectrum analyzer.
 *
 * The `rfe` pointer must not be used after a successful power-off unless the
 * device is reconnected.
 */
enum Result rfe_spectrum_analyzer_power_off(struct SpectrumAnalyzer *rfe);

/**
 * Returns the current sweep start frequency in hertz.
 */
uint64_t rfe_spectrum_analyzer_start_freq_hz(const struct SpectrumAnalyzer *rfe);

/**
 * Returns the current sweep step size in hertz.
 */
uint64_t rfe_spectrum_analyzer_step_size_hz(const struct SpectrumAnalyzer *rfe);

/**
 * Returns the current sweep stop frequency in hertz.
 */
uint64_t rfe_spectrum_analyzer_stop_freq_hz(const struct SpectrumAnalyzer *rfe);

/**
 * Returns the current sweep center frequency in hertz.
 */
uint64_t rfe_spectrum_analyzer_center_freq_hz(const struct SpectrumAnalyzer *rfe);

/**
 * Returns the current sweep span in hertz.
 */
uint64_t rfe_spectrum_analyzer_span_hz(const struct SpectrumAnalyzer *rfe);

/**
 * Returns the active radio module's minimum supported frequency in hertz.
 */
uint64_t rfe_spectrum_analyzer_min_freq_hz(const struct SpectrumAnalyzer *rfe);

/**
 * Returns the active radio module's maximum supported frequency in hertz.
 */
uint64_t rfe_spectrum_analyzer_max_freq_hz(const struct SpectrumAnalyzer *rfe);

/**
 * Returns the active radio module's maximum supported span in hertz.
 */
uint64_t rfe_spectrum_analyzer_max_span_hz(const struct SpectrumAnalyzer *rfe);

/**
 * Returns the resolution bandwidth in hertz.
 *
 * Returns zero if the device has not reported a resolution bandwidth.
 */
uint64_t rfe_spectrum_analyzer_rbw_hz(const struct SpectrumAnalyzer *rfe);

/**
 * Returns the bottom displayed amplitude in dBm.
 */
int16_t rfe_spectrum_analyzer_min_amp_dbm(const struct SpectrumAnalyzer *rfe);

/**
 * Returns the top displayed amplitude in dBm.
 */
int16_t rfe_spectrum_analyzer_max_amp_dbm(const struct SpectrumAnalyzer *rfe);

/**
 * Returns the amplitude offset in dB.
 *
 * Returns zero if the device has not reported an amplitude offset.
 */
int8_t rfe_spectrum_analyzer_amp_offset_db(const struct SpectrumAnalyzer *rfe);

/**
 * Returns the number of points in each sweep.
 */
uint16_t rfe_spectrum_analyzer_sweep_len(const struct SpectrumAnalyzer *rfe);

/**
 * Returns the current operating mode.
 */
Mode rfe_spectrum_analyzer_mode(const struct SpectrumAnalyzer *rfe);

/**
 * Returns the current calculator mode.
 *
 * Returns the enum default if the device has not reported a calculator mode.
 */
CalcMode rfe_spectrum_analyzer_calc_mode(const struct SpectrumAnalyzer *rfe);

/**
 * Copies the most recent sweep into a caller-provided buffer.
 *
 * `sweep_buf` must point to at least `buf_len` `float` values. If `sweep_len`
 * is non-NULL, it is set to the number of values written. Returns
 * `RESULT_INVALID_INPUT_ERROR` if the buffer is too small, or `RESULT_NO_DATA`
 * if no sweep has been received.
 */
enum Result rfe_spectrum_analyzer_sweep(const struct SpectrumAnalyzer *rfe,
                                        float *sweep_buf,
                                        uintptr_t buf_len,
                                        uintptr_t *sweep_len);

/**
 * Waits for the next sweep and copies it into a caller-provided buffer.
 *
 * `sweep_buf` must point to at least `buf_len` `float` values. If `sweep_len`
 * is non-NULL, it is set to the number of values written.
 */
enum Result rfe_spectrum_analyzer_wait_for_next_sweep(const struct SpectrumAnalyzer *rfe,
                                                      float *sweep_buf,
                                                      uintptr_t buf_len,
                                                      uintptr_t *sweep_len);

/**
 * Waits up to `timeout_secs` seconds for the next sweep and copies it into a buffer.
 *
 * `sweep_buf` must point to at least `buf_len` `float` values. If `sweep_len`
 * is non-NULL, it is set to the number of values written.
 */
enum Result rfe_spectrum_analyzer_wait_for_next_sweep_with_timeout(const struct SpectrumAnalyzer *rfe,
                                                                   uint64_t timeout_secs,
                                                                   float *sweep_buf,
                                                                   uintptr_t buf_len,
                                                                   uintptr_t *sweep_len);

/**
 * Returns the most recent LCD screen capture.
 *
 * On success, `screen_data` receives a heap-allocated `ScreenData` pointer
 * owned by the caller. Free it with `rfe_screen_data_free`.
 */
enum Result rfe_spectrum_analyzer_screen_data(const struct SpectrumAnalyzer *rfe,
                                              const struct ScreenData **screen_data);

/**
 * Waits for the next LCD screen capture.
 *
 * On success, `screen_data` receives a heap-allocated `ScreenData` pointer
 * owned by the caller. Free it with `rfe_screen_data_free`.
 */
enum Result rfe_spectrum_analyzer_wait_for_next_screen_data(const struct SpectrumAnalyzer *rfe,
                                                            const struct ScreenData **screen_data);

/**
 * Waits up to `timeout_secs` seconds for the next LCD screen capture.
 *
 * On success, `screen_data` receives a heap-allocated `ScreenData` pointer
 * owned by the caller. Free it with `rfe_screen_data_free`.
 */
enum Result rfe_spectrum_analyzer_wait_for_next_screen_data_with_timeout(const struct SpectrumAnalyzer *rfe,
                                                                         uint64_t timeout_secs,
                                                                         const struct ScreenData **screen_data);

/**
 * Writes the current DSP mode to `dsp_mode`.
 *
 * Returns `RESULT_NO_DATA` if the device has not reported a DSP mode.
 */
enum Result rfe_spectrum_analyzer_dsp_mode(const struct SpectrumAnalyzer *rfe, DspMode *dsp_mode);

/**
 * Writes the current tracking status to `tracking_status`.
 *
 * Returns `RESULT_NO_DATA` if the device has not reported a tracking status.
 */
enum Result rfe_spectrum_analyzer_tracking_status(const struct SpectrumAnalyzer *rfe,
                                                  TrackingStatus *tracking_status);

/**
 * Writes the current input stage to `input_stage`.
 *
 * Returns `RESULT_NO_DATA` if the device has not reported an input stage.
 */
enum Result rfe_spectrum_analyzer_input_stage(const struct SpectrumAnalyzer *rfe,
                                              InputStage *input_stage);

/**
 * Returns the main radio module model.
 *
 * Returns `SPECTRUM_ANALYZER_MODEL_UNKNOWN` if no model has been reported.
 */
SpectrumAnalyzerModel rfe_spectrum_analyzer_main_radio_model(const struct SpectrumAnalyzer *rfe);

/**
 * Returns the expansion radio module model.
 *
 * Returns `SPECTRUM_ANALYZER_MODEL_UNKNOWN` if no expansion model has been reported.
 */
SpectrumAnalyzerModel rfe_spectrum_analyzer_expansion_radio_model(const struct SpectrumAnalyzer *rfe);

/**
 * Returns the currently active radio module model.
 *
 * Returns `SPECTRUM_ANALYZER_MODEL_UNKNOWN` if no model has been reported.
 */
SpectrumAnalyzerModel rfe_spectrum_analyzer_active_radio_model(const struct SpectrumAnalyzer *rfe);

/**
 * Returns the currently inactive radio module model.
 *
 * Returns `SPECTRUM_ANALYZER_MODEL_UNKNOWN` if no inactive model exists.
 */
SpectrumAnalyzerModel rfe_spectrum_analyzer_inactive_radio_model(const struct SpectrumAnalyzer *rfe);

/**
 * Starts Wi-Fi analyzer mode for the requested Wi-Fi band.
 */
enum Result rfe_spectrum_analyzer_start_wifi_analyzer(const struct SpectrumAnalyzer *rfe,
                                                      WifiBand wifi_band);

/**
 * Stops Wi-Fi analyzer mode.
 */
enum Result rfe_spectrum_analyzer_stop_wifi_analyzer(const struct SpectrumAnalyzer *rfe);

/**
 * Requests tracking mode and waits for a tracking status response.
 *
 * `start_hz` is the tracking start frequency in hertz and `step_hz` is the
 * tracking step frequency in hertz.
 */
enum Result rfe_spectrum_analyzer_request_tracking(const struct SpectrumAnalyzer *rfe,
                                                   uint64_t start_hz,
                                                   uint64_t step_hz);

/**
 * Steps over the tracking step frequency and makes a measurement.
 */
enum Result rfe_spectrum_analyzer_tracking_step(const struct SpectrumAnalyzer *rfe, uint16_t step);

/**
 * Sets the sweep start and stop frequencies in hertz.
 */
enum Result rfe_spectrum_analyzer_set_start_stop(const struct SpectrumAnalyzer *rfe,
                                                 uint64_t start_hz,
                                                 uint64_t stop_hz);

/**
 * Sets the sweep start frequency, stop frequency, and number of sweep points.
 *
 * Frequencies are represented in hertz.
 */
enum Result rfe_spectrum_analyzer_set_start_stop_sweep_len(const struct SpectrumAnalyzer *rfe,
                                                           uint64_t start_hz,
                                                           uint64_t stop_hz,
                                                           uint16_t sweep_len);

/**
 * Sets the sweep center frequency and span in hertz.
 */
enum Result rfe_spectrum_analyzer_set_center_span(const struct SpectrumAnalyzer *rfe,
                                                  uint64_t center_hz,
                                                  uint64_t span_hz);

/**
 * Sets the sweep center frequency, span, and number of sweep points.
 *
 * Frequencies are represented in hertz.
 */
enum Result rfe_spectrum_analyzer_set_center_span_sweep_len(const struct SpectrumAnalyzer *rfe,
                                                            uint64_t center_hz,
                                                            uint64_t span_hz,
                                                            uint16_t sweep_len);

/**
 * Sets the minimum and maximum amplitudes displayed on the RF Explorer screen.
 *
 * Amplitudes are represented in dBm.
 */
enum Result rfe_spectrum_analyzer_set_min_max_amps(const struct SpectrumAnalyzer *rfe,
                                                   int16_t min_amp_dbm,
                                                   int16_t max_amp_dbm);

/**
 * Sets the callback called when a sweep is received.
 *
 * The callback may be invoked from a background thread, and multiple callback
 * invocations may overlap. The `sweep` pointer passed to the callback is only
 * valid for the duration of that callback call. `user_data`, if non-NULL, must
 * remain valid until the callback is removed or the analyzer is freed.
 */
void rfe_spectrum_analyzer_set_sweep_callback(const struct SpectrumAnalyzer *rfe,
                                              void (*callback)(const float *sweep,
                                                               uintptr_t sweep_len,
                                                               uint64_t start_hz,
                                                               uint64_t stop_hz,
                                                               void *user_data),
                                              void *user_data);

/**
 * Removes the sweep callback.
 */
void rfe_spectrum_analyzer_remove_sweep_callback(const struct SpectrumAnalyzer *rfe);

/**
 * Sets the callback called when a spectrum analyzer configuration is received.
 *
 * The callback may be invoked from a background thread, and multiple callback
 * invocations may overlap. `user_data`, if non-NULL, must remain valid until
 * the callback is removed or the analyzer is freed.
 */
void rfe_spectrum_analyzer_set_config_callback(const struct SpectrumAnalyzer *rfe,
                                               void (*callback)(struct SpectrumAnalyzerConfig config,
                                                                void *user_data),
                                               void *user_data);

/**
 * Removes the configuration callback.
 */
void rfe_spectrum_analyzer_remove_config_callback(const struct SpectrumAnalyzer *rfe);

/**
 * Sets the number of points in each sweep.
 *
 * Only Plus models support changing the sweep length.
 */
enum Result rfe_spectrum_analyzer_set_sweep_len(const struct SpectrumAnalyzer *rfe,
                                                uint16_t sweep_len);

/**
 * Sets the calculator mode.
 */
enum Result rfe_spectrum_analyzer_set_calc_mode(const struct SpectrumAnalyzer *rfe,
                                                CalcMode calc_mode);

/**
 * Activates the main radio module.
 */
enum Result rfe_spectrum_analyzer_activate_main_radio(const struct SpectrumAnalyzer *rfe);

/**
 * Activates the expansion radio module.
 */
enum Result rfe_spectrum_analyzer_activate_expansion_radio(const struct SpectrumAnalyzer *rfe);

/**
 * Sets the spectrum analyzer input stage.
 */
enum Result rfe_spectrum_analyzer_set_input_stage(const struct SpectrumAnalyzer *rfe,
                                                  InputStage input_stage);

/**
 * Sets the amplitude offset in dB.
 */
enum Result rfe_spectrum_analyzer_set_offset_db(const struct SpectrumAnalyzer *rfe,
                                                int8_t offset_db);

/**
 * Sets the DSP mode.
 */
enum Result rfe_spectrum_analyzer_set_dsp_mode(const struct SpectrumAnalyzer *rfe,
                                               DspMode dsp_mode);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* rfe_h */
