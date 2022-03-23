//! Reset register. Reset the device.
#![allow(clippy::new_without_default)]
use crate::interface::Frame;

/// Reset register. Reset the device.
///
/// Can have only one value so it contains not data.
#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
pub struct Reset;

impl Reset {
    pub fn new() -> Self {
        Self
    }
    pub fn to_frame(&self) -> Frame {
        Frame { data: 0b1111 << 9 }
    }
}
