#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::prelude::*;
use hal::*;

pub use hal::common::*;
pub use hal::samd21::*;
pub use hal::target_device as pac;

use crate::hal::gpio::{self, *};
use gpio::{Floating, Input, Port};

use hal::clock::GenericClockController;
// use hal::sercom::UART3;
// use hal::time::Hertz;

#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

define_pins!(
    /// Maps the pins to their names and
    /// the numbers printed on the board.
    struct Pins,
    target_device: target_device,

    /// LEDS
    pin led_fail = a22,
    pin led_pass = a23,

    /// Start switch
    pin start_sw = a18,

    /// Enable pins
    pin vbus_en = a16,
    pin vbat_en = a17,

    /// USB D+/- pins
    pin usb_dm = a24,
    pin usb_dp = a25,

    /// DUT related
    pin en = a15,
    pin d0 = a13,
    pin d1 = a12,
    pin d2 = b15,
    pin d3 = a11,
    pin d4 = a10,
    pin d5 = a9,
    pin d6 = a8,
    pin d7 = a7,
    pin d8 = a6,
    pin d9 = b13,
    pin d10 = b12,
    pin d11 = b11,
    pin d12 = b10,
    pin a5 = b8,
    pin a4 = b7,
    pin a3 = b6,
    pin a2 = b5,
    pin a1 = b4,
    pin a0 = b3,
    pin md = b2,
    pin rst = b0,
    pin ps_en = b14,

);

impl Pins {
    /// Split the device pins into subsets
    pub fn split(self) -> Sets {
        let analog = Analog {
            a0: self.a0,
            a1: self.a1,
            a2: self.a2,
            a3: self.a3,
            a4: self.a4,
            a5: self.a5,
        };

        let digital = Digital {
            d0: self.d0,
            d1: self.d1,
            d2: self.d2,
            d3: self.d3,
            d4: self.d4,
            d5: self.d5,
            d6: self.d6,
            d7: self.d7,
            d8: self.d8,
            d9: self.d9,
            d10: self.d10,
            d11: self.d11,
            d12: self.d12,
        };

        let leds = Leds {
            fail: self.led_fail,
            pass: self.led_pass,
        };

        let pwr = PowerControl {
            vbus: self.vbus_en,
            vbat: self.vbat_en,
        };

        let ctrl = DutControl {
            rst: self.rst,
            en: self.en,
        };

        let usb = USB {
            dm: self.usb_dm,
            dp: self.usb_dp,
        };

        Sets {
            port: self.port,
            leds,
            analog,
            digital,
            pwr,
            ctrl,
            usb,
        }
    }
}

/// Sets of pins split apart by category
pub struct Sets {
    /// Port
    pub port: Port,

    /// Pass/fail leds
    pub leds: Leds,

    /// Analog pins.
    pub analog: Analog,

    /// Digital pins.
    pub digital: Digital,

    /// DUT power control
    pub pwr: PowerControl,

    /// DUT control
    pub ctrl: DutControl,

    /// USB pins
    pub usb: USB,
}

/// Controlling power on the DUT
pub struct PowerControl {
    pub vbus: Pa16<Input<Floating>>,
    pub vbat: Pa17<Input<Floating>>,
}

/// Enable/Reset control
pub struct DutControl {
    pub rst: Pb0<Input<Floating>>,
    pub en: Pa15<Input<Floating>>,
}

/// Led pins
pub struct Leds {
    pub pass: Pa23<Input<Floating>>,
    pub fail: Pa22<Input<Floating>>,
}

/// Analog pins
pub struct Analog {
    pub a0: Pb3<Input<Floating>>,
    pub a1: Pb4<Input<Floating>>,
    pub a2: Pb5<Input<Floating>>,
    pub a3: Pb6<Input<Floating>>,
    pub a4: Pb7<Input<Floating>>,
    pub a5: Pb8<Input<Floating>>,
}

/// Digital pins
pub struct Digital {
    pub d0: Pa13<Input<Floating>>,
    pub d1: Pa12<Input<Floating>>,
    pub d2: Pb15<Input<Floating>>,
    pub d3: Pa11<Input<Floating>>,
    pub d4: Pa10<Input<Floating>>,
    pub d5: Pa9<Input<Floating>>,
    pub d6: Pa8<Input<Floating>>,
    pub d7: Pa7<Input<Floating>>,
    pub d8: Pa6<Input<Floating>>,
    pub d9: Pb13<Input<Floating>>,
    pub d10: Pb12<Input<Floating>>,
    pub d11: Pb11<Input<Floating>>,
    pub d12: Pb10<Input<Floating>>,
}

/// USB pins
pub struct USB {
    pub dm: Pa24<Input<Floating>>,
    pub dp: Pa25<Input<Floating>>,
}

#[cfg(feature = "usb")]
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    pm: &mut pac::PM,
    dm: gpio::Pa24<Input<Floating>>,
    dp: gpio::Pa25<Input<Floating>>,
    port: &mut Port,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let usb_clock = &clocks.usb(&gclk0).unwrap();

    UsbBusAllocator::new(UsbBus::new(
        usb_clock,
        pm,
        dm.into_function(port),
        dp.into_function(port),
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
//     rx: gpio::Pb14<Input<Floating>>,
//     tx: gpio::Pb13<Input<Floating>>,
//     port: &mut Port,
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
//         (rx.into_pad(port), tx.into_pad(port)),
//     )
// }
