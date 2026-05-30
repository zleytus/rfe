use rfe::ScreenData;

use super::Result;

/// Gets one pixel from an RF Explorer LCD screen capture.
///
/// The top-left pixel is `(0, 0)` and the bottom-right pixel is `(127, 63)`.
/// On success, `pixel` is set to `true` for an enabled pixel and `false` for a
/// disabled pixel. Returns `RESULT_INVALID_INPUT_ERROR` if the coordinates are
/// out of range.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_screen_data_get_pixel(
    screen_data: Option<&ScreenData>,
    x: u8,
    y: u8,
    pixel: Option<&mut bool>,
) -> Result {
    rfe_screen_data_get_pixel_checked(screen_data, x, y, pixel)
}

/// Gets one pixel from an RF Explorer LCD screen capture with bounds checking.
///
/// This is equivalent to `rfe_screen_data_get_pixel`; both functions return
/// `RESULT_INVALID_INPUT_ERROR` for out-of-range coordinates.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_screen_data_get_pixel_checked(
    screen_data: Option<&ScreenData>,
    x: u8,
    y: u8,
    pixel: Option<&mut bool>,
) -> Result {
    let (Some(screen_data), Some(pixel)) = (screen_data, pixel) else {
        return Result::NullPtrError;
    };

    if let Some(pixel_val) = screen_data.get_pixel_checked(x, y) {
        *pixel = pixel_val;
        Result::Success
    } else {
        Result::InvalidInputError
    }
}

/// Writes the screen capture timestamp as Unix seconds.
#[unsafe(no_mangle)]
pub extern "C" fn rfe_screen_data_timestamp(
    screen_data: Option<&ScreenData>,
    timestamp: Option<&mut i64>,
) -> Result {
    if let (Some(screen_data), Some(timestamp)) = (screen_data, timestamp) {
        *timestamp = screen_data.timestamp().timestamp();
        Result::Success
    } else {
        Result::NullPtrError
    }
}

/// Frees screen data returned by an `rfe_*_screen_data` function.
///
/// Passing `NULL` is allowed and has no effect.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rfe_screen_data_free(screen_data: Option<&mut ScreenData>) {
    if let Some(screen_data) = screen_data {
        drop(unsafe { Box::from_raw(screen_data) });
    }
}
