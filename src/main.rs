//! board, with the USB driver running in the main thread.
//!
//! It generates movement reports which will twitch the cursor up and down by a
//! few pixels, several times a second.
//!
//! See the `Cargo.toml` file for Copyright and license details.
//!
//! This is a port of
//! https://github.com/atsamd-rs/atsamd/blob/master/boards/itsybitsy_m0/examples/twitching_usb_mouse.rs

#![allow(unused)]
#![no_std]
#![no_main]

mod bytestring;
mod display;
mod icons;
mod keys;
mod pins;

use core::{fmt::Write, ops::Deref};

use bsp::{hal::Clock, pac};
use defmt::info;
use display::{DrawIcon, DrawTextParts};
use embedded_graphics::{
    mono_font::{
        ascii::{FONT_5X8, FONT_7X13, FONT_9X15},
        MonoTextStyleBuilder,
    },
    pixelcolor::BinaryColor,
    prelude::Point,
    text::{Baseline, Text},
    Drawable,
};
use hid::page::Keyboard;
use rp_pico::{self as bsp, entry, hal};

use defmt_rtt as _;
use panic_probe as _;
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_human_interface_device::{self as hid, prelude::*};

// Time handling traits:
use fugit::{ExtU32, RateExtU32};

macro_rules! format {
    ($($arg: tt)*) => {
        {
            let mut text_buffer = bytestring::ByteStringWriter::default();
            write!(text_buffer, $($arg)*);
            text_buffer
        }
    };
}

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
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

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let (display_pins, key_pins) = pins::setup(pico_pins);
    let (sda, scl) = display_pins.split();
    let i2c = hal::I2C::i2c1(
        pac.I2C1,
        sda,
        scl,
        1000.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    let core = pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    delay.delay_ms(10); // wait for display to start
    let mut disp = display::setup(i2c);

    let mut keyboard = UsbHidClassBuilder::new()
        .add_device(hid::device::keyboard::NKROBootKeyboardConfig::default())
        .build(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x0001))
        .manufacturer("Crusty Keebs")
        .product("TX Keeb")
        .serial_number("TEST")
        .build();

    let mut old_keycodes = [Keyboard::NoEventIndicated; keys::MAX_KEYCODES];
    let mut iconx = 0;
    let mut icony = 0;

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_7X13)
        .text_color(BinaryColor::On)
        .build();

    let mut cnt = 0;
    let mut icon_idx = 0;
    let mut icon_scale = 1;
    loop {
        cnt += 1;
        // TODO: replace with proper delay
        delay.delay_ms(10);

        if cnt % 12 == 0 {
            iconx = (iconx + 1) % (128 + 16);
            if iconx == 0 {
                icon_idx = (icon_idx + 1) % icons::ICONS.len();
            }

            if cnt % 50 == 0 {
                icon_scale = (icon_scale % 2) + 1;
            }

            disp.clear();

            let ix = iconx - 16;
            let iy = icony + 16;
            disp.draw_icon_with_scale(icons::ICONS[icon_idx], ix, iy, icon_scale);
            Text::with_baseline(
                &format!("author: {}", icons::ICONS[icon_idx].author),
                Point::new(0, 35),
                text_style,
                Baseline::Top,
            )
            .draw(&mut disp)
            .unwrap();

            Text::with_baseline(
                &format!("name: {}", icons::ICONS[icon_idx].name),
                Point::new(0, 50),
                text_style,
                Baseline::Top,
            )
            .draw(&mut disp)
            .unwrap();

            Text::with_baseline(
                &format!("[{}/{}] {icon_scale}00%", icon_idx + 1, icons::ICONS.len()),
                Point::new(0, 0),
                text_style,
                Baseline::Top,
            )
            .draw(&mut disp)
            .unwrap();

            disp.flush();
        }

        let keystates = key_pins.poll();
        let keycodes = keys::mapkeys(keystates);
        if old_keycodes != keycodes {
            info!("new keycodes: {}", keycodes.map(|k| k as u8));
            old_keycodes = keycodes;
        }

        let keycodes = [];
        match keyboard.device().write_report(keycodes) {
            Err(UsbHidError::WouldBlock) => {}
            Err(UsbHidError::Duplicate) => {}
            Ok(_) => {}
            Err(e) => {
                core::panic!("Failed to write keyboard report: {:?}", e)
            }
        }

        match keyboard.tick() {
            Err(UsbHidError::WouldBlock) => {}
            Ok(_) => {}
            Err(e) => {
                core::panic!("Failed to process keyboard tick: {:?}", e)
            }
        };

        if usb_dev.poll(&mut [&mut keyboard]) {
            match keyboard.device().read_report() {
                Err(UsbError::WouldBlock) => {
                    //do nothing
                }
                Err(e) => {
                    core::panic!("Failed to read keyboard report: {:?}", e)
                }
                Ok(_leds) => {}
            }
        }
    }
}
