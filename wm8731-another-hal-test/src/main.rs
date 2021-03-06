#![no_std]
#![no_main]

use core::panic::PanicInfo;
//use stm32::interrupt;
use rtt_target::rprintln;

use stm32f4xx_hal as hal;

use hal::gpio::{Alternate, NoPin, Output, Pin, PushPull};
use hal::pac;
use hal::spi;
use hal::spi::Spi;
use wm8731_another_hal::prelude::*;
type MyWm8731 = Wm8731<
    SPIInterfaceU8<
        Spi<
            pac::SPI1,
            (
                Pin<Alternate<PushPull, 5_u8>, 'A', 5_u8>,
                NoPin,
                Pin<Alternate<PushPull, 5_u8>, 'A', 7_u8>,
            ),
            spi::TransferModeNormal,
        >,
        stm32f4xx_hal::gpio::Pin<Output<PushPull>, 'B', 2_u8>,
    >,
>;

//pass various datatype to logger
pub enum Log {
    Str(&'static str),
    U8(u8),
    U16(u16),
    I8(i8),
    I16(i16),
}

use core::fmt::Write;
impl Log {
    fn log(&self, chan: &mut rtt_target::UpChannel) {
        match self {
            Log::Str(val) => writeln!(chan, "{}", val).unwrap_or_default(),
            Log::U8(val) => writeln!(chan, "{:08b}", val).unwrap_or_default(),
            Log::U16(val) => writeln!(chan, "{:02x}", val).unwrap_or_default(),
            Log::I8(val) => writeln!(chan, "{:08b}", val).unwrap_or_default(),
            Log::I16(val) => writeln!(chan, "{:02x}", val).unwrap_or_default(),
            //_ => writeln!(chan, "unimplemented").unwrap_or_default(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct I2sLocal {
    pub step: u16,
    pub count: u16,
    pub adc_data: (u16, u16),
}

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true,dispatchers = [EXTI0, EXTI1, EXTI2, EXTI3 ])]
mod app {
    use stm32f4xx_hal as hal;

    use super::I2sLocal;
    use super::Log;
    use super::MyWm8731;
    use commands as cmd;
    use hal::gpio::NoPin;
    use hal::pac::spi1::sr::CHSIDE_A;
    use hal::pac::{I2S2EXT, SPI2};
    use hal::prelude::*;
    use hal::spi;
    use hal::spi::Spi;
    use rtt_target::{rprintln, rtt_init, set_print_channel};
    use setup::*;
    use wm8731_another_hal::prelude::*;
    use wm8731_another_hal_test::*;

    //Clock configuration of the used i2s interface
    const I2SDIV: u8 = 2;
    const ODD: bool = true;

    //generate Master Clock ? Modifying this require to adapt the i2s clock
    const MCK: bool = true;

    #[shared]
    struct Shared {
        i2s2: SPI2,
        i2s2ext: I2S2EXT,
        wm8731: MyWm8731,
    }

    #[local]
    struct Local {
        input: rtt_target::DownChannel,
        i2s_local: I2sLocal,
        log_chan: rtt_target::UpChannel,
        //output: rtt_target::UpChannel,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let channels = rtt_init! {
            up: {
                0: {
                    size: 1024
                    name: "Term"
                }
                1: {
                    size: 512
                    name: "Log"
                }
            }
            down: {
                0: {
                    size: 128
                    mode: BlockIfFull
                    name: "Down zero"
                }
            }
        };
        let output = channels.up.0;
        let log_chan = channels.up.1;
        let input = channels.down.0;
        set_print_channel(output);

        let device = ctx.device;
        let gpioa = device.GPIOA.split();
        let gpiob = device.GPIOB.split();
        let gpioc = device.GPIOC.split();
        let rcc = device.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(96.MHz())
            .hclk(96.MHz())
            .pclk1(50.MHz())
            .pclk2(100.MHz())
            .i2s_clk(61440.kHz())
            .freeze();

        unsafe {
            let rcc = &(*hal::pac::RCC::ptr());
            //enable system clock for APB1 bus and SPI2 (I2S2)
            rcc.apb1enr
                .modify(|_, w| w.pwren().set_bit().spi2en().set_bit());
        }

