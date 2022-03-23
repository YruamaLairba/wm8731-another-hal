//! A thin HAL to control and configure the Cirrus Logic/Wolfson WM8731 audio codec
//!
//! This crate provide abstractions to communicate with the WM8731
#![doc = concat!("[Software Control Interface](",
    env!("CARGO_MANIFEST_DIR"),
    "/WM8731_v4.9.pdf#%5B%7B%22num%22%3A141%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22FitH%22%7D%2C560%5D)")]
//! and edit content of
#![doc = concat!("[Control Registers](",
    env!("CARGO_MANIFEST_DIR"),
    "/WM8731_v4.9.pdf#%5B%7B%22num%22%3A139%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22FitH%22%7D%2C697%5D).")]
//! Look [Wm8731] documentation to know how to control the codec.
//!
//! Part of this documentation refers to WM8731 datasheets. Copy are available in the project
//! repository.
#![no_std]
#![doc(html_root_url = "https://docs.rs/embedded-hal/0.2.0/")]

pub mod interface;
pub mod prelude;
pub mod registers;

use interface::WriteFrame;

#[doc(inline)]
pub use registers::analogue_audio_path::{InselV, SideAttdB};
#[doc(inline)]
pub use registers::digital_audio_interface::{FormatV, IwlV, MsV};
#[doc(inline)]
pub use registers::digital_audio_path::DeempV;
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

/// Wm8731 Control Interface Abstraction.
///
/// This type provide abstraction of Wm8731 Control Interface. The control is mainly done through
/// getters and setters methods giving access to a field of a control register. Since control
/// registers are write only, this type actually mirror registers content to simulate read/write
/// operation to a particular field.
///
/// # Build an instance
///
/// Building a `Wm8731` instance is a two step operation. First, you need to instantiate an
/// interface handling communication details, and then you use this interface to instantiate
/// a `Wm8731` object. When you have a SPI or an I2C object implementing embeded_hal blocking
/// write, this is actually fast to do.
///
/// ## Building with an I2C interface
///
/// In the following example, `i2c1` implements [`embedded_hal::blocking::i2c::Write`]
/// ```
/// use wm8731_another_hal::prelude::*;
///
/// let address = 0b001_1010; //Wm8731 address when CSB is low
/// let wm8731 = Wm8731::new(I2CInterface::new(i2c1, address));
/// ```
///
/// ## Building with a SPI interface
///
/// In the following example, `spi1` implements [`embedded_hal::blocking::spi::Write<u8>`] and `cs_pin`
/// implements [`embedded_hal::digital::v2::OutputPin`]
/// ```
/// use wm8731_another_hal::prelude::*;
///
/// let wm8731 = Wm8731::new(SPIInterfaceU8::new(spi1, cs_pin));
/// ```
/// # Usage
///
/// In general, A Wm8731 object just gives you access to a control register field through one
/// getter and one setter method. Getter is just the field name in lower case and setter is the
/// field name prefixed with `set_`. So, `set_mutemic(true)` mute the mic at ADC input. However
/// there is some exception:
///  - fields affecting line and headphone volume, to provide more appropriate methods and to allow
/// code factorisation internally.
///  - `USB\NORMAL`, `BOSR` and `SR` fields. They are replace by [`SamplingRates`] to prevent invalid
///  setting.
///
///  Please also note that all setters affecting Digital Audio Interface Format and Sampling Control
///  register don't change field content when the device is active. This is done to prevent
///  synchronisation issues.
///
///  And finally, to know effect of each field, consult the
#[doc = concat!("[Datasheet page 50](", env!("CARGO_MANIFEST_DIR"), "/WM8731_v4.9.pdf#page=50).")]
#[derive(Debug)]
pub struct Wm8731<I>
where
    I: WriteFrame,
{
    interface: I,
    left_line_in: LeftLineIn,
    right_line_in: RightLineIn,
    left_hpvol: HpVoldB,
    right_hpvol: HpVoldB,
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
            left_hpvol: Default::default(),
            right_hpvol: Default::default(),
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
        self.left_hpvol = Default::default();
        self.right_hpvol = Default::default();
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
    pub fn left_invol(&self) -> InVoldB {
        self.left_line_in.vol()
    }

    pub fn right_invol(&self) -> InVoldB {
        self.right_line_in.vol()
    }

    pub fn both_invol(&self) -> (InVoldB, InVoldB) {
        (self.left_line_in.vol(), self.right_line_in.vol())
    }

    pub fn left_inmute(&self) -> bool {
        self.left_line_in.mute()
    }

    pub fn right_inmute(&self) -> bool {
        self.right_line_in.mute()
    }

    pub fn both_inmute(&self) -> (bool, bool) {
        (self.left_line_in.mute(), self.right_line_in.mute())
    }

    pub fn set_left_invol(&mut self, volume: InVoldB) {
        self.left_line_in.set_vol(volume);
        self.left_line_in.set_both(false);
        self.interface.write(self.left_line_in.to_frame());
    }

    pub fn set_right_invol(&mut self, volume: InVoldB) {
        self.right_line_in.set_vol(volume);
        self.right_line_in.set_both(false);
        self.interface.write(self.right_line_in.to_frame());
    }

    pub fn set_both_invol(&mut self, volume: InVoldB) {
        self.left_line_in.set_vol(volume);
        self.right_line_in.set_vol(volume);
        self.left_line_in.set_both(true);
        self.interface.write(self.left_line_in.to_frame());
    }

    pub fn set_left_inmute(&mut self, mute: bool) {
        self.left_line_in.set_mute(mute);
        self.left_line_in.set_both(false);
        self.interface.write(self.left_line_in.to_frame());
    }

    pub fn set_right_inmute(&mut self, mute: bool) {
        self.right_line_in.set_mute(mute);
        self.right_line_in.set_both(false);
        self.interface.write(self.right_line_in.to_frame());
    }

    pub fn set_both_inmute(&mut self, mute: bool) {
        self.left_line_in.set_mute(mute);
        self.right_line_in.set_mute(mute);
        self.left_line_in.set_both(true);
        self.interface.write(self.left_line_in.to_frame());
    }
}

