#ifndef rfe_h
#define rfe_h

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum RfExplorerResult {
  RF_EXPLORER_RESULT_SUCCESS,
  RF_EXPLORER_RESULT_INVALID_INPUT_ERROR,
  RF_EXPLORER_RESULT_INVALID_OPERATION_ERROR,
  RF_EXPLORER_RESULT_IO_ERROR,
  RF_EXPLORER_RESULT_TIMEOUT_ERROR,
  RF_EXPLORER_RESULT_NULL_PTR_ERROR,
} RfExplorerResult;

typedef struct RfExplorer RfExplorer;

void rfe_free(struct RfExplorer *rfe);

enum RfExplorerResult rfe_send_bytes(struct RfExplorer *rfe, const uint8_t *bytes, uintptr_t len);

enum RfExplorerResult rfe_port_name(const struct RfExplorer *rfe,
                                    char *name_buf,
                                    uintptr_t buf_len);

enum RfExplorerResult rfe_main_module_model(const struct RfExplorer *rfe, Model *model);

enum RfExplorerResult rfe_expansion_module_model(const struct RfExplorer *rfe, Model *model);

enum RfExplorerResult rfe_firmware_version(const struct RfExplorer *rfe,
                                           char *firmware_version_buf,
                                           uintptr_t buf_len);

enum RfExplorerResult rfe_serial_number(struct RfExplorer *rfe,
                                        char *serial_number_buf,
                                        uintptr_t buf_len);

enum RfExplorerResult rfe_lcd_on(struct RfExplorer *rfe);

enum RfExplorerResult rfe_lcd_off(struct RfExplorer *rfe);

enum RfExplorerResult rfe_enable_dump_screen(struct RfExplorer *rfe);

enum RfExplorerResult rfe_disable_dump_screen(struct RfExplorer *rfe);

enum RfExplorerResult rfe_hold(struct RfExplorer *rfe);

enum RfExplorerResult rfe_reboot(struct RfExplorer *rfe);

enum RfExplorerResult rfe_power_off(struct RfExplorer *rfe);

#endif /* rfe_h */
