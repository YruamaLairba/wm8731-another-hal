use rtic::Mutex;
use rtt_target::rprintln;
use wm8731_another_hal::prelude::*;

/// for cmd taking a bool as parameter
macro_rules! bool_cmd {
    ($name:ident, $set_name:ident) => {
        pub fn $name<I: WriteFrame>(mut wm8731: impl Mutex<T = Wm8731<I>>, val: &str) {
            let val = val.trim();
            if let Ok(val) = val.parse::<bool>() {
                wm8731.lock(|wm8731| {
                    wm8731.$set_name(val);
                });
                rprintln!(concat!(stringify!($name), " {:?}"), val);
            }
        }
    };
}

pub fn hpvol<I: WriteFrame>(mut wm8731: impl Mutex<T = Wm8731<I>>, val: &str) {
    let val = val.trim();
    if val.is_empty() {
    } else if let Ok(val) = val.parse::<i8>() {
        let vol = (val + (HpVoldB::Z0DB.into_raw() as i8)) as u8;
        let vol = HpVoldB::from_raw(vol);
        wm8731.lock(|wm8731| {
            wm8731.set_both_headphone_out_vol(vol, false);
        });
        rprintln!("hpvol {}", vol);
    }
}

pub fn insel<I: WriteFrame>(mut wm8731: impl Mutex<T = Wm8731<I>>, val: &str) {
    let val = val.trim();
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

bool_cmd!(dacsel, set_dacsel);
bool_cmd!(bypass, set_bypass);
bool_cmd!(dacmu, set_dacmu);
