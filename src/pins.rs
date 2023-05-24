use embedded_hal::digital::v2::InputPin;
use hal::gpio::FunctionI2C;
use rp_pico::hal::gpio::{
    bank0::*, AnyPin, Disabled, Function, Pin as GpioPin, PullDown, PullUpInput,
};
use rp_pico::hal::i2c::{SclPin, SdaPin};
use rp_pico::pac::I2C1;
use rp_pico::{hal, Pins as GpioPins, GP26_I2_C1_SDA_MODE};

pub const NKEY: usize = 6;

pub(crate) fn setup(pins: GpioPins) -> (DisplayPins, KeyPins) {
    let key_pins = KeyPins {
        // TODO: make configurable via text file
        pins: [
            // row1
            P3(pins.gpio3.into_pull_up_input()),
            P9(pins.gpio9.into_pull_up_input()),
            P13(pins.gpio13.into_pull_up_input()),
            // row2
            P2(pins.gpio2.into_pull_up_input()),
            P8(pins.gpio8.into_pull_up_input()),
            P12(pins.gpio12.into_pull_up_input()),
        ],
    };

    let display_pins = DisplayPins(
        pins.gpio26.into_mode::<hal::gpio::FunctionI2C>(),
        pins.gpio27.into_mode::<hal::gpio::FunctionI2C>(),
    );
    (display_pins, key_pins)
}

pub struct KeyPins {
    pins: [PullUpPin; NKEY],
}

pub struct DisplayPins(GpioPin<Gpio26, FunctionI2C>, GpioPin<Gpio27, FunctionI2C>);

impl DisplayPins {
    pub fn split(self) -> (GpioPin<Gpio26, FunctionI2C>, GpioPin<Gpio27, FunctionI2C>) {
        (self.0, self.1)
    }
}

impl KeyPins {
    pub fn poll(&self) -> [bool; NKEY] {
        // TODO: implement debounce
        let mut keystates = [false; NKEY];
        for (i, v) in self.pins.iter().map(|p| p.is_low()).enumerate() {
            keystates[i] = v;
        }
        keystates
    }
}

#[allow(unused)]
enum PullUpPin {
    P0(GpioPin<Gpio0, PullUpInput>),
    P1(GpioPin<Gpio1, PullUpInput>),
    P2(GpioPin<Gpio2, PullUpInput>),
    P3(GpioPin<Gpio3, PullUpInput>),
    P4(GpioPin<Gpio4, PullUpInput>),
    P5(GpioPin<Gpio5, PullUpInput>),
    P6(GpioPin<Gpio6, PullUpInput>),
    P7(GpioPin<Gpio7, PullUpInput>),
    P8(GpioPin<Gpio8, PullUpInput>),
    P9(GpioPin<Gpio9, PullUpInput>),
    P10(GpioPin<Gpio10, PullUpInput>),
    P11(GpioPin<Gpio11, PullUpInput>),
    P12(GpioPin<Gpio12, PullUpInput>),
    P13(GpioPin<Gpio13, PullUpInput>),
    P14(GpioPin<Gpio14, PullUpInput>),
    P15(GpioPin<Gpio15, PullUpInput>),
    P16(GpioPin<Gpio16, PullUpInput>),
    P17(GpioPin<Gpio17, PullUpInput>),
    P18(GpioPin<Gpio18, PullUpInput>),
    P19(GpioPin<Gpio19, PullUpInput>),
    P20(GpioPin<Gpio20, PullUpInput>),
    P21(GpioPin<Gpio21, PullUpInput>),
    P22(GpioPin<Gpio22, PullUpInput>),
    P26(GpioPin<Gpio26, PullUpInput>),
    P27(GpioPin<Gpio27, PullUpInput>),
    P28(GpioPin<Gpio28, PullUpInput>),
}
use ssd1306::prelude::I2CInterface;
use PullUpPin::*;

impl PullUpPin {
    fn is_low(&self) -> bool {
        match self {
            P0(p) => p.is_low().unwrap(),
            P1(p) => p.is_low().unwrap(),
            P2(p) => p.is_low().unwrap(),
            P3(p) => p.is_low().unwrap(),
            P4(p) => p.is_low().unwrap(),
            P5(p) => p.is_low().unwrap(),
            P6(p) => p.is_low().unwrap(),
            P7(p) => p.is_low().unwrap(),
            P8(p) => p.is_low().unwrap(),
            P9(p) => p.is_low().unwrap(),
            P10(p) => p.is_low().unwrap(),
            P11(p) => p.is_low().unwrap(),
            P12(p) => p.is_low().unwrap(),
            P13(p) => p.is_low().unwrap(),
            P14(p) => p.is_low().unwrap(),
            P15(p) => p.is_low().unwrap(),
            P16(p) => p.is_low().unwrap(),
            P17(p) => p.is_low().unwrap(),
            P18(p) => p.is_low().unwrap(),
            P19(p) => p.is_low().unwrap(),
            P20(p) => p.is_low().unwrap(),
            P21(p) => p.is_low().unwrap(),
            P22(p) => p.is_low().unwrap(),
            P26(p) => p.is_low().unwrap(),
            P27(p) => p.is_low().unwrap(),
            P28(p) => p.is_low().unwrap(),
        }
    }
}