/// Left and Right Headphone Out.
impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    pub fn left_hpvol(&self) -> HpVoldB {
        self.left_hpvol
    }

    pub fn right_hpvol(&self) -> HpVoldB {
        self.right_hpvol
    }

    pub fn both_hpvol(&self) -> (HpVoldB, HpVoldB) {
        (self.left_hpvol, self.right_hpvol)
    }

    /// Set left headphone out volume.
    ///
    /// When `zcen` is `false`, volume is changed immediately.
    ///
    /// When `zcen` is `true`, volume is set when signal is close to zero to avoid audible
    /// noise. The volume may never change if signal at gain stage input get never close to +/-
    /// 20mv.
    pub fn set_left_hpvol(&mut self, volume: HpVoldB, zcen: bool) {
        self.left_hpvol = volume;
        self.interface.write(
            LeftHeadphoneOut::default()
                .set_both(false)
                .set_zcen(zcen)
                .set_vol(volume)
                .to_frame(),
        );
    }

    /// Set right headphone out volume.
    ///
    /// When `zcen` is `false`, volume is changed immediately.
    ///
    /// When `zcen` is `true`, volume is set when signal is close to zero to avoid audible
    /// noise. The volume may never change if signal at gain stage input get never close to +/-
    /// 20mv.
    pub fn set_right_hpvol(&mut self, volume: HpVoldB, zcen: bool) {
        self.right_hpvol = volume;
        self.interface.write(
            RightHeadphoneOut::default()
                .set_both(false)
                .set_zcen(zcen)
                .set_vol(volume)
                .to_frame(),
        );
    }

    /// Set both headphone out volume.
    ///
    /// When `zcen` is `false`, volume is changed immediately.
    ///
    /// When `zcen` is `true`, volume is set when signal is close to zero to avoid audible
    /// noise. The volume may never change if signal at gain stage input get never close to +/-
    /// 20mv.
    pub fn set_both_hpvol(&mut self, volume: HpVoldB, zcen: bool) {
        self.left_hpvol = volume;
        self.right_hpvol = volume;
        self.interface.write(
            LeftHeadphoneOut::default()
                .set_both(true)
                .set_zcen(zcen)
                .set_vol(volume)
                .to_frame(),
        );
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

    pub fn set_micboost(&mut self, value: bool) {
        self.analogue_audio_path.set_micboost(value);
        self.interface.write(self.analogue_audio_path.to_frame());
    }

    pub fn set_mutemic(&mut self, value: bool) {
        self.analogue_audio_path.set_mutemic(value);
        self.interface.write(self.analogue_audio_path.to_frame());
    }

    pub fn set_insel(&mut self, value: InselV) {
        self.analogue_audio_path.set_insel(value);
        self.interface.write(self.analogue_audio_path.to_frame());
    }

    pub fn set_bypass(&mut self, value: bool) {
        self.analogue_audio_path.set_bypass(value);
        self.interface.write(self.analogue_audio_path.to_frame());
    }

    pub fn set_dacsel(&mut self, value: bool) {
        self.analogue_audio_path.set_dacsel(value);
        self.interface.write(self.analogue_audio_path.to_frame());
    }

    pub fn set_sidetone(&mut self, value: bool) {
        self.analogue_audio_path.set_sidetone(value);
        self.interface.write(self.analogue_audio_path.to_frame());
    }

    pub fn set_sideatt(&mut self, value: SideAttdB) {
        self.analogue_audio_path.set_sideatt(value);
        self.interface.write(self.analogue_audio_path.to_frame());
    }
}

