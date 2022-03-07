use crate::hal::pac::{I2S2EXT, SPI2};
use rtic::Mutex;
use rtt_target::rprintln;
use wm8731_another_hal::prelude::*;

/// for cmd taking a bool as parameter
macro_rules! bool_cmd {
    ($name:ident, $set_name:ident) => {
        pub fn $name<'a, I: WriteFrame>(
            mut wm8731: impl Mutex<T = Wm8731<I>>,
            mut opts: impl Iterator<Item = &'a str>,
        ) {
            if let Some(val) = opts.next() {
                if let Ok(val) = val.parse::<bool>() {
                    wm8731.lock(|wm8731| {
                        wm8731.$set_name(val);
                    });
                    rprintln!(concat!(stringify!($name), " {:?}"), val);
                }
            } else {
                let val = wm8731.lock(|wm8731| wm8731.$name());
                rprintln!(concat!(stringify!($name), " is {}"), val);
            }
        }
    };
}
pub fn status<I: WriteFrame + core::fmt::Debug>(
    mut wm8731: impl Mutex<T = Wm8731<I>>,
    mut i2s: impl Mutex<T = SPI2>,
    mut i2sext: impl Mutex<T = I2S2EXT>,
) {
    match i2s.lock(|i2s| i2s.i2scfgr.read().i2se().is_enabled()) {
        true => rprintln!("i2s is enabled"),
        false => rprintln!("i2s is disabled"),
    }
    match i2sext.lock(|i2sext| i2sext.i2scfgr.read().i2se().is_enabled()) {
        true => rprintln!("i2sext is enabled"),
        false => rprintln!("i2sext is disabled"),
    }
    match wm8731.lock(|wm8731| wm8731.is_active()) {
        true => rprintln!("wm8731 is active"),
        false => rprintln!("wm8731 is inactive"),
    }
    rprintln!(
        "left in: vol: {}, mute: {}",
        wm8731.lock(|wm8731| wm8731.left_invol()),
        wm8731.lock(|wm8731| wm8731.left_inmute())
    );
    rprintln!(
        "right in: vol: {}, mute: {}",
        wm8731.lock(|wm8731| wm8731.right_invol()),
        wm8731.lock(|wm8731| wm8731.right_inmute())
    );
    rprintln!(
        "hpvol: left: {}, right:{}",
        wm8731.lock(|wm8731| wm8731.left_hpvol()),
        wm8731.lock(|wm8731| wm8731.right_hpvol()),
    );

    //Analogue audio path
    let (micboost, mutemic, insel, bypass, dacsel, sidetone, sideatt) = wm8731.lock(|wm8731| {
        (
            wm8731.micboost(),
            wm8731.mutemic(),
            wm8731.insel(),
            wm8731.bypass(),
            wm8731.dacsel(),
            wm8731.sidetone(),
            wm8731.sideatt(),
        )
    });
    rprintln!(
        "micboost: {}, mutemic: {}, insel: {:?}, bypass: {}, dacsel {}, sidetone {}, sideatt {}",
        micboost,
        mutemic,
        insel,
        bypass,
        dacsel,
        sidetone,
        sideatt
    );

    //Digital audio path
    let (adchpd, dacmu, hpor) =
        wm8731.lock(|wm8731| (wm8731.adchpd(), wm8731.dacmu(), wm8731.hpor()));
    rprintln!("adchpd: {}, dacmu: {}, hpor: {}", adchpd, dacmu, hpor);

    //Power Down Control
    let (lineinpd, micpd, adcpd, dacpd, outpd, oscpd, clkoutpd, poweroff) = wm8731.lock(|wm8731| {
        (
            wm8731.lineinpd(),
            wm8731.micpd(),
            wm8731.adcpd(),
            wm8731.dacpd(),
            wm8731.outpd(),
            wm8731.oscpd(),
            wm8731.clkoutpd(),
            wm8731.poweroff(),
        )
    });
    rprintln!(
        "lineinpd: {}, micpd: {}, adcpd: {}, dacpd: {}, outpd: {}, oscpd: {}, clkoutpd: {}, poweroff: {}",
        lineinpd,
        micpd,
        adcpd,
        dacpd,
        outpd,
        oscpd,
        clkoutpd,
        poweroff
    );

    // Digital Audio Interface Format
    let (format, iwl, lrp, lrswap, ms, bclkinv) = wm8731.lock(|wm8731| {
        (
            wm8731.format(),
            wm8731.iwl(),
            wm8731.lrp(),
            wm8731.lrswap(),
            wm8731.ms(),
            wm8731.bclkinv(),
        )
    });
    rprintln!(
        "format: {:?}, iwl: {:?}, lrp: {}, lrswap: {}, ms: {:?}, bclkinv: {}",
        format,
        iwl,
        lrp,
        lrswap,
        ms,
        bclkinv
    );

    // Sampling Control
    let (sampling_rates, clkidiv2, clkodiv2) = wm8731.lock(|wm8731| {
        (
            wm8731.sampling_rates(),
            wm8731.clkidiv2(),
            wm8731.clkodiv2(),
        )
    });
    rprintln!(
        "sampling_rates: {}, clkidiv2: {}, clkodiv2: {}",
        sampling_rates,
        clkidiv2,
        clkodiv2
    );
}

