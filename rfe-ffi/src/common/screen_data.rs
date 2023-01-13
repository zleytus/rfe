use rfe::ScreenData;

use super::Result;

#[no_mangle]
pub extern "C" fn rfe_screen_data_get_pixel(
    screen_data: Option<&ScreenData>,
    x: u8,
    y: u8,
    pixel: Option<&mut bool>,
) -> Result {
    if let (Some(screen_data), Some(pixel)) = (screen_data, pixel) {
        *pixel = screen_data.get_pixel(x, y);
        Result::Success
    } else {
        Result::NullPtrError
    }
}

#[no_mangle]
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

#[no_mangle]
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

#[no_mangle]
pub unsafe extern "C" fn rfe_screen_data_free(screen_data: Option<&mut ScreenData>) {
    if let Some(screen_data) = screen_data {
        drop(Box::from_raw(screen_data));
    }
}
