//! Analogue Audio Path configuration
#![allow(clippy::new_without_default)]
use crate::interface::Frame;

#[path = "side_att_db.rs"]
mod side_att_db;
pub use side_att_db::*;

/// Analogue audio path configuration builder.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct AnalogueAudioPath {
    data: u8,
}

impl Default for AnalogueAudioPath {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalogueAudioPath {
    pub fn new() -> Self {
        Self { data: 0b1010 }
    }
    pub fn to_frame(&self) -> Frame {
        Frame {
            data: 0b100 << 9 | self.data as u16,
        }
    }
}

impl AnalogueAudioPath {
    /// Get if mic boost enabled.
    pub fn micboost(&self) -> bool {
        let pos = 0;
        self.data & (1 << pos) == 1 << pos
    }
    /// Get if mic is muted
    pub fn mutemic(&self) -> bool {
        let pos = 1;
        self.data & (1 << pos) == 1 << pos
    }
    /// Get if ADC input is conected to line in or microphone.
    pub fn insel(&self) -> InselV {
        let pos = 2;
        match self.data & (1 << pos) == 1 << pos {
            false => InselV::Line,
            true => InselV::Mic,
        }
    }
    /// Get if line in are directly connected to outputs.
    pub fn bypass(&self) -> bool {
        let pos = 3;
        self.data & (1 << pos) == 1 << pos
    }
    /// Get if DAC are connected to outputs.
    pub fn dacsel(&self) -> bool {
        let pos = 4;
        self.data & (1 << pos) == 1 << pos
    }
    /// Get if sidetone path is enabled. When enabled, microphone is directly connected to outputs.
    pub fn sidetone(&self) -> bool {
        let pos = 5;
        self.data & (1 << pos) == 1 << pos
    }
    /// Get attenuation applyed to the sidetone path.
    pub fn sideatt(&self) -> SideAttdB {
        let pos = 6;
        unsafe { SideAttdB::from_raw_unchecked(((self.data & (0b11 << pos)) >> pos) as _) }
    }

    /// Set if mic boost enabled.
    pub fn set_micboost(&mut self, value: bool) -> &mut Self {
        let pos = 0;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    /// Set if mic is muted
    pub fn set_mutemic(&mut self, value: bool) -> &mut Self {
        let pos = 1;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    /// Set if ADC input is conected to line in or microphone.
    pub fn set_insel(&mut self, value: InselV) -> &mut Self {
        let pos = 2;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    /// Set if line in are directly connected to outputs.
    pub fn set_bypass(&mut self, value: bool) -> &mut Self {
        let pos = 3;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    /// Set if DAC are connected to outputs.
    pub fn set_dacsel(&mut self, value: bool) -> &mut Self {
        let pos = 4;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    /// Set if sidetone path is enabled. When enabled, microphone is directly connected to outputs.
    pub fn set_sidetone(&mut self, value: bool) -> &mut Self {
        let pos = 5;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    /// Set attenuation applyed to the sidetone path.
    pub fn set_sideatt(&mut self, value: SideAttdB) -> &mut Self {
        let pos = 6;
        self.data = self.data & !(0b11 << pos) | (value.into_raw() as u8);
        self
    }
}

/// Inputs to ADC selection.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum InselV {
    Line = 0,
    Mic = 1,
}
