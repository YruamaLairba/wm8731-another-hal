//! Headphone ouputs configuration
#![allow(clippy::new_without_default)]
use crate::interface::Frame;

use core::marker::PhantomData;

#[path = "hp_vol_db.rs"]
mod hp_vol_db;
pub use hp_vol_db::*;

///Marker indicating left channel
pub struct Left;

///Marker indicating right channel
pub struct Right;

///Headphone out configuration builder
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct HeadphoneOut<CHANNEL> {
    data: u16,
    channel: PhantomData<CHANNEL>,
}

///Marker indicating left headphone output concern
pub type LeftHeadphoneOut = HeadphoneOut<Left>;

///Marker indicating left headphone output concern
pub type RightHeadphoneOut = HeadphoneOut<Right>;

impl Default for LeftHeadphoneOut {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RightHeadphoneOut {
    fn default() -> Self {
        Self::new()
    }
}

impl LeftHeadphoneOut {
    pub fn new() -> Self {
        Self {
            data: 0x2 << 9 | 0b0_0111_1001,
            channel: PhantomData::<Left>,
        }
    }

    pub fn to_frame(&self) -> Frame {
        Frame {
            data: 0x2 << 9 | self.data,
        }
    }
}

impl RightHeadphoneOut {
    pub fn new() -> Self {
        Self {
            data: 0x3 << 9 | 0b0_0111_1001,
            channel: PhantomData::<Right>,
        }
    }

    pub fn to_frame(&self) -> Frame {
        Frame {
            data: 0x3 << 9 | self.data,
        }
    }
}

impl<CHANNEL> HeadphoneOut<CHANNEL> {
    /// Get volume.
    pub fn vol(&mut self) -> HpVoldB {
        unsafe { HpVoldB::from_raw_unchecked((self.data & 0b0111_1111) as _) }
    }
    /// Get if volume update on zero cross.
    pub fn zc(&mut self) -> bool {
        let pos = 7;
        self.data & (1 << pos) == 1 << pos
    }
    /// Get if it apply to both channel.
    pub fn both(&mut self) -> bool {
        let pos = 8;
        self.data & (1 << pos) == 1 << pos
    }

    /// Set volume.
    pub fn set_vol(&mut self, volume: HpVoldB) -> &mut Self {
        self.data = self.data & !0b0111_1111 | (volume.into_raw() as u16);
        self
    }
    /// Set if volume will updated on zero cross.
    pub fn set_zc(&mut self, value: bool) -> &mut Self {
        let pos = 7;
        self.data = self.data & !(1 << pos) | (value as u16) << pos;
        self
    }
    /// Set if it apply to both channel.
    pub fn set_both(&mut self, value: bool) -> &mut Self {
        let pos = 8;
        self.data = self.data & !(1 << pos) | (value as u16) << pos;
        self
    }
}
