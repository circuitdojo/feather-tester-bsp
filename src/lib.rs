#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use hal::common::*;
pub use hal::samd21::*;
pub use hal::target_device as pac;

// use hal::sercom::v2::UART3;
// use hal::time::Hertz;
use hal::bsp_pins;

#[cfg(feature = "usb")]
use hal::clock::GenericClockController;
#[cfg(feature = "usb")]
use hal::gpio::v2::{Floating, Input, Pin, PA24, PA25};
#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

bsp_pins!(
    #[cfg(feature = "unproven")]
    PA23 {
        name: led_pass,
        aliases: { PushPullOutput: LedPass }
    }

    #[cfg(feature = "unproven")]
    PA22 {
        name: led_fail,
        aliases: { PushPullOutput: LedFail }
    }

    #[cfg(feature = "unproven")]
    PA17 {
        name: vbat_en,
        aliases: { PushPullOutput: VbatEn }
    }

    #[cfg(feature = "unproven")]
    PB14 {
        name: ps_en,
        aliases: { PushPullOutput: PsEn }
    }

    #[cfg(feature = "unproven")]
    PA24 {
        name: dm,
        aliases: {
            #[cfg(feature = "usb")]
            AlternateG: UsbDm
        }
    }

    #[cfg(feature = "unproven")]
    PA25 {
        name: dp,
        aliases: {
            #[cfg(feature = "usb")]
            AlternateG: UsbDp
        }
    }
);

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
