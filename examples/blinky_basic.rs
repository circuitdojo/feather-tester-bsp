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
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

// Enable semihosting
use cortex_m_semihosting::{hprint, hprintln};

#[entry]
fn main() -> ! {
    // Core peripherals
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    // Clock used for time tracking
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    #[cfg(feature = "use_semihosting")]
    hprintln!("Tester started.").unwrap();

    // Get the pins
    let port0 = peripherals.PORT.in0.read().bits();
    let _ = hprint!("Port: {:X?}\n", port0);

    // Split the pins
    let mut pins = hal::Pins::new(peripherals.PORT).split();

    // Set up the LEDs
    let mut led_fail = pins.leds.fail.into_open_drain_output(&mut pins.port);
    let mut led_pass = pins.leds.pass.into_open_drain_output(&mut pins.port);

    // Set up the other pins
    let _vbus_en = pins.pwr.vbus.into_push_pull_output(&mut pins.port);
    let _vbat_en = pins.pwr.vbat.into_push_pull_output(&mut pins.port);

    // let arry = [_vbus_en,_vbat_en];

    // Set up the reset and enable pins
    let _dut_en = pins.ctrl.en.into_pull_up_input(&mut pins.port);
    let _dut_rst = pins.ctrl.rst.into_pull_up_input(&mut pins.port);

    // Delay used to block the processor
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // Loop
    loop {
        delay.delay_ms(500u32);
        led_fail.set_high().unwrap();
        led_pass.set_low().unwrap();
        delay.delay_ms(500u32);
        led_fail.set_low().unwrap();
        led_pass.set_high().unwrap();

        #[cfg(feature = "use_semihosting")]
        hprintln!(".").unwrap();
    }
}