pub fn is_enabled<'a, I: WriteFrame>(
    mut wm8731: impl Mutex<T = Wm8731<I>>,
    mut i2s: impl Mutex<T = SPI2>,
    mut i2sext: impl Mutex<T = I2S2EXT>,
    opts: impl Iterator<Item = &'a str>,
) {
    for device in opts {
        match device {
            "wm" | "wm8731" => match wm8731.lock(|wm8731| wm8731.is_active()) {
                true => rprintln!("wm8731 is enabled"),
                false => rprintln!("wm8731 is disabled"),
            },
            "i2s" => match i2s.lock(|i2s| i2s.i2scfgr.read().i2se().is_enabled()) {
                true => rprintln!("i2s is enabled"),
                false => rprintln!("i2s is disabled"),
            },
            "i2sext" => match i2sext.lock(|i2sext| i2sext.i2scfgr.read().i2se().is_enabled()) {
                true => rprintln!("i2sext is enabled"),
                false => rprintln!("i2sext is disabled"),
            },
            _ => (),
        }
    }
}

pub fn enable<'a, I: WriteFrame>(
    mut wm8731: impl Mutex<T = Wm8731<I>>,
    mut i2s: impl Mutex<T = SPI2>,
    mut i2sext: impl Mutex<T = I2S2EXT>,
    opts: impl Iterator<Item = &'a str>,
) {
    for device in opts {
        match device {
            "wm" | "wm8731" => {
                wm8731.lock(|wm8731| wm8731.activate());
                rprintln!("wm8731 enabled");
            }
            "i2s" => {
                i2s.lock(|i2s| i2s.i2scfgr.modify(|_, w| w.i2se().enabled()));
                rprintln!("i2s enabled");
            }
            "i2sext" => {
                i2sext.lock(|i2sext| i2sext.i2scfgr.modify(|_, w| w.i2se().enabled()));
                rprintln!("i2sext enabled");
            }
            _ => (),
        }
    }
}

pub fn disable<'a, I: WriteFrame>(
    mut wm8731: impl Mutex<T = Wm8731<I>>,
    mut i2s: impl Mutex<T = SPI2>,
    mut i2sext: impl Mutex<T = I2S2EXT>,
    opts: impl Iterator<Item = &'a str>,
) {
    for device in opts {
        match device {
            "wm" | "wm8731" => {
                wm8731.lock(|wm8731| wm8731.activate());
                rprintln!("wm8731 disabled");
            }
            "i2s" => {
                i2s.lock(|i2s| i2s.i2scfgr.modify(|_, w| w.i2se().disabled()));
                rprintln!("i2s disabled");
            }
            "i2sext" => {
                i2sext.lock(|i2sext| i2sext.i2scfgr.modify(|_, w| w.i2se().disabled()));
                rprintln!("i2sext disabled");
            }
            _ => (),
        }
    }
}

pub fn is_active<I: WriteFrame>(mut wm8731: impl Mutex<T = Wm8731<I>>) {
    match wm8731.lock(|wm8731| wm8731.is_active()) {
        true => rprintln!("wm8731 digital audio interface is active"),
        false => rprintln!("wm8731 digital audio interface is inactive"),
    }
}

pub fn activate<I: WriteFrame>(mut wm8731: impl Mutex<T = Wm8731<I>>) {
    wm8731.lock(|wm8731| {
        wm8731.activate();
    });
    rprintln!("wm8731 digital audio interface activated");
}

