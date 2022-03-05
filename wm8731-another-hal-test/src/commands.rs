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
            }
        }
    };
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
                wm8731.set_both_line_in_vol(vol);
            });
            rprintln!("invol set to {}", vol);
        }
    } else {
        let vol = wm8731.lock(|wm8731| wm8731.both_line_in_vol());
        rprintln!("invol: left {}, right {}", vol.0, vol.1);
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
                wm8731.set_both_headphone_out_vol(vol, false);
            });
            rprintln!("hpvol set to {}", vol);
        }
    } else {
        let vol = wm8731.lock(|wm8731| wm8731.both_headphone_out_vol());
        rprintln!("hpvol: left {}, right {}", vol.0, vol.1);
    }
}

bool_cmd!(micboost, set_micboost);
bool_cmd!(mutemic, set_mutemic);

pub fn insel<'a, I: WriteFrame>(
    mut wm8731: impl Mutex<T = Wm8731<I>>,
    mut opts: impl Iterator<Item = &'a str>,
) {
    if let Some(val) = opts.next() {
        let val2 = match val {
            "Line" => InselV::Line,
            "Mic" => InselV::Mic,
            _ => return,
        };
        wm8731.lock(|wm8731| {
            wm8731.set_insel(val2);
        });
        rprintln!("insel {:?}", val);
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
