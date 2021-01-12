#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate circuitdojo_tester as hal;
use panic_halt as _;

use hal::clock::GenericClockController;

use hal::delay::Delay;
use hal::prelude::*;
use hal::Pins;

#[rtic::app(device = hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        led_pass: hal::LedPass,
        delay: Delay,
    }
    #[init()]
    fn init(cx: init::Context) -> init::LateResources {
        let mut peripherals = cx.device;
        let mut clocks = GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let pins = Pins::new(peripherals.PORT);
        let led_pass = pins.led_pass.into_push_pull_output();

        let delay = Delay::new(cx.core.SYST, &mut clocks);

        init::LateResources { led_pass, delay }
    }
    #[idle(resources=[led_pass, delay])]
    fn idle(cx: idle::Context) -> ! {
        loop {
            let _ = cx.resources.led_pass.toggle();
            cx.resources.delay.delay_ms(500u32);
        }
    }
};
