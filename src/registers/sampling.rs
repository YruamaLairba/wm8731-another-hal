//! Command builder for sampling configuration.
#![allow(clippy::new_without_default)]
use crate::interface::Frame;

/// Builder for sampling command.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Sampling {
    data: u8,
}

impl Default for Sampling {
    fn default() -> Self {
        Self::new()
    }
}

impl Sampling {
    #[allow(clippy::identity_op)]
    pub fn new() -> Self {
        Self { data: 0b0000_0000 }
    }
    pub fn to_frame(&self) -> Frame {
        Frame {
            data: 0b1000 << 9 | self.data as u16,
        }
    }
}
impl Sampling {
    /// Get Sampling Rate. Replace USB/NORMAL, BOSR and SR getters.
    pub fn sampling_rates(&self) -> SamplingRates {
        let pos = 0;
        unsafe { SamplingRates::from_raw_unchecked((self.data & (0b111111 << pos)) >> pos) }
    }
    pub fn clkidiv2(&self) -> bool {
        let pos = 6;
        self.data & (1 << pos) == 1 << pos
    }
    pub fn clkodiv2(&self) -> bool {
        let pos = 7;
        self.data & (1 << pos) == 1 << pos
    }

    /// Set Sampling Rate. Replace USB/NORMAL, BOSR and SR setters.
    pub fn set_sampling_rates(&mut self, value: SamplingRates) -> &mut Self {
        let pos = 0;
        self.data = self.data & !(0b111111 << pos) | (value.into_raw() as u8) << pos;
        self
    }
    pub fn set_clkidiv2(&mut self, value: bool) -> &mut Self {
        let pos = 6;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
    pub fn set_clkodiv2(&mut self, value: bool) -> &mut Self {
        let pos = 7;
        self.data = self.data & !(1 << pos) | (value as u8) << pos;
        self
    }
}

/// Abstraction for Sampling Rates. Replace `USB/NORMAL`, `BOSR` and `SR` fields representation.
///
/// The aim of this type is to provide a safe way to select sampling rate frequencies. It prevent
/// to set unspecified/invalid values or combination of values for `USB/NORMAL`, `BOSR` and `SR`
/// fields. To achieve that, it wrap values of those fields and provide valid values through
/// many associated constants.
///
/// # First naming convention
///
/// The first convention is intended to represent uniquely all possible configurations and for use
/// with non standards clocks. It use a clock divider semantic with the pattern
/// `ADC<div1>_DAC<div2>_<letter>` where:
///   - `<div1>` is how many time the core clock is divided to generate the ADC sampling frequency.
///   - `<div2>` is how many time the core clock is divided to generate the DAC sampling frequency.
///   - `<letter>` is a letter to differentiate configurations producing same clock division :
///     * A is used for configuration presented with 12.288MHz clock in datasheet.
///     * B is used for configuration presented with 11.2896MHz clock in datasheet.
///     * C is used for configuration presented with 18.432MHz clock in datasheet.
///     * D is used for configuration presented with 16.9344MHz clock in datasheet.
///     * U is used for usb mode configuration using 12MHz clock.
///
/// ## Example with `ADC256_DAC1536_A`:
///  - ADC sampling frequency is core clock frequency divided by 256.
///  - DAC sampling frequency is core clock frequency divided by 1536.
///  - This configuration is presented with 12.288MHz clock in datasheet.
///
/// # Second naming convention
///
/// The second convention is intended to make more readable code. TODO!
///
/// # Layout
///
/// Internally, this type encapsulate bits of `SR`,`BOSR` and `USB/NORMAL` fields of Sampling Rate
/// Control register. Below is the internal bits layout:
///
/// | Fields            | SR\[3:0\] | BOSR | USB/NORMAL |
/// |-------------------|-----------|------|------------|
/// | SamplingRates bits | 5:2       | 1    | 0          |
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct SamplingRates {
    inner: u8,
}

#[allow(clippy::derivable_impls)]
impl Default for SamplingRates {
    fn default() -> Self {
        Self { inner: 0 }
    }
}

impl core::fmt::Display for SamplingRates {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            SamplingRates::ADC256_DAC256_A => f.write_str("ADC256_DAC256_A"),
            SamplingRates::ADC256_DAC1536_A => f.write_str("ADC256_DAC1536_A"),
            SamplingRates::ADC1536_DAC256_A => f.write_str("ADC1536_DAC256_A"),
            SamplingRates::ADC1536_DAC1536_A => f.write_str("ADC1536_DAC1536_A"),
            SamplingRates::ADC384_DAC384_A => f.write_str("ADC384_DAC384_A"),
            SamplingRates::ADC128_DAC128_A => f.write_str("ADC128_DAC128_A"),

            SamplingRates::ADC256_DAC256_B => f.write_str("ADC256_DAC256_B"),
            SamplingRates::ADC256_DAC1408_B => f.write_str("ADC256_DAC1408_B"),
            SamplingRates::ADC1408_DAC256_B => f.write_str("ADC1408_DAC256_B"),
            SamplingRates::ADC1408_DAC1408_B => f.write_str("ADC1408_DAC1408_B"),
            SamplingRates::ADC128_DAC128_B => f.write_str("ADC128_DAC128_B"),

            SamplingRates::ADC384_DAC384_C => f.write_str("ADC384_DAC384_C"),
            SamplingRates::ADC384_DAC2304_C => f.write_str("ADC384_DAC2304_C"),
            SamplingRates::ADC2304_DAC384_C => f.write_str("ADC2304_DAC384_C"),
            SamplingRates::ADC2304_DAC2304_C => f.write_str("ADC2304_DAC2304_C"),
            SamplingRates::ADC576_DAC576_C => f.write_str("ADC576_DAC576_C"),
            SamplingRates::ADC192_DAC192_C => f.write_str("ADC192_DAC192_C"),