/// Digital Audio Path Control.
impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    ///  `true` means ADC high pass filter disabled. `false` means ADC high pass filter enabled.
    pub fn adchpd(&self) -> bool {
        self.digital_audio_path.adchpd()
    }

    pub fn deemp(&self) -> DeempV {
        self.digital_audio_path.deemp()
    }

    pub fn dacmu(&self) -> bool {
        self.digital_audio_path.dacmu()
    }

    pub fn hpor(&self) -> bool {
        self.digital_audio_path.hpor()
    }

    /// Disable/Enable ADC high pass filter. `true` to disable it, `false` to enable it.
    pub fn set_adchpd(&mut self, value: bool) {
        self.digital_audio_path.set_adchpd(value);
        self.interface.write(self.digital_audio_path.to_frame());
    }

    /// Disable or select a de-emphasis filter. It's up to user to choose the correct value.
    ///
    /// When using de-emphasis, the correct value of `DEEMP` should match the actual DAC sampling
    /// frequency. It's up to user to choose the correct value because actual sampling
    /// frequency depends on clocks and this HAL doesn't know about clocks. Setting a wrong value
    /// is not unsafe, it just apply a filter that doesn't conform
    /// with CD de-emphasis.
    pub fn set_deemp(&mut self, value: DeempV) {
        self.digital_audio_path.set_deemp(value);
        self.interface.write(self.digital_audio_path.to_frame());
    }

    /// DAC Soft Mute Control. Doesn't work correctly with some sampling configurations.
    ///
    /// DAC Soft Mute Control doesn't work correctly when `SR` is `0b0111` or `0b1111`. This concern
    /// sampling configurations where `core clock` / `sampling frequency` is less or equal to
    /// 192.
    pub fn set_dacmu(&mut self, value: bool) {
        self.digital_audio_path.set_dacmu(value);
        self.interface.write(self.digital_audio_path.to_frame());
    }

    pub fn set_hpor(&mut self, value: bool) {
        self.digital_audio_path.set_hpor(value);
        self.interface.write(self.digital_audio_path.to_frame());
    }
}

