#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use hal::common::*;
pub use hal::samd21::*;
pub use hal::target_device as pac;

use hal::clock::GenericClockController;
use hal::gpio::v2::{Floating, Input, Pin, PA24, PA25};
// use hal::sercom::v2::UART3;
// use hal::time::Hertz;

#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

// define_pins!(
//     /// Maps the pins to their names and
//     /// the numbers printed on the board.
//     struct Pins,
//     target_device: target_device,

//     /// LEDS
//     pin led_fail = a22,
//     pin led_pass = a23,

//     /// Start switch
//     pin start_sw = a18,

//     /// Enable pins
//     pin vbus_en = a16,
//     pin vbat_en = a17,

//     /// USB D+/- pins
//     pin usb_dm = a24,
//     pin usb_dp = a25,

//     /// DUT related
//     pin en = a15,
//     pin d0 = a13,
//     pin d1 = a12,
//     pin d2 = b15,
//     pin d3 = a11,
//     pin d4 = a10,
//     pin d5 = a9,
//     pin d6 = a8,
//     pin d7 = a7,
//     pin d8 = a6,
//     pin d9 = b13,
//     pin d10 = b12,
//     pin d11 = b11,
//     pin d12 = b10,
//     pin a5 = b8,
//     pin a4 = b7,
//     pin a3 = b6,
//     pin a2 = b5,
//     pin a1 = b4,
//     pin a0 = b3,
//     pin md = b2,
//     pin rst = b0,
//     pin ps_en = b14,

// );

#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: Pin<PA24, Input<Floating>>,
    dp: Pin<PA25, Input<Floating>>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let usb_clock = &clocks.usb(&gclk0).unwrap();

    UsbBusAllocator::new(UsbBus::new(
        usb_clock,
        pm,
        dm.into_alternate(),
        dp.into_alternate(),
        usb,
    ))
}

// /// Convenience for setting up the labelled RX, TX pins to
// /// operate as a UART device running at the specified baud.
// pub fn uart<F: Into<Hertz>>(
//     clocks: &mut GenericClockController,
//     baud: F,
//     sercom3: pac::SERCOM3,
//     pm: &mut pac::PM,
//     rx: Pin<PB14, Input<Floating>>,
//     tx: Pin<PB13, Input<Floating>>,
// ) -> UART3<
//     hal::sercom::Sercom3Pad0<gpio::Pb14<PfD>>,
//     hal::sercom::Sercom3Pad1<gpio::Pb13<PfD>>,
//     (),
//     (),
// > {
//     let gclk0 = clocks.gclk0();

//     UART3::new(
//         &clocks.sercom3_core(&gclk0).unwrap(),
//         baud.into(),
//         sercom3,
//         pm,
//         (rx.into_alternate(), tx.into_alternate()),
//     )
// }