            SamplingRates::ADC384_DAC384_D => f.write_str("ADC384_DAC384_D"),
            SamplingRates::ADC384_DAC2112_D => f.write_str("ADC384_DAC2112_D"),
            SamplingRates::ADC2112_DAC384_D => f.write_str("ADC2112_DAC384_D"),
            SamplingRates::ADC2112_DAC2112_D => f.write_str("ADC2112_DAC2112_D"),
            SamplingRates::ADC192_DAC192_D => f.write_str("ADC192_DAC192_D"),

            SamplingRates::ADC250_DAC250_U => f.write_str("ADC250_DAC250_U"),
            SamplingRates::ADC272_DAC272_U => f.write_str("ADC272_DAC272_U"),
            SamplingRates::ADC250_DAC1500_U => f.write_str("ADC250_DAC1500_U"),
            SamplingRates::ADC272_DAC1496_U => f.write_str("ADC272_DAC1496_U"),
            SamplingRates::ADC1500_DAC250_U => f.write_str("ADC1500_DAC250_U"),
            SamplingRates::ADC1496_DAC272_U => f.write_str("ADC1496_DAC272_U"),
            SamplingRates::ADC1500_DAC1500_U => f.write_str("ADC1500_DAC1500_U"),
            SamplingRates::ADC1496_DAC1496_U => f.write_str("ADC1496_DAC1496_U"),
            SamplingRates::ADC375_DAC375_U => f.write_str("ADC375_DAC375_U"),
            SamplingRates::ADC125_DAC125_U => f.write_str("ADC125_DAC125_U"),
            SamplingRates::ADC136_DAC136_U => f.write_str("ADC136_DAC136_U"),

            _ => f.write_str("ADC???_DAC???_?"),
        }
    }
}

impl SamplingRates {
    /// Instantiate from raw bits
    ///
    /// # Safety
    ///
    /// Since this type encapsulate `SR`,`BOSR` and `USB/NORMAL` fields and since some combination
    /// of `SR`,`BOSR` and `USB/NORMAL` values are invalid, setting this type from raw value is
    /// unsafe. Look Layout section of type description and look for `SR`,`BOSR` and `USB/NORMAL`
    /// in datasheet to know how to setup raw value.
    pub const unsafe fn from_raw_unchecked(raw: u8) -> Self {
        Self { inner: raw }
    }

    /// Return the raw underlying representation.
    pub const fn into_raw(self) -> u8 {
        self.inner
    }
}

/// Sampling rate configuration usually used with a 12.288MHz core clock.
#[rustfmt::skip]
impl SamplingRates {
    pub const ADC256_DAC256_A:    SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b000000) };
    pub const ADC256_DAC1536_A:   SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b000100) };
    pub const ADC1536_DAC256_A:   SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b001000) };
    pub const ADC1536_DAC1536_A:  SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b001100) };
    pub const ADC384_DAC384_A:    SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b011000) };
    pub const ADC128_DAC128_A:    SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b011100) };
}

/// Sampling rate configuration usually used with a 11.2896MHz core clock.
#[rustfmt::skip]
impl SamplingRates {
    pub const ADC256_DAC256_B:    SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b100000) };
    pub const ADC256_DAC1408_B:   SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b100100) };
    pub const ADC1408_DAC256_B:   SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b101000) };
    pub const ADC1408_DAC1408_B:  SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b101100) };
    pub const ADC128_DAC128_B:    SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b111100) };
}

/// Sampling rate configuration usually used with a 18.432MHz core clock.
#[rustfmt::skip]
impl SamplingRates {
    pub const ADC384_DAC384_C:    SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b000010) };
    pub const ADC384_DAC2304_C:   SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b000110) };
    pub const ADC2304_DAC384_C:   SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b001010) };
    pub const ADC2304_DAC2304_C:  SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b001110) };
    pub const ADC576_DAC576_C:    SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b011010) };
    pub const ADC192_DAC192_C:    SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b011110) };
}

/// Sampling rate configuration usually used with a 16.9344MHz core clock.
#[rustfmt::skip]
impl SamplingRates {
    pub const ADC384_DAC384_D:    SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b100010) };
    pub const ADC384_DAC2112_D:   SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b100110) };
    pub const ADC2112_DAC384_D:   SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b101010) };
    pub const ADC2112_DAC2112_D:  SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b101110) };
    pub const ADC192_DAC192_D:    SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b111110) };
}

/// Sampling rate configuration usually used with a 12MHz core clock generated from a USB clock.
#[rustfmt::skip]
impl SamplingRates {
    pub const ADC250_DAC250_U:    SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b000001) };
    pub const ADC272_DAC272_U:    SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b100011) };
    pub const ADC250_DAC1500_U:   SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b000101) };
    pub const ADC272_DAC1496_U:   SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b100111) };
    pub const ADC1500_DAC250_U:   SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b001001) };
    pub const ADC1496_DAC272_U:   SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b101011) };
    pub const ADC1500_DAC1500_U:  SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b001101) };
    pub const ADC1496_DAC1496_U:  SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b101111) };
    pub const ADC375_DAC375_U:    SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b011011) };
    pub const ADC125_DAC125_U:    SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b011101) };
    pub const ADC136_DAC136_U:    SamplingRates = unsafe { SamplingRates::from_raw_unchecked(0b111111) };
}
