//! Power down configuration
#![allow(clippy::new_without_default)]
use crate::interface::Frame;

/// Power down configuration builder.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct PowerDown {
    data: u8,
}

impl Default for PowerDown {
    fn default() -> Self {
        Self::new()
    }
}

impl PowerDown {
    pub fn new() -> Self {
        Self { data: 0b1001_1111 }
    }
    pub fn to_frame(&self) -> Frame {
        Frame {
            data: 0b110 << 9 | self.data as u16,
        }
    }
}

impl PowerDown {
    pub fn lineinpd(&self) -> bool {
        let pos = 0;
        self.data & (1 << pos) == 1 << pos
    }
    pub fn micpd(&self) -> bool {
        let pos = 1;
        self.data & (1 << pos) == 1 << pos
    }
    pub fn adcpd(&self) -> bool {
        let pos = 2;
        self.data & (1 << pos) == 1 << pos
    }
    pub fn dacpd(&self) -> bool {
        let pos = 3;
        self.data & (1 << pos) == 1 << pos
    }
    pub fn outpd(&self) -> bool {
        let pos = 4;
        self.data & (1 << pos) == 1 << pos
    }
    pub fn oscpd(&self) -> bool {
        let pos = 5;
        self.data & (1 << pos) == 1 << pos
    }
    pub fn clkoutpd(&self) -> bool {
        let pos = 6;
        self.data & (1 << pos) == 1 << pos
    }
    pub fn poweroff(&self) -> bool {
        let pos = 7;
        self.data & (1 << pos) == 1 << pos
    }

    pub fn set_lineinpd(&mut self, value: bool) -> &mut Self {
        let pos = 0;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    pub fn set_micpd(&mut self, value: bool) -> &mut Self {
        let pos = 1;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    pub fn set_adcpd(&mut self, value: bool) -> &mut Self {
        let pos = 2;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    pub fn set_dacpd(&mut self, value: bool) -> &mut Self {
        let pos = 3;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    pub fn set_outpd(&mut self, value: bool) -> &mut Self {
        let pos = 4;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    pub fn set_oscpd(&mut self, value: bool) -> &mut Self {
        let pos = 5;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    pub fn set_clkoutpd(&mut self, value: bool) -> &mut Self {
        let pos = 6;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    pub fn set_poweroff(&mut self, value: bool) -> &mut Self {
        let pos = 7;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
}
