pub mod interface;
pub mod prelude;
mod registers;

use interface::WriteFrame;

#[doc(inline)]
pub use registers::headphone_out::HpVoldB;
#[doc(inline)]
pub use registers::line_in::InVoldB;

use registers::headphone_out::LeftHeadphoneOut;
use registers::headphone_out::RightHeadphoneOut;
use registers::line_in::LeftLineIn;
use registers::line_in::RightLineIn;

pub struct Wm8731<I>
where
    I: WriteFrame,
{
    interface: I,
    left_line_in: LeftLineIn,
    right_line_in: RightLineIn,
    left_headphone_out_vol: HpVoldB,
    right_headphone_out_vol: HpVoldB,
}

impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    ///Instantiate a driver. This also reset the codec to guarantee a known state.
    pub fn new(interface: I) -> Self {
        let mut codec = Self {
            interface,
            left_line_in: Default::default(),
            right_line_in: Default::default(),
            left_headphone_out_vol: Default::default(),
            right_headphone_out_vol: Default::default(),
        };
        codec.reset();
        codec
    }

    /// Reset the codec. All configuration is lost.
    pub fn reset(&mut self) {
        todo!();
    }
}

impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    pub fn set_left_line_in_vol(&mut self, volume: InVoldB) -> &mut Self {
        self.left_line_in.set_vol(volume);
        self.left_line_in.set_both(false);
        self.interface.write(self.left_line_in.to_frame());
        self
    }

    pub fn set_right_line_in_vol(&mut self, volume: InVoldB) -> &mut Self {
        self.right_line_in.set_vol(volume);
        self.right_line_in.set_both(false);
        self.interface.write(self.right_line_in.to_frame());
        self
    }

    pub fn set_both_line_in_vol(&mut self, volume: InVoldB) -> &mut Self {
        self.left_line_in.set_vol(volume);
        self.right_line_in.set_vol(volume);
        self.left_line_in.set_both(true);
        self.interface.write(self.left_line_in.to_frame());
        self
    }

    pub fn set_left_line_in_mute(&mut self, mute: bool) -> &mut Self {
        self.left_line_in.set_mute(mute);
        self.left_line_in.set_both(false);
        self.interface.write(self.left_line_in.to_frame());
        self
    }

    pub fn set_right_line_in_mute(&mut self, mute: bool) -> &mut Self {
        self.right_line_in.set_mute(mute);
        self.right_line_in.set_both(false);
        self.interface.write(self.right_line_in.to_frame());
        self
    }

    pub fn set_both_line_in_mute(&mut self, mute: bool) -> &mut Self {
        self.left_line_in.set_mute(mute);
        self.right_line_in.set_mute(mute);
        self.left_line_in.set_both(true);
        self.interface.write(self.left_line_in.to_frame());
        self
    }

    /// Set left headphone out volume.
    ///
    /// When `zero_cross` is `false`, volume is changed immediately.
    ///
    /// When `zero_cross` is `true`, volume is set when signal is close to zero to avoid audible
    /// noise. The volume may never change if signal at gain stage input get never close to +/-
    /// 20mv.
    pub fn set_left_headphone_out_vol(&mut self, volume: HpVoldB, zero_cross: bool) -> &mut Self {
        self.left_headphone_out_vol = volume;
        self.interface.write(
            LeftHeadphoneOut::default()
                .set_both(false)
                .set_zc(zero_cross)
                .set_vol(volume)
                .to_frame(),
        );
        self
    }

    /// Set right headphone out volume.
    ///
    /// When `zero_cross` is `false`, volume is changed immediately.
    ///
    /// When `zero_cross` is `true`, volume is set when signal is close to zero to avoid audible
    /// noise. The volume may never change if signal at gain stage input get never close to +/-
    /// 20mv.
    pub fn set_right_headphone_out_vol(&mut self, volume: HpVoldB, zero_cross: bool) -> &mut Self {
        self.right_headphone_out_vol = volume;
        self.interface.write(
            RightHeadphoneOut::default()
                .set_both(false)
                .set_zc(zero_cross)
                .set_vol(volume)
                .to_frame(),
        );
        self
    }

    /// Set both headphone out volume.
    ///
    /// When `zero_cross` is `false`, volume is changed immediately.
    ///
    /// When `zero_cross` is `true`, volume is set when signal is close to zero to avoid audible
    /// noise. The volume may never change if signal at gain stage input get never close to +/-
    /// 20mv.
    pub fn set_both_headphone_out_vol(&mut self, volume: HpVoldB, zero_cross: bool) -> &mut Self {
        self.left_headphone_out_vol = volume;
        self.right_headphone_out_vol = volume;
        self.interface.write(
            LeftHeadphoneOut::default()
                .set_both(true)
                .set_zc(zero_cross)
                .set_vol(volume)
                .to_frame(),
        );
        self
    }
}
