//! Activate or deactivate digital audio interface
#![allow(clippy::new_without_default)]
use crate::interface::Frame;

/// Power down configuration builder.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Active {
    data: bool,
}

impl Default for Active {
    fn default() -> Self {
        Self { data: false }
    }
}

impl Active {
    pub fn new() -> Self {
        Self { data: false }
    }
    pub fn to_frame(&self) -> Frame {
        Frame {
            data: 0b1001 << 9 | self.data as u16,
        }
    }
}

impl Active {
    pub fn get(&self) -> bool {
        self.data
    }
    pub fn set(&mut self, value: bool) -> &mut Self {
        self.data = value;
        self
    }
}
