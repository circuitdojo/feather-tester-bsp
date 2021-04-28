#![no_std]

extern crate atsamd_hal as hal;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use hal::common::*;
pub use hal::target_device as pac;

// Shared with UART/USB
#[cfg(any(feature = "uart", feature = "usb"))]
use hal::clock::GenericClockController;
#[cfg(any(feature = "uart", feature = "usb"))]
use hal::gpio::v2::AlternateC;

// UART related
#[cfg(feature = "uart")]
use hal::gpio::v2::{PB12, PB13};
#[cfg(feature = "uart")]
use hal::sercom::UART4;
#[cfg(feature = "uart")]
use hal::time::Hertz;

use hal::bsp_pins;

#[cfg(feature = "usb")]
use hal::gpio::v2::{AnyPin, PA24, PA25};
#[cfg(feature = "usb")]
use hal::usb::usb_device::bus::UsbBusAllocator;
#[cfg(feature = "usb")]
pub use hal::usb::UsbBus;

bsp_pins!(

    #[cfg(feature = "unproven")]
    PA13 {
        name: d0
    }

    #[cfg(feature = "unproven")]
    PA12 {
        name: d1
    }

    #[cfg(feature = "unproven")]
    PB15 {
        name: d2
    }

    #[cfg(feature = "unproven")]
    PA11 {
        name: d3
    }

    #[cfg(feature = "unproven")]
    PA10 {
        name: d4
    }

    #[cfg(feature = "unproven")]
    PA09  {
        name: d5
    }

    #[cfg(feature = "unproven")]
    PA08 {
        name: d6
    }

    #[cfg(feature = "unproven")]
    PA06 {
        name: d8
    }

    #[cfg(feature = "unproven")]
    PB12 {
        name: d10
        aliases: {
            #[cfg(feature = "uart")]
            AlternateC: Tx
        }
    }

    #[cfg(feature = "unproven")]
    PB11 {
        name: d11
    }

    #[cfg(feature = "unproven")]
    PB10 {
        name: d12
    }

    #[cfg(feature = "unproven")]
    PB09 {
        name: d13
    }

    #[cfg(feature = "unproven")]
    PB08 {
        name: a5
    }

    #[cfg(feature = "unproven")]
    PB07 {
        name: a4
    }

    #[cfg(feature = "unproven")]
    PB06 {
        name: a3
    }

    #[cfg(feature = "unproven")]
    PB05 {
        name: a2
    }

    #[cfg(feature = "unproven")]
    PB04 {
        name: a1
    }

    #[cfg(feature = "unproven")]
    PB03 {
        name: a0
        aliases: { PullDownInput: SimCheck }
    }

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
    PB01 {
        name: meas_3v3,
        aliases: { AlternateB: Meas3V3 }
    }

    #[cfg(feature = "unproven")]
    PB02 {
        name: md
    }

    #[cfg(feature = "unproven")]
    PB00 {
        name: rst
    }

    #[cfg(feature = "unproven")]
    PB13 {
        name: d9,
        aliases: {
            #[cfg(feature = "uart")]
            AlternateC: Rx
        }
    }

    #[cfg(feature = "unproven")]
    PA14 {
        name: dut_en,
        aliases: { PushPullOutput: DutEn }
    }

    #[cfg(feature = "unproven")]
    PA16 {
        name: vbus_en,
        aliases: { PushPullOutput: VbusEn }
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
    dm: impl AnyPin<Id = PA24>,
    dp: impl AnyPin<Id = PA25>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let usb_clock = &clocks.usb(&gclk0).unwrap();

    // Then pop those into UsbBus
    UsbBusAllocator::new(UsbBus::new(usb_clock, pm, dm, dp, usb))
}

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
#[cfg(feature = "uart")]
pub fn uart<F: Into<Hertz>>(
    clocks: &mut GenericClockController,
    baud: F,
    sercom4: pac::SERCOM4,
    pm: &mut pac::PM,
    rx: gpio::v2::Pin<PB13, AlternateC>,
    tx: gpio::v2::Pin<PB12, AlternateC>,
) -> UART4<
    hal::sercom::Sercom4Pad1<gpio::Pb13<gpio::PfC>>,
    hal::sercom::Sercom4Pad0<gpio::Pb12<gpio::PfC>>,
    (),
    (),
> {
    let gclk0 = clocks.gclk0();

    UART4::new(
        &clocks.sercom4_core(&gclk0).unwrap(),
        baud.into(),
        sercom4,
        pm,
        (rx.into(), tx.into()),
    )
}
