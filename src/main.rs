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

mod display;
mod keys;
mod pins;

use bsp::{hal::Clock, pac};
use defmt::info;
use display::DrawIcon;
use embedded_graphics::{text::{Text, Baseline}, prelude::Point, mono_font::{MonoTextStyleBuilder, ascii::FONT_9X15}, pixelcolor::BinaryColor, Drawable};
use hid::page::Keyboard;
use rp_pico::{self as bsp, entry, hal};

use defmt_rtt as _;
use panic_probe as _;
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_human_interface_device::{self as hid, prelude::*};

// Time handling traits:
use fugit::{ExtU32, RateExtU32};

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
        400.kHz(),
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

    let core = pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    delay.delay_ms(100); // wait for display to start
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


    let istr = "0123456789";

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_9X15)
        .text_color(BinaryColor::On)
        .build();

    let mut cnt = 0;
    loop {
        cnt += 1;
        // TODO: replace with proper delay
        delay.delay_ms(10);

        // TODO: only call once sufficient time has passed
        iconx = (iconx + 1) % (128 + 16);
        icony = if iconx == 0 { (icony + 10) % 64 } else { icony };
        disp.clear();

        let ix = iconx - 16;
        let iy = icony;
        disp.draw_icon(ix, icony);


    let ix_ones = (ix % 10).unsigned_abs() as usize;
    let ix_tens = ((ix / 10) % 10).unsigned_abs() as usize;
    let ix_hund = ((ix / 100) % 10).unsigned_abs() as usize;

        // disp.draw_text(["x: ", if ix < 0 { "-" } else { " "},
// &istr[ix_hund..ix_hund+1],
// &istr[ix_tens..ix_tens+1],
// &istr[ix_ones..ix_ones+1],
        // ], Point::new(0,0), text_style, Baseline::Top);

    Text::with_baseline("x: ", Point::new(0, 0), text_style, Baseline::Top)
        .draw(&mut disp)
        .unwrap();
    Text::with_baseline(if ix < 0 { "-" } else { " "}, Point::new(40, 0), text_style, Baseline::Top)
        .draw(&mut disp)
        .unwrap();
    Text::with_baseline(&istr[ix_ones..ix_ones+1], Point::new(70, 0), text_style, Baseline::Top)
        .draw(&mut disp)
        .unwrap();

    Text::with_baseline(&istr[ix_tens..ix_tens+1], Point::new(60, 0), text_style, Baseline::Top)
        .draw(&mut disp)
        .unwrap();

    Text::with_baseline(&istr[ix_hund..ix_hund+1], Point::new(50, 0), text_style, Baseline::Top)
        .draw(&mut disp)
        .unwrap();

    let iy_ones = (iy % 10).unsigned_abs() as usize;
    let iy_tens = ((iy / 10) % 10).unsigned_abs() as usize;
    let iy_hund = ((iy / 100) % 10).unsigned_abs() as usize;

        // disp.draw_text(["x: ", if iy < 0 { "-" } else { " "},
// &istr[iy_hund..iy_hund+1],
// &istr[iy_tens..iy_tens+1],
// &istr[iy_ones..iy_ones+1],
        // ], Point::new(0,0), text_style, Baseline::Top);

    Text::with_baseline("y: ", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut disp)
        .unwrap();
    Text::with_baseline(if iy < 0 { "-" } else { " "}, Point::new(40, 16), text_style, Baseline::Top)
        .draw(&mut disp)
        .unwrap();
    Text::with_baseline(&istr[iy_ones..iy_ones+1], Point::new(70, 16), text_style, Baseline::Top)
        .draw(&mut disp)
        .unwrap();

    Text::with_baseline(&istr[iy_tens..iy_tens+1], Point::new(60, 16), text_style, Baseline::Top)
        .draw(&mut disp)
        .unwrap();

    Text::with_baseline(&istr[iy_hund..iy_hund+1], Point::new(50, 16), text_style, Baseline::Top)
        .draw(&mut disp)
        .unwrap();

        disp.flush();

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
