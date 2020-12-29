#![no_std]
#![no_main]

extern crate circuitdojo_tester as hal;
extern crate cortex_m;
extern crate cortex_m_semihosting;
#[cfg(not(feature = "use_semihosting"))]
extern crate panic_halt;
#[cfg(feature = "use_semihosting")]
extern crate panic_semihosting;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::gpio::v2::Pins;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = Pins::new(peripherals.PORT);
    let mut red_led = pins.pa23.into_push_pull_output();

    // Enable the battery output
    let mut vbat_en = pins.pa17.into_push_pull_output();
    vbat_en.set_high().unwrap();

    // Enable the md in
    let mut ps_en = pins.pa22.into_push_pull_output();
    ps_en.set_high().unwrap();

    let mut delay = Delay::new(core.SYST, &mut clocks);
    loop {
        delay.delay_ms(200u8);
        red_led.set_high().unwrap();
        delay.delay_ms(200u8);
        red_led.set_low().unwrap();
    }
}