pub fn deactivate<I: WriteFrame>(mut wm8731: impl Mutex<T = Wm8731<I>>) {
    wm8731.lock(|wm8731| {
        wm8731.deactivate();
    });
    rprintln!("wm8731 digital audio interface deactivated");
}

pub fn invol<'a, I: WriteFrame>(
    mut wm8731: impl Mutex<T = Wm8731<I>>,
    mut opts: impl Iterator<Item = &'a str>,
) {
    if let Some(val) = opts.next() {
        if let Ok(val) = val.parse::<f32>() {
            let coef = 2. / 3.;
            let vol = (val * coef + (InVoldB::Z0DB.into_raw() as f32)) as u8;
            let vol = InVoldB::from_raw(vol);
            wm8731.lock(|wm8731| {
                wm8731.set_both_invol(vol);
            });
            rprintln!("invol set to {}", vol);
        }
    } else {
        let vol = wm8731.lock(|wm8731| wm8731.both_invol());
        rprintln!("invol: left {}, right {}", vol.0, vol.1);
    }
}

pub fn inmute<'a, I: WriteFrame>(
    mut wm8731: impl Mutex<T = Wm8731<I>>,
    mut opts: impl Iterator<Item = &'a str>,
) {
    if let Some(val) = opts.next() {
        if let Ok(val) = val.parse::<bool>() {
            wm8731.lock(|wm8731| {
                wm8731.set_both_inmute(val);
            });
            rprintln!("inmute {:?}", val);
        }
    } else {
        let val = wm8731.lock(|wm8731| wm8731.both_inmute());
        rprintln!("inmute: left {}, right {}", val.0, val.1);
    }
}

pub fn hpvol<'a, I: WriteFrame>(
    mut wm8731: impl Mutex<T = Wm8731<I>>,
    mut opts: impl Iterator<Item = &'a str>,
) {
    if let Some(val) = opts.next() {
        if let Ok(val) = val.parse::<i8>() {
            let vol = (val + (HpVoldB::Z0DB.into_raw() as i8)) as u8;
            let vol = HpVoldB::from_raw(vol);
            wm8731.lock(|wm8731| {
                wm8731.set_both_hpvol(vol, false);
            });
            rprintln!("hpvol set to {}", vol);
        }
    } else {
        let vol = wm8731.lock(|wm8731| wm8731.both_hpvol());
        rprintln!("hpvol: left {}, right {}", vol.0, vol.1);
    }
}

bool_cmd!(micboost, set_micboost);
bool_cmd!(mutemic, set_mutemic);

pub fn insel<'a, I: WriteFrame>(
    mut wm8731: impl Mutex<T = Wm8731<I>>,
    mut opts: impl Iterator<Item = &'a str>,
) {
    use InselV::*;
    if let Some(val) = opts.next() {
        let val2 = match val {
            "line" => Line,
            "mic" => Mic,
            _ => return,
        };
        wm8731.lock(|wm8731| {
            wm8731.set_insel(val2);
        });
        rprintln!("insel {}", val);
    } else {
        match wm8731.lock(|wm8731| wm8731.insel()) {
            Line => rprintln!("insel is line"),
            Mic => rprintln!("insel is mic"),
        }
    }
}

bool_cmd!(bypass, set_bypass);
bool_cmd!(dacsel, set_dacsel);
bool_cmd!(sidetone, set_sidetone);

//sideatt

bool_cmd!(adchpd, set_adchpd);
bool_cmd!(dacmu, set_dacmu);
bool_cmd!(hpor, set_hpor);

// Power down commands
bool_cmd!(lineinpd, set_lineinpd);
bool_cmd!(micpd, set_micpd);
bool_cmd!(adcpd, set_adcpd);
bool_cmd!(dacpd, set_dacpd);
bool_cmd!(outpd, set_outpd);
bool_cmd!(oscpd, set_oscpd);
bool_cmd!(clkoutpd, set_clkoutpd);
bool_cmd!(poweroff, set_poweroff);

// digital audio format
//format
//iwl
bool_cmd!(lrp, set_lrp);
bool_cmd!(lrswap, set_lrswap);
//ms
bool_cmd!(bclkinv, set_bclkinv);

// sampling control
//sampling_rates
bool_cmd!(clkidiv2, set_clkidiv2);
bool_cmd!(clkodiv2, set_clkodiv2);
