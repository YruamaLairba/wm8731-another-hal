#![no_std]

#[macro_use]
mod macros;

pub mod interface;
pub mod prelude;
mod registers;

use interface::WriteFrame;

#[doc(inline)]
pub use registers::analogue_audio_path::{InselV, SideAttdB};
#[doc(inline)]
pub use registers::digital_audio_interface::{FormatV, IwlV, MsV};
#[doc(inline)]
pub use registers::headphone_out::HpVoldB;
#[doc(inline)]
pub use registers::line_in::InVoldB;
#[doc(inline)]
pub use registers::sampling::SamplingRates;

use registers::active::Active;
use registers::analogue_audio_path::AnalogueAudioPath;
use registers::digital_audio_interface::DigitalAudioInterface;
use registers::digital_audio_path::DigitalAudioPath;
use registers::headphone_out::LeftHeadphoneOut;
use registers::headphone_out::RightHeadphoneOut;
use registers::line_in::LeftLineIn;
use registers::line_in::RightLineIn;
use registers::power_down::PowerDown;
use registers::reset::Reset;
use registers::sampling::Sampling;

/// The codec driver.
#[derive(Debug)]
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
    digital_audio_interface: DigitalAudioInterface,
    sampling: Sampling,
    active: Active,
}

/// Constructor and Destructor.
impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    ///Instantiate a driver. This also reset the codec to guarantee a known coherent state.
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
            digital_audio_interface: Default::default(),
            sampling: Default::default(),
            active: Default::default(),
        };
        codec.interface.write(Reset::new().to_frame());
        codec
    }

    /// Destroy the driver and release it's serial interface abstraction.
    pub fn release(self) -> I {
        self.interface
    }
}

/// Active Control and Reset
impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    /// Returns `true` if the digital audio interface is running.
    pub fn is_active(&self) -> bool {
        self.active.get()
    }

    /// Activate digital audio interface.
    pub fn activate(&mut self) {
        self.interface
            .write(self.digital_audio_interface.to_frame());
        self.interface.write(self.sampling.to_frame());
        self.active.set(true);
        self.interface.write(self.active.to_frame());
    }
    /// Deactivate digital audio interface.
    pub fn deactivate(&mut self) {
        self.active.set(false);
        self.interface.write(self.active.to_frame());
    }

    /// Reset the codec. All configuration is lost.
    pub fn reset(&mut self) {
        self.interface.write(Reset::new().to_frame());
        self.left_line_in = Default::default();
        self.right_line_in = Default::default();
        self.left_headphone_out_vol = Default::default();
        self.right_headphone_out_vol = Default::default();
        self.analogue_audio_path = Default::default();
        self.digital_audio_path = Default::default();
        self.power_down = Default::default();
        self.digital_audio_interface = Default::default();
        self.sampling = Default::default();
        self.active = Default::default();
    }
}

/// Left and Right Line In.
impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    pub fn left_line_in_vol(&self) -> InVoldB {
        self.left_line_in.vol()
    }

    pub fn right_line_in_vol(&self) -> InVoldB {
        self.right_line_in.vol()
    }

    pub fn both_line_in_vol(&self) -> (InVoldB, InVoldB) {
        (self.left_line_in.vol(), self.right_line_in.vol())
    }

    pub fn left_line_in_mute(&self) -> bool {
        self.left_line_in.mute()
    }

    pub fn right_line_in_mute(&self) -> bool {
        self.right_line_in.mute()
    }

    pub fn both_line_in_mute(&self) -> (bool, bool) {
        (self.left_line_in.mute(), self.right_line_in.mute())
    }

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
}

