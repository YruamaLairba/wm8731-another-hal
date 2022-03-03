use stm32f4xx_hal as hal;

use hal::pac::{I2S2EXT, SPI2};

pub fn setup_i2s2(spi2: &mut SPI2, i2sdiv: u8, odd: bool, mck: bool) {
    //i2s2 interrupt and dma
    spi2.cr2.modify(|_, w| {
        w.txeie().clear_bit();
        w.rxneie().set_bit();
        w.errie().set_bit();
        w.rxdmaen().clear_bit()
    });
    //setup spi2 peripheral into i2s mode
    spi2.i2spr.modify(|_, w| {
        unsafe { w.i2sdiv().bits(i2sdiv) };
        w.odd().bit(odd);
        w.mckoe().bit(mck)
    });
    spi2.i2scfgr.modify(|_, w| {
        w.i2smod().i2smode(); //
        w.i2scfg().master_rx(); //
        w.pcmsync().long(); //
        w.i2sstd().philips(); //
        w.ckpol().idle_high(); //
        w.datlen().sixteen_bit(); //
        w.chlen().sixteen_bit(); //
        w.i2se().disabled()
    });
}

pub fn setup_i2s2ext(i2s2ext: &mut I2S2EXT, i2sdiv: u8, odd: bool, mck: bool) {
    //i2s2_ext interrupt and dma
    i2s2ext.cr2.modify(|_, w| {
        w.txeie().set_bit();
        w.rxneie().clear_bit();
        w.errie().set_bit();
        w.txdmaen().clear_bit()
    });
    //setup i2s2ext peripheral
    i2s2ext.i2spr.modify(|_, w| {
        unsafe { w.i2sdiv().bits(i2sdiv) };
        w.odd().bit(odd);
        w.mckoe().bit(mck)
    });
    i2s2ext.i2scfgr.modify(|_, w| {
        w.i2smod().i2smode(); //
        w.i2scfg().slave_tx(); //
        w.pcmsync().long(); //
        w.i2sstd().philips(); //
        w.ckpol().idle_high(); //
        w.datlen().sixteen_bit(); //
        w.chlen().sixteen_bit(); //
        w.i2se().disabled()
    });
}