        //Setup i2s2 and i2s2_ext
        //gpio
        let _pb13 = gpiob.pb13.into_alternate::<5>(); //CK
        let _pb15 = gpiob.pb15.into_alternate::<5>(); //SD
        let _pb14 = gpiob.pb14.into_alternate::<6>(); //ext_SD
        let _pb12 = gpiob.pb12.into_alternate::<5>(); //WS
        let _pc6 = gpioc.pc6.into_alternate::<5>(); //MCK

        //Setup an interrupt that can be triggered by pb12 pin
        //Note: The hal doesn't allow to manipulate interrupt for pin in aternate mode
        let syscfg = device.SYSCFG;
        let exti = device.EXTI;
        //i on pb12
        syscfg
            .exticr4
            .modify(|_, w| unsafe { w.exti12().bits(0b0001) });
        //unmask EXTI interrupt
        exti.imr.modify(|_, w| w.mr12().set_bit());
        //trigger interrupt on rising edge
        exti.rtsr.modify(|_, w| w.tr12().set_bit());

        let mut i2s2 = device.SPI2;
        setup_i2s2(&mut i2s2, I2SDIV, ODD, MCK);

        let mut i2s2ext = device.I2S2EXT;
        setup_i2s2ext(&mut i2s2ext, I2SDIV, ODD, MCK);

        //Spi com
        let pa5 = gpioa.pa5.into_alternate(); //_af5(); //CK
        let pa7 = gpioa.pa7.into_alternate(); //_af5(); //MOSI
        let mut pb2 = gpiob.pb2.into_push_pull_output(); //CS
        let _ = pb2.set_high();

        let spi1_mode = spi::Mode {
            polarity: spi::Polarity::IdleHigh,
            phase: spi::Phase::CaptureOnSecondTransition, //With IdleHigh, capture on rising edge
        };

        let spi1 = Spi::new(
            device.SPI1,
            (pa5, NoPin, pa7),
            spi1_mode,
            500.kHz(),
            &clocks,
        );

        // Create a delay abstraction based on SysTick
        let mut delay = ctx.core.SYST.delay(&clocks);

        rprintln!("Instanciate wm8731");
        let mut wm8731: MyWm8731 = Wm8731::new(SPIInterfaceU8::new(spi1, pb2));
        {
            //power down
            rprintln!("Power Down");
            wm8731.set_lineinpd(false);
            wm8731.set_micpd(false);
            wm8731.set_adcpd(false);
            wm8731.set_dacpd(false);
            wm8731.set_oscpd(false);
            wm8731.set_clkoutpd(false);
            wm8731.set_poweroff(false);
            rprintln!("Mute headphone");
            wm8731.set_both_hpvol(HpVoldB::MUTE, false);
            rprintln!("Unmute line in");
            wm8731.set_both_inmute(false);
            wm8731.set_both_invol(InVoldB::Z0DB);
            rprintln!("Anaoutput Path");
            wm8731.set_micboost(false);
            wm8731.set_mutemic(true);
            wm8731.set_insel(InselV::Line);
            wm8731.set_bypass(false);
            wm8731.set_dacsel(true);
            wm8731.set_sidetone(false);
            //digital_audio_path
            //wm8731.set_adchpd(false);
            wm8731.set_dacmu(false);
            //wm8731.set_deemp(false);
            //digital_audio_interface
            wm8731.set_format(FormatV::I2s);
            wm8731.set_iwl(IwlV::Iwl16Bits);
            wm8731.set_lrp(false);
            wm8731.set_lrswap(false);
            wm8731.set_ms(MsV::Slave);
            wm8731.set_bclkinv(false);
            //sampling
            wm8731.set_sampling_rates(SamplingRates::ADC256_DAC256_A);
            wm8731.set_clkidiv2(false);
            wm8731.set_clkodiv2(false);
            rprintln!("Out power up");
            wm8731.set_outpd(false);
            delay.delay_ms(100_u32);
            rprintln!("Progressive HP vol");
            let mut vol = HpVoldB::MIN;
            while vol != HpVoldB::Z0DB {
                vol.increase();
                wm8731.set_both_hpvol(vol, true);
                delay.delay_ms(10_u32);
            }
        }
        wm8731.activate();
        i2s2ext.i2scfgr.modify(|_, w| w.i2se().enabled());
        i2s2.i2scfgr.modify(|_, w| w.i2se().enabled());

