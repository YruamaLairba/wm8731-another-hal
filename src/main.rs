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

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true)]
mod app {
    use super::MyWm8731;
    use core::fmt::Write;
    use hal::gpio::NoPin;
    use hal::prelude::*;
    use hal::spi;
    use hal::spi::Spi;
    use rtt_target::{rprintln, rtt_init, set_print_channel};
    use stm32f4xx_hal as hal;
    use wm8731_another_hal::prelude::*;

    #[shared]
    struct Shared {
        wm8731: MyWm8731,
    }

    #[local]
    struct Local {
        input: rtt_target::DownChannel,
        //output: rtt_target::UpChannel,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let channels = rtt_init! {
            up: {
                0: {
                    size: 512
                    name: "Up zero"
                }
                1: {
                    size: 1024
                    name: "Up one"
                }
            }
            down: {
                0: {
                    size: 512
                    mode: BlockIfFull
                    name: "Down zero"
                }
            }
        };
        let output = channels.up.0;
        let mut log = channels.up.1;
        let input = channels.down.0;
        set_print_channel(output);

        let device = ctx.device;
        let gpioa = device.GPIOA.split();
        let gpiob = device.GPIOB.split();
        let _gpioc = device.GPIOC.split();
        let rcc = device.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(96.MHz())
            .hclk(96.MHz())
            .pclk1(50.MHz())
            .pclk2(100.MHz())
            .freeze();

        let pa5 = gpioa.pa5.into_alternate(); //_af5(); //CK
        let pa7 = gpioa.pa7.into_alternate(); //_af5(); //MOSI
        let mut pb2 = gpiob.pb2.into_push_pull_output(); //CS
        let _ = pb2.set_high();

        let spi1_mode = spi::Mode {
            polarity: spi::Polarity::IdleHigh,
            phase: spi::Phase::CaptureOnSecondTransition, //With IdleHigh, capture on rising edge
        };

        let spi1 = Spi::new(device.SPI1, (pa5, NoPin, pa7), spi1_mode, 1.MHz(), &clocks);

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
            wm8731.set_both_headphone_out_vol(HpVoldB::MUTE, false);
            rprintln!("Unmute line in");
            wm8731.set_both_line_in_mute(false);
            rprintln!("Anaoutput Path");
            wm8731.set_micboost(false);
            wm8731.set_mutemic(true);
            wm8731.set_insel(InselV::Line);
            wm8731.set_bypass(true);
            wm8731.set_dacsel(false);
            wm8731.set_sidetone(false);
            //digital_audio_path
            wm8731.set_adchpd(false);
            //wm8731.set_deemp(false);
            wm8731.set_dacsel(false);
            //digital_audio_interface
            wm8731.set_format(FormatV::I2s);
            wm8731.set_iwl(IwlV::Iwl32Bits);
            wm8731.set_lrp(false);
            wm8731.set_lrswap(false);
            wm8731.set_ms(MsV::Slave);
            wm8731.set_bclkinv(false);
            //sampling
            wm8731.set_sampling_rates(SamplingRates::ADC256_DAC256_A);
            wm8731.set_clkidiv2(false);
            wm8731.set_clkodiv2(false);
            delay.delay_ms(10_u32);
            rprintln!("Out power up");
            wm8731.set_outpd(false);
            rprintln!("Progressive HP vol");
            let mut vol = HpVoldB::MIN;
            while vol != HpVoldB::Z0DB {
                vol.increase();
                wm8731.set_both_headphone_out_vol(vol, true);
                delay.delay_ms(10_u32);
            }
            writeln!(log, "{:#?}", wm8731).ok();
        }

        (Shared { wm8731 }, Local { input }, init::Monotonics())
    }

    #[idle(shared = [wm8731], local = [input])]
    fn idle(cx: idle::Context) -> ! {
        let mut wm8731 = cx.shared.wm8731;
        let input = cx.local.input;
        let mut buf = [0u8; 512];
        loop {
            let bytes = input.read(&mut buf[..]);
            let cmd = unsafe { core::str::from_utf8_unchecked(&buf[..bytes]) }.trim_end();

            //hpvol ctrl
            if let Some(val) = cmd.strip_prefix("hpvol") {
                let val = val.trim();
                if let Ok(val) = val.parse::<i8>() {
                    let vol = (val + (HpVoldB::Z0DB.into_raw() as i8)) as u8;
                    let vol = HpVoldB::from_raw(vol);
                    wm8731.lock(|wm8731| {
                        wm8731.set_both_headphone_out_vol(vol, false);
                    });
                    rprintln!("hpvol {}", vol);
                }
            }

            //insel
            if let Some(val) = cmd.strip_prefix("insel") {
                let val = val.trim();
                let val2 = match val {
                    "Line" => InselV::Line,
                    "Mic" => InselV::Mic,
                    _ => continue,
                };
                wm8731.lock(|wm8731| {
                    wm8731.set_insel(val2);
                });
                rprintln!("insel {:?}", val);
            }

            //dacsel
            if let Some(val) = cmd.strip_prefix("dacsel") {
                let val = val.trim();
                if let Ok(val) = val.parse::<bool>() {
                    wm8731.lock(|wm8731| {
                        wm8731.set_dacsel(val);
                    });
                    rprintln!("dacsel {:?}", val);
                }
            }

            //bypass
            if let Some(val) = cmd.strip_prefix("bypass") {
                let val = val.trim();
                if let Ok(val) = val.parse::<bool>() {
                    wm8731.lock(|wm8731| {
                        wm8731.set_bypass(val);
                    });
                    rprintln!("bypass {:?}", val);
                }
            }
            buf.fill(0);
        }
    }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {} // You might need a compiler fence in here.
}
