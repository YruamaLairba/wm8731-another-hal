use rtic::Mutex;
use rtt_target::rprintln;
use wm8731_another_hal::prelude::*;

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