        (
            Shared {
                i2s2,
                i2s2ext,
                wm8731,
            },
            Local {
                input,
                i2s_local: Default::default(),
                log_chan,
            },
            init::Monotonics(),
        )
    }

    #[idle(shared = [wm8731, i2s2, i2s2ext], local = [input])]
    fn idle(cx: idle::Context) -> ! {
        let mut wm8731 = cx.shared.wm8731;
        let mut i2s2 = cx.shared.i2s2;
        let mut i2s2ext = cx.shared.i2s2ext;
        let input = cx.local.input;
        let mut buf = [0u8; 512];
        loop {
            let bytes = input.read(&mut buf[..]);
            let cmds = unsafe { core::str::from_utf8_unchecked(&buf[..bytes]) }.trim_end();
            let cmds = cmds.split(';');
            for cmd in cmds {
                let mut args = cmd.split_ascii_whitespace();
                if let Some(cmd) = args.next() {
                    match cmd {
                        "status" => cmd::status(&mut wm8731, &mut i2s2, &mut i2s2ext),
                        "is_enabled" => cmd::is_enabled(&mut wm8731, &mut i2s2, &mut i2s2ext, args),
                        "enable" => cmd::enable(&mut wm8731, &mut i2s2, &mut i2s2ext, args),
                        "disable" => cmd::disable(&mut wm8731, &mut i2s2, &mut i2s2ext, args),
                        "is_active" => cmd::is_active(&mut wm8731),
                        "activate" => cmd::activate(&mut wm8731),
                        "deactivate" => cmd::deactivate(&mut wm8731),
                        "invol" => cmd::invol(&mut wm8731, args),
                        "inmute" => cmd::inmute(&mut wm8731, args),
                        "hpvol" => cmd::hpvol(&mut wm8731, args),
                        "micboost" => cmd::micboost(&mut wm8731, args),
                        "mutemic" => cmd::mutemic(&mut wm8731, args),
                        "insel" => cmd::insel(&mut wm8731, args),
                        "bypass" => cmd::bypass(&mut wm8731, args),
                        "dacsel" => cmd::dacsel(&mut wm8731, args),
                        "sidetone" => cmd::sidetone(&mut wm8731, args),
                        "adchpd" => cmd::adchpd(&mut wm8731, args),
                        "deemp" => cmd::deemp(&mut wm8731, args),
                        "dacmu" => cmd::dacmu(&mut wm8731, args),
                        "hpor" => cmd::hpor(&mut wm8731, args),
                        "lineinpd" => cmd::lineinpd(&mut wm8731, args),
                        "micpd" => cmd::micpd(&mut wm8731, args),
                        "adcpd" => cmd::adcpd(&mut wm8731, args),
                        "dacpd" => cmd::dacpd(&mut wm8731, args),
                        "outpd" => cmd::outpd(&mut wm8731, args),
                        "oscpd" => cmd::oscpd(&mut wm8731, args),
                        "clkoutpd" => cmd::clkoutpd(&mut wm8731, args),
                        "poweroff" => cmd::poweroff(&mut wm8731, args),
                        "lrp" => cmd::lrp(&mut wm8731, args),
                        "lrswap" => cmd::lrswap(&mut wm8731, args),
                        "bclkinv" => cmd::bclkinv(&mut wm8731, args),
                        "clkidiv2" => cmd::clkidiv2(&mut wm8731, args),
                        "clkodiv2" => cmd::clkodiv2(&mut wm8731, args),
                        _ => (),
                    }
                }
            }

            buf.fill(0);
        }
    }

    #[task(capacity = 10, local = [log_chan])]
    fn logger(cx: logger::Context, log: Log) {
        //cx.local.log_chan.write_str(log).ok();
        log.log(cx.local.log_chan);
    }

    #[task(shared = [i2s2ext])]
    fn resync(cx: resync::Context) {
        let mut i2s2ext = cx.shared.i2s2ext;
        //blocking wait on ws
        while !unsafe {
            let gpiob = &(*hal::pac::GPIOB::ptr());
            gpiob.idr.read().idr12().bit()
        } {}
        i2s2ext.lock(|i2s2ext| i2s2ext.i2scfgr.modify(|_, w| w.i2se().enabled()));
        logger::spawn(Log::Str("Resynced (resync)")).ok();
    }

