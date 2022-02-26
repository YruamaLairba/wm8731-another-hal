//! Line inputs configuration
#![allow(clippy::new_without_default)]

use crate::interface::Frame;
use core::marker::PhantomData;

#[path = "in_vol_db.rs"]
mod in_vol_db;
pub use in_vol_db::InVoldB;

///Marker indicating left channel
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Left;

///Marker indicating right channel
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Right;

/// Line in configuration builder.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct LineIn<CHANNEL> {
    data: u16,
    channel: PhantomData<CHANNEL>,
}

/// Left line in configuration builder.
pub type LeftLineIn = LineIn<Left>;

/// Right line in configuration builder.
pub type RightLineIn = LineIn<Right>;

impl Default for LeftLineIn {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RightLineIn {
    fn default() -> Self {
        Self::new()
    }
}

impl LeftLineIn {
    pub fn new() -> Self {
        Self {
            data: 0b0_1001_0111,
            channel: PhantomData::<Left>,
        }
    }

    /// build the corresponding frame to be send throught serial interface.
    pub fn to_frame(&self) -> Frame {
        Frame { data: self.data }
    }
}

impl RightLineIn {
    pub fn new() -> Self {
        Self {
            data: 0x1 << 9 | 0b0_1001_0111,
            channel: PhantomData::<Right>,
        }
    }

    /// build the corresponding frame to be send throught serial interface.
    pub fn to_frame(&self) -> Frame {
        Frame {
            data: 0x1 << 9 | self.data,
        }
    }
}

impl<CHANNEL> LineIn<CHANNEL> {
    /// Get volume.
    pub fn vol(&mut self) -> InVoldB {
        unsafe { InVoldB::from_raw_unchecked((self.data & 0b11111) as _) }
    }
    /// Set volume.
    pub fn set_vol(&mut self, volume: InVoldB) -> &mut Self {
        self.data = self.data & !0b11111 | (volume.into_raw() as u16);
        self
    }

    /// Get if Mute (`true`) or Unmute (`false`).
    pub fn mute(&mut self) -> bool {
        let pos = 7;
        self.data & (1 << pos) == 1 << pos
    }
    /// Set Mute (`true`) or Unmute (`false`).
    pub fn set_mute(&mut self, value: bool) -> &mut Self {
        let pos = 7;
        self.data = self.data & !(1 << pos) | (value as u16) << pos;
        self
    }

    /// Get if both channel are affected.
    pub fn both(&mut self) -> bool {
        let pos = 8;
        self.data & (1 << pos) == 1 << pos
    }
    /// Set if both channel are affected.
    pub fn set_both(&mut self, value: bool) -> &mut Self {
        let pos = 8;
        self.data = self.data & !(1 << pos) | (value as u16) << pos;
        self
    }
}
