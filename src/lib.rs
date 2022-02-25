pub mod interface;
pub mod prelude;
mod registers;

use interface::WriteFrame;

#[doc(inline)]
pub use registers::analogue_audio_path::{InselV, SideAttdB};
#[doc(inline)]
pub use registers::headphone_out::HpVoldB;
#[doc(inline)]
pub use registers::line_in::InVoldB;

use registers::analogue_audio_path::AnalogueAudioPath;
use registers::digital_audio_path::DigitalAudioPath;
use registers::headphone_out::LeftHeadphoneOut;
use registers::headphone_out::RightHeadphoneOut;
use registers::line_in::LeftLineIn;
use registers::line_in::RightLineIn;
use registers::power_down::PowerDown;

pub struct Wm8731<I>
where
    I: WriteFrame,
{
    interface: I,
    left_line_in: LeftLineIn,
    right_line_in: RightLineIn,
    left_headphone_out_vol: HpVoldB,
    right_headphone_out_vol: HpVoldB,
    analogue_audio_path: AnalogueAudioPath,
    digital_audio_path: DigitalAudioPath,
    power_down: PowerDown,
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
            analogue_audio_path: Default::default(),
            digital_audio_path: Default::default(),
            power_down: Default::default(),
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

    pub fn set_micboost(&mut self, value: bool) -> &mut Self {
        self.analogue_audio_path.set_micboost(value);
        self.interface.write(self.analogue_audio_path.to_frame());
        self
    }

    pub fn set_mutemic(&mut self, value: bool) -> &mut Self {
        self.analogue_audio_path.set_mutemic(value);
        self.interface.write(self.analogue_audio_path.to_frame());
        self
    }

    pub fn set_insel(&mut self, value: InselV) -> &mut Self {
        self.analogue_audio_path.set_insel(value);
        self.interface.write(self.analogue_audio_path.to_frame());
        self
    }

    pub fn set_bypass(&mut self, value: bool) -> &mut Self {
        self.analogue_audio_path.set_bypass(value);
        self.interface.write(self.analogue_audio_path.to_frame());
        self
    }

    pub fn set_dacsel(&mut self, value: bool) -> &mut Self {
        self.analogue_audio_path.set_dacsel(value);
        self.interface.write(self.analogue_audio_path.to_frame());
        self
    }

    pub fn set_sidetone(&mut self, value: bool) -> &mut Self {
        self.analogue_audio_path.set_sidetone(value);
        self.interface.write(self.analogue_audio_path.to_frame());
        self
    }

    pub fn set_sideatt(&mut self, value: SideAttdB) -> &mut Self {
        self.analogue_audio_path.set_sideatt(value);
        self.interface.write(self.analogue_audio_path.to_frame());
        self
    }

    ///  `true` disable ADC high pass filter. `false` enable ADC high pass filter.
    pub fn set_adchpd(&mut self, value: bool) -> &mut Self {
        self.digital_audio_path.set_adchpd(value);
        self.interface.write(self.digital_audio_path.to_frame());
        self
    }

    #[cfg(doc)]
    /// *Unavailable yet because it require some strategies to prevent invalid values.*
    pub fn set_deemp(&mut self, value: bool) -> &mut Self {
        todo!()
    }

    pub fn set_dacmu(&mut self, value: bool) -> &mut Self {
        self.digital_audio_path.set_dacmu(value);
        self.interface.write(self.digital_audio_path.to_frame());
        self
    }

    pub fn set_hpor(&mut self, value: bool) -> &mut Self {
        self.digital_audio_path.set_hpor(value);
        self.interface.write(self.digital_audio_path.to_frame());
        self
    }

    pub fn set_lineinpd(&mut self, value: bool) -> &mut Self {
        self.power_down.set_lineinpd(value);
        self.interface.write(self.power_down.to_frame());
        self
    }
    pub fn set_micpd(&mut self, value: bool) -> &mut Self {
        self.power_down.set_micpd(value);
        self.interface.write(self.power_down.to_frame());
        self
    }
    pub fn set_adcpd(&mut self, value: bool) -> &mut Self {
        self.power_down.set_adcpd(value);
        self.interface.write(self.power_down.to_frame());
        self
    }
    pub fn set_dacpd(&mut self, value: bool) -> &mut Self {
        self.power_down.set_dacpd(value);
        self.interface.write(self.power_down.to_frame());
        self
    }
    pub fn set_outpd(&mut self, value: bool) -> &mut Self {
        self.power_down.set_outpd(value);
        self.interface.write(self.power_down.to_frame());
        self
    }
    pub fn set_oscpd(&mut self, value: bool) -> &mut Self {
        self.power_down.set_oscpd(value);
        self.interface.write(self.power_down.to_frame());
        self
    }
    pub fn set_clkoutpd(&mut self, value: bool) -> &mut Self {
        self.power_down.set_clkoutpd(value);
        self.interface.write(self.power_down.to_frame());
        self
    }
    pub fn set_poweroff(&mut self, value: bool) -> &mut Self {
        self.power_down.set_poweroff(value);
        self.interface.write(self.power_down.to_frame());
        self
    }
}
