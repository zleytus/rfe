use std::convert::TryInto;

use chrono::{DateTime, Utc};
use nom::{bytes::complete::tag, bytes::streaming::take, combinator::map_res, IResult};

use crate::common::parsers::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ScreenData {
    screen_data_matrix: Box<[[u8; ScreenData::COLUMNS]; ScreenData::ROWS]>,
    timestamp: DateTime<Utc>,
}

impl ScreenData {
    pub const WIDTH_PX: u8 = 128;
    pub const HEIGHT_PX: u8 = 64;
    pub(crate) const PREFIX: &'static [u8] = b"$D";
    const ROWS: usize = 8;
    const COLUMNS: usize = 128;
    const ROW_HEIGHT_PX: usize = 8;

    /// Returns whether a pixel is on or off at a given xy-coordinate.
    ///
    /// The top-left of the screen is (0, 0) and the bottom-right is (127, 63).
    ///
    /// # Panics
    ///
    /// Panics if the coordinate is out of range.
    pub fn get_pixel(&self, x: u8, y: u8) -> bool {
        let row = usize::from(y) / Self::ROW_HEIGHT_PX;
        let column = usize::from(x);

        (self.screen_data_matrix[row][column] & (1 << (y % 8))) > 0
    }

    /// Returns whether a pixel is on or off at a given xy-coordinate.
    ///
    /// The top-left of the screen is (0, 0) and the bottom-right is (127, 63).
    ///
    /// `None` is returned if the coordinate is out of range.
    pub fn get_pixel_checked(&self, x: u8, y: u8) -> Option<bool> {
        let row = usize::from(y) / Self::ROW_HEIGHT_PX;
        let column = usize::from(x);

        Some((self.screen_data_matrix.get(row)?.get(column)? & (1 << (y % 8))) > 0)
    }

    /// The time at which this `ScreenData` was captured.
    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub(crate) fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        // Parse the prefix of the message
        let (bytes, _) = tag(Self::PREFIX)(bytes)?;

        // Parse the screen data
        let (bytes, screen_data): (&[u8], &[u8; Self::ROWS * Self::COLUMNS]) =
            map_res(take(Self::ROWS * Self::COLUMNS), TryInto::try_into)(bytes)?;

        // Consume any \r or \r\n line endings and make sure there aren't any bytes left
        let (bytes, _) = parse_opt_line_ending(bytes)?;

        // Convert the slice of bytes representing the screen data into a matrix
        let screen_data_matrix = {
            let mut matrix = Box::new([[0; ScreenData::COLUMNS]; ScreenData::ROWS]);
            for (row_index, row_bytes) in screen_data.chunks_exact(ScreenData::COLUMNS).enumerate()
            {
                matrix[row_index].clone_from_slice(row_bytes);
            }
            matrix
        };

        Ok((
            bytes,
            ScreenData {
                screen_data_matrix,
                timestamp: Utc::now(),
            },
        ))
    }
}