    #[task(priority = 4, binds = SPI2, local = [i2s_local], shared = [i2s2, i2s2ext, wm8731])]
    fn i2s2(cx: i2s2::Context) {
        use CHSIDE_A::*;
        let mut i2s2 = cx.shared.i2s2;
        let mut i2s2ext = cx.shared.i2s2ext;
        let i2s_local: &mut I2sLocal = cx.local.i2s_local;

        i2s2.lock(|i2s2| {
            let i2s2_sr_read = i2s2.sr.read();
            if i2s2_sr_read.rxne().bit() {
                match i2s2_sr_read.chside().variant() {
                    LEFT => i2s_local.adc_data.0 = i2s2.dr.read().dr().bits(),
                    RIGHT => i2s_local.adc_data.1 = i2s2.dr.read().dr().bits(),
                }
            }
            if i2s2_sr_read.fre().bit() {
                //can never happen in master mode
                logger::spawn(Log::Str("I2sFrameError")).ok();
            }
            if i2s2_sr_read.ovr().bit() {
                logger::spawn(Log::Str("I2sOverrun")).ok();
            }
            if i2s2_sr_read.udr().bit() {
                //can only happen in slave transmission mode
                logger::spawn(Log::Str("I2sUnderrun")).ok();
            }
            //clear error flag
            i2s2.dr.read().bits();
            i2s2.sr.read().bits();
        });

        i2s2ext.lock(|i2s2ext| {
            let i2s2ext_sr_read = i2s2ext.sr.read();
            if i2s2ext_sr_read.txe().bit() {
                match i2s2ext_sr_read.chside().variant() {
                    LEFT => i2s2ext.dr.write(|w| w.dr().bits(i2s_local.adc_data.0)),
                    RIGHT => i2s2ext.dr.write(|w| w.dr().bits(i2s_local.adc_data.1)),
                }
            }

            if i2s2ext_sr_read.fre().bit() {
                logger::spawn(Log::Str("I2sExtFrameError")).ok();
                //can never happen in master mode
                i2s2ext.i2scfgr.modify(|_, w| w.i2se().disabled());
                let gpiob = unsafe { &(*hal::pac::GPIOB::ptr()) };
                let ws = gpiob.idr.read().idr12().bit();
                if ws {
                    i2s2ext.i2scfgr.modify(|_, w| w.i2se().enabled());
                    logger::spawn(Log::Str("Resynced (I2S)")).ok();
                } else {
                    logger::spawn(Log::Str("Enable (Ext)")).ok();
                    let exti = unsafe { &(*hal::pac::EXTI::ptr()) };
                    exti.imr.modify(|_, w| w.mr12().set_bit());
                    resync::spawn().ok();
                }
            }
            if i2s2ext_sr_read.ovr().bit() {
                logger::spawn(Log::Str("I2sExtOverrun")).ok();
            }
            if i2s2ext_sr_read.udr().bit() {
                //can only happen in slave transmission mode
                logger::spawn(Log::Str("I2sExtUnderrun")).ok();
            }
            //clear error flag
            i2s2ext.dr.read();
            i2s2ext.sr.read();
        });
    }

    #[task(priority = 5, binds = EXTI15_10, shared = [i2s2ext])]
    fn exti15_10(cx: exti15_10::Context) {
        logger::spawn(Log::Str("EXTI0")).ok();
        let mut i2s2ext = cx.shared.i2s2ext;
        let exti = unsafe { &(*hal::pac::EXTI::ptr()) };
        let ws = unsafe {
            let gpiob = &(*hal::pac::GPIOB::ptr());
            gpiob.idr.read().idr12().bit()
        };
        //erase the event
        //exti.pr.modify(|_, w| w.pr12().set_bit());
        //look if ws/pb1 is high
        if ws {
            //disable interrupt on EXTI12
            exti.imr.modify(|_, w| w.mr12().clear_bit());
            i2s2ext.lock(|i2s2ext| i2s2ext.i2scfgr.modify(|_, w| w.i2se().enabled()));
            logger::spawn(Log::Str("Resynced (EXTI0)")).ok();
        }
    }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {} // You might need a compiler fence in here.
}