/// Left and Right Headphone Out.
impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    pub fn left_headphone_out_vol(&self) -> HpVoldB {
        self.left_headphone_out_vol
    }

    pub fn right_headphone_out_vol(&self) -> HpVoldB {
        self.right_headphone_out_vol
    }

    pub fn both_headphone_out_vol(&self) -> (HpVoldB, HpVoldB) {
        (self.left_headphone_out_vol, self.right_headphone_out_vol)
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

/// Analogue Audio Path Control.
impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    pub fn micboost(&self) -> bool {
        self.analogue_audio_path.micboost()
    }

    pub fn mutemic(&self) -> bool {
        self.analogue_audio_path.mutemic()
    }

    pub fn insel(&self) -> InselV {
        self.analogue_audio_path.insel()
    }

    pub fn bypass(&self) -> bool {
        self.analogue_audio_path.bypass()
    }

    pub fn dacsel(&self) -> bool {
        self.analogue_audio_path.dacsel()
    }

    pub fn sidetone(&self) -> bool {
        self.analogue_audio_path.sidetone()
    }

    pub fn sideatt(&self) -> SideAttdB {
        self.analogue_audio_path.sideatt()
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
}

/// Digital Audio Path Control.
impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    ///  `true` means ADC high pass filter disabled. `false` means ADC high pass filter enbaled.
    pub fn adchpd(&self) -> bool {
        self.digital_audio_path.adchpd()
    }

    #[cfg(doc)]
    /// *Unavailable yet because it require some strategies to get meaningfull values.*
    pub fn deemp(&self) -> bool {
        todo!()
    }

    pub fn dacmu(&self) -> bool {
        self.digital_audio_path.dacmu()
    }

    pub fn hpor(&self) -> bool {
        self.digital_audio_path.hpor()
    }

    /// Disable/Enable ADC high pass filter. `true` to disable it, `false` to enable it.
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

    #[doc = concat!(warning!("May be buggy")," DAC Soft Mute Control")]
    ///
    /// **May be buggy:** With some chips, actual behavior doesn't correspond to the datasheet
    /// description and may interfere with `DACSEL`, `BYPASS` and `SIDETONE` setting. A workaround
    /// consist to always resend an analogue path control command right after a digital path
    /// control command.
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
}

/// Power Down Control.
impl<I> Wm8731<I>
where
    I: WriteFrame,
{
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

/// Digital Audio Interface Format. Value stored only if inactive, sended during activation.
impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    pub fn set_format(&mut self, value: FormatV) -> &mut Self {
        if !self.active.get() {
            self.digital_audio_interface.set_format(value);
        }
        self
    }
    pub fn set_iwl(&mut self, value: IwlV) -> &mut Self {
        if !self.active.get() {
            self.digital_audio_interface.set_iwl(value);
        }
        self
    }
    pub fn set_lrp(&mut self, value: bool) -> &mut Self {
        if !self.active.get() {
            self.digital_audio_interface.set_lrp(value);
        }
        self
    }
    pub fn set_lrswap(&mut self, value: bool) -> &mut Self {
        if !self.active.get() {
            self.digital_audio_interface.set_lrswap(value);
        }
        self
    }
    pub fn set_ms(&mut self, value: MsV) -> &mut Self {
        if !self.active.get() {
            self.digital_audio_interface.set_ms(value);
        }
        self
    }
    pub fn set_bclkinv(&mut self, value: bool) -> &mut Self {
        if !self.active.get() {
            self.digital_audio_interface.set_bclkinv(value);
        }
        self
    }
}

/// Sampling Control. Value stored only if inactive, sended only during activation.
impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    /// Set Sampling Rates.
    pub fn set_sampling_rates(&mut self, value: SamplingRates) -> &mut Self {
        if !self.active.get() {
            self.sampling.set_sampling_rates(value);
        }
        self
    }
    pub fn set_clkidiv2(&mut self, value: bool) -> &mut Self {
        if !self.active.get() {
            self.sampling.set_clkidiv2(value);
        }
        self
    }
    pub fn set_clkodiv2(&mut self, value: bool) -> &mut Self {
        if !self.active.get() {
            self.sampling.set_clkodiv2(value);
        }
        self
    }
}