/// Power Down Control.
impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    pub fn lineinpd(&self) -> bool {
        self.power_down.lineinpd()
    }
    pub fn micpd(&self) -> bool {
        self.power_down.micpd()
    }
    pub fn adcpd(&self) -> bool {
        self.power_down.adcpd()
    }
    pub fn dacpd(&self) -> bool {
        self.power_down.dacpd()
    }
    pub fn outpd(&self) -> bool {
        self.power_down.outpd()
    }
    pub fn oscpd(&self) -> bool {
        self.power_down.oscpd()
    }
    pub fn clkoutpd(&self) -> bool {
        self.power_down.clkoutpd()
    }
    pub fn poweroff(&self) -> bool {
        self.power_down.poweroff()
    }

    pub fn set_lineinpd(&mut self, value: bool) {
        self.power_down.set_lineinpd(value);
        self.interface.write(self.power_down.to_frame());
    }
    pub fn set_micpd(&mut self, value: bool) {
        self.power_down.set_micpd(value);
        self.interface.write(self.power_down.to_frame());
    }
    pub fn set_adcpd(&mut self, value: bool) {
        self.power_down.set_adcpd(value);
        self.interface.write(self.power_down.to_frame());
    }
    pub fn set_dacpd(&mut self, value: bool) {
        self.power_down.set_dacpd(value);
        self.interface.write(self.power_down.to_frame());
    }
    pub fn set_outpd(&mut self, value: bool) {
        self.power_down.set_outpd(value);
        self.interface.write(self.power_down.to_frame());
    }
    pub fn set_oscpd(&mut self, value: bool) {
        self.power_down.set_oscpd(value);
        self.interface.write(self.power_down.to_frame());
    }
    pub fn set_clkoutpd(&mut self, value: bool) {
        self.power_down.set_clkoutpd(value);
        self.interface.write(self.power_down.to_frame());
    }
    pub fn set_poweroff(&mut self, value: bool) {
        self.power_down.set_poweroff(value);
        self.interface.write(self.power_down.to_frame());
    }
}

/// Digital Audio Interface Format. Value stored only if inactive, sent during activation.
impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    pub fn format(&self) -> FormatV {
        self.digital_audio_interface.format()
    }
    pub fn iwl(&self) -> IwlV {
        self.digital_audio_interface.iwl()
    }
    pub fn lrp(&self) -> bool {
        self.digital_audio_interface.lrp()
    }
    pub fn lrswap(&self) -> bool {
        self.digital_audio_interface.lrswap()
    }
    pub fn ms(&self) -> MsV {
        self.digital_audio_interface.ms()
    }
    pub fn bclkinv(&self) -> bool {
        self.digital_audio_interface.bclkinv()
    }

    pub fn set_format(&mut self, value: FormatV) {
        if !self.active.get() {
            self.digital_audio_interface.set_format(value);
        }
    }
    pub fn set_iwl(&mut self, value: IwlV) {
        if !self.active.get() {
            self.digital_audio_interface.set_iwl(value);
        }
    }
    pub fn set_lrp(&mut self, value: bool) {
        if !self.active.get() {
            self.digital_audio_interface.set_lrp(value);
        }
    }
    pub fn set_lrswap(&mut self, value: bool) {
        if !self.active.get() {
            self.digital_audio_interface.set_lrswap(value);
        }
    }
    pub fn set_ms(&mut self, value: MsV) {
        if !self.active.get() {
            self.digital_audio_interface.set_ms(value);
        }
    }
    pub fn set_bclkinv(&mut self, value: bool) {
        if !self.active.get() {
            self.digital_audio_interface.set_bclkinv(value);
        }
    }
}

/// Sampling Control. Value stored only if inactive, sent only during activation.
impl<I> Wm8731<I>
where
    I: WriteFrame,
{
    /// Get Sampling Rates configuration.
    pub fn sampling_rates(&self) -> SamplingRates {
        self.sampling.sampling_rates()
    }
    pub fn clkidiv2(&self) -> bool {
        self.sampling.clkidiv2()
    }
    pub fn clkodiv2(&self) -> bool {
        self.sampling.clkodiv2()
    }

    /// Set Sampling Rates.
    pub fn set_sampling_rates(&mut self, value: SamplingRates) {
        if !self.active.get() {
            self.sampling.set_sampling_rates(value);
        }
    }
    pub fn set_clkidiv2(&mut self, value: bool) {
        if !self.active.get() {
            self.sampling.set_clkidiv2(value);
        }
    }
    pub fn set_clkodiv2(&mut self, value: bool) {
        if !self.active.get() {
            self.sampling.set_clkodiv2(value);
        }
    }
}
