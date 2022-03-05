//! Digital Audio Path configuration
#![allow(clippy::new_without_default)]
use crate::interface::Frame;

/// builder for digital audio interface configuration
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct DigitalAudioInterface {
    data: u8,
}

impl Default for DigitalAudioInterface {
    fn default() -> Self {
        Self::new()
    }
}

impl DigitalAudioInterface {
    pub fn new() -> Self {
        Self { data: 0b1010 }
    }
    pub fn to_frame(&self) -> Frame {
        Frame {
            data: 0b111 << 9 | self.data as u16,
        }
    }
}
impl DigitalAudioInterface {
    pub fn format(&self) -> FormatV {
        let pos = 0;
        match (self.data & (0b11 << pos)) >> pos {
            0b11 => FormatV::Dsp,
            0b10 => FormatV::I2s,
            0b01 => FormatV::LeftJustified,
            0b00 => FormatV::RigthJustified,
            _ => unreachable!(),
        }
    }
    pub fn iwl(&self) -> IwlV {
        let pos = 2;
        match (self.data & (0b11 << pos)) >> pos {
            0b11 => IwlV::Iwl32Bits,
            0b10 => IwlV::Iwl24Bits,
            0b01 => IwlV::Iwl20Bits,
            0b00 => IwlV::Iwl16Bits,
            _ => unreachable!(),
        }
    }
    pub fn lrp(&self) -> bool {
        let pos = 4;
        self.data & (1 << pos) == 1 << pos
    }
    pub fn lrswap(&self) -> bool {
        let pos = 5;
        self.data & (1 << pos) == 1 << pos
    }
    pub fn ms(&self) -> MsV {
        let pos = 6;
        match self.data & (1 << pos) == 1 << pos {
            true => MsV::Master,
            false => MsV::Slave,
        }
    }
    pub fn bclkinv(&self) -> bool {
        let pos = 7;
        self.data & (1 << pos) == 1 << pos
    }

    pub fn set_format(&mut self, value: FormatV) -> &mut Self {
        let pos = 0;
        self.data = self.data & !(0b11 << pos) | (value as u8) << pos;
        self
    }
    pub fn set_iwl(&mut self, value: IwlV) -> &mut Self {
        let pos = 2;
        self.data = self.data & !(0b11 << pos) | (value as u8) << pos;
        self
    }
    pub fn set_lrp(&mut self, value: bool) -> &mut Self {
        let pos = 4;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    pub fn set_lrswap(&mut self, value: bool) -> &mut Self {
        let pos = 5;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    pub fn set_ms(&mut self, value: MsV) -> &mut Self {
        let pos = 6;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    pub fn set_bclkinv(&mut self, value: bool) -> &mut Self {
        let pos = 7;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
}

pub enum FormatV {
    Dsp = 0b11,
    I2s = 0b10,
    LeftJustified = 0b01,
    RigthJustified = 0b00,
}

pub enum IwlV {
    Iwl32Bits = 0b11,
    Iwl24Bits = 0b10,
    Iwl20Bits = 0b01,
    Iwl16Bits = 0b00,
}

pub enum MsV {
    Master = 0b1,
    Slave = 0b0,
}
