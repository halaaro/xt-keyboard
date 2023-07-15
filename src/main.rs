#![no_std]
#![no_main]

mod keys;
mod pins;

use bsp::hal::usb::UsbBus;
use cortex_m::peripheral::NVIC;

use defmt::{debug, info};
use keyberon::{hid::HidClass, key_code::KbHidReport, keyboard::Keyboard};
use rp_pico::{self as bsp, entry, hal, hal::pac::interrupt, hal::Clock};
use usb_device::{
    bus::UsbBusAllocator,
    class_prelude::UsbClass,
    prelude::{UsbDevice, UsbDeviceBuilder, UsbDeviceState::Configured, UsbVidPid},
};

use defmt_rtt as _;
use panic_probe as _;

static mut USB_BUS: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_DEV: Option<UsbDevice<UsbBus>> = None;
static mut USB_CLASS: Option<HidClass<UsbBus, Keyboard<()>>> = None;

#[entry]
fn main() -> ! {
    let mut pac = bsp::pac::Peripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);
    let pico_pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let usb_bus = {
        let bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
            pac.USBCTRL_REGS,
            pac.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut pac.RESETS,
        ));
        unsafe {
            USB_BUS = Some(bus);
            USB_BUS.as_ref().unwrap()
        }
    };

    let mut pins = pins::setup(pico_pins);

    let mut delay = {
        let core = bsp::pac::CorePeripherals::take().unwrap();
        cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz())
    };

    let usb_class = keyberon::new_class(usb_bus, ());
    let usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x1209, 0x0001))
        .manufacturer("Aaron's Crusty Keebs")
        .product("TX42 Keeb")
        .serial_number("0101010")
        .build();

    unsafe {
        USB_CLASS = Some(usb_class);
        USB_DEV = Some(usb_dev);
        NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    }

    let mut keymap = keys::KeyMap::new();
    let mut count = 0;
    let mut configured = false;

    loop {
        delay.delay_ms(1);

        // scan every 10ms
        count = (count + 1) % 10;
        if count == 0 {
            let keystates = pins.poll();
            let keycodes = keymap.mapkeys(keystates);
            let report: KbHidReport = keycodes.into_iter().collect();
            critical_section::with(|_| {
                let usb_dev = unsafe { USB_DEV.as_ref().unwrap() };
                if usb_dev.state() != Configured {
                    return;
                }
                if !configured {
                    configured = true;
                    info!("usb device configured");
                }
                let usb_class = unsafe { USB_CLASS.as_mut().unwrap() };
                if usb_class.device_mut().set_keyboard_report(report.clone()) {
                    debug!("new keycodes: {}", keycodes.map(|k| k as u8));
                    while let Ok(0) = usb_class.write(report.as_bytes()) {}
                }
            });
            if keycodes[0] as u8 != 0 {
                pins.led(true);
            } else {
                pins.led(false);
            }
        }
    }
}

#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    let usb_dev = USB_DEV.as_mut().unwrap();
    let usb_class = USB_CLASS.as_mut().unwrap();

    if usb_dev.poll(&mut [usb_class]) {
        usb_class.poll();
    }
}
