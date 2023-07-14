#![no_std]
#![no_main]

mod keys;
mod pins;

use defmt::{info, debug};
use keyberon::key_code::KbHidReport;
use rp_pico::{self as bsp, entry, hal, hal::Clock};
use usb_device::{
    bus::UsbBusAllocator,
    class_prelude::UsbClass,
    prelude::{UsbDeviceBuilder, UsbDeviceState::Configured, UsbVidPid},
};

use defmt_rtt as _;
use panic_probe as _;

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

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut pins = pins::setup(pico_pins);

    let mut delay = {
        let core = bsp::pac::CorePeripherals::take().unwrap();
        cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz())
    };

    let mut usb_class = keyberon::new_class(&usb_bus, ());

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x0001))
        .manufacturer("Aaron's Crusty Keebs")
        .product("TX42 Keeb")
        .serial_number("0101010")
        .build();

    let mut keymap = keys::KeyMap::new();
    let mut count = 0;
    let mut configured = false;

    loop {
        delay.delay_ms(1);

        if usb_dev.poll(&mut [&mut usb_class]) {
            usb_class.poll();
        }
        if usb_dev.state() != Configured {
            continue;
        }
        if !configured {
            configured = true;
            info!("usb device configured");
        }

        // scan every 10ms
        count = (count + 1) % 10;
        if count == 0 {
            let keystates = pins.poll();
            let keycodes = keymap.mapkeys(keystates);
            let report: KbHidReport = keycodes.into_iter().collect();
            if usb_class.device_mut().set_keyboard_report(report.clone()) {
                debug!("new keycodes: {}", keycodes.map(|k| k as u8));
                while let Ok(0) = usb_class.write(report.as_bytes()) {}
            }
            if keycodes[0] as u8 != 0 {
                pins.led(true);
            } else {
                pins.led(false);
            }
        }
    }
}
