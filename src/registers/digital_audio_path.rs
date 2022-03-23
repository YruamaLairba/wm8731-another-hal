//! Digital Audio Path Control.
#![allow(clippy::new_without_default)]
use crate::interface::Frame;

/// Digital Audio Path Control register.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct DigitalAudioPath {
    data: u8,
}

impl Default for DigitalAudioPath {
    fn default() -> Self {
        Self::new()
    }
}

impl DigitalAudioPath {
    pub fn new() -> Self {
        Self { data: 0b1000 }
    }
    pub fn to_frame(&self) -> Frame {
        Frame {
            data: 0b101 << 9 | self.data as u16,
        }
    }
}
impl DigitalAudioPath {
    pub fn adchpd(&self) -> bool {
        let pos = 0;
        self.data & (1 << pos) == 1 << pos
    }
    pub fn deemp(&self) -> DeempV {
        let pos = 1;
        match (self.data & (0b11 << pos)) >> pos {
            0b00 => DeempV::Disable,
            0b01 => DeempV::F32k,
            0b10 => DeempV::F44k1,
            0b11 => DeempV::F48k,
            _ => unreachable!(),
        }
    }
    pub fn dacmu(&self) -> bool {
        let pos = 3;
        self.data & (1 << pos) == 1 << pos
    }
    pub fn hpor(&self) -> bool {
        let pos = 4;
        self.data & (1 << pos) == 1 << pos
    }

    pub fn set_adchpd(&mut self, value: bool) -> &mut Self {
        let pos = 0;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    pub fn set_deemp(&mut self, value: DeempV) -> &mut Self {
        let pos = 1;
        self.data = self.data & !(0b11 << pos) | (value as u8) << pos;
        self
    }
    /// DAC Soft Mute Control. Does'nt work correctly with some sampling configurations.
    ///
    /// DAC Soft Mute Control doesn't work correctly when `SR` is `0b0111` or `0b1111`. This concern
    /// sampling configurations where `core clock` / `sampling frequency` is less or equal to
    /// 192.
    pub fn set_dacmu(&mut self, value: bool) -> &mut Self {
        let pos = 3;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    pub fn set_hpor(&mut self, value: bool) -> &mut Self {
        let pos = 4;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
}

/// De-emphasis filter selection.
pub enum DeempV {
    Disable = 0b00,
    F32k = 0b01,
    F44k1 = 0b10,
    F48k = 0b11,
}
