use embedded_hal::digital::v2::{InputPin, OutputPin, PinState};
use rp_pico::hal::gpio::{bank0::*, Output, Pin as GpioPin, PullUpInput, PushPull};
use rp_pico::Pins as GpioPins;

pub const NKEY: usize = 21;

pub(crate) fn setup(pins: GpioPins) -> Pins {
    Pins {
        led: pins.led.into_push_pull_output(),
        pins: [
            // top row
            P0(pins.gpio0.into_pull_up_input()),
            P1(pins.gpio1.into_pull_up_input()),
            P2(pins.gpio2.into_pull_up_input()),
            P3(pins.gpio3.into_pull_up_input()),
            P4(pins.gpio4.into_pull_up_input()),
            P5(pins.gpio5.into_pull_up_input()),
            // middle row
            P6(pins.gpio6.into_pull_up_input()),
            P7(pins.gpio7.into_pull_up_input()),
            P8(pins.gpio8.into_pull_up_input()),
            P9(pins.gpio9.into_pull_up_input()),
            P10(pins.gpio10.into_pull_up_input()),
            P11(pins.gpio11.into_pull_up_input()),
            // bottom row
            P12(pins.gpio12.into_pull_up_input()),
            P13(pins.gpio13.into_pull_up_input()),
            P14(pins.gpio14.into_pull_up_input()),
            P15(pins.gpio15.into_pull_up_input()),
            P16(pins.gpio16.into_pull_up_input()),
            P17(pins.gpio17.into_pull_up_input()),
            // thumb cluster
            P18(pins.gpio18.into_pull_up_input()),
            P19(pins.gpio19.into_pull_up_input()),
            P22(pins.gpio22.into_pull_up_input()),
        ],
    }
}

pub struct Pins {
    pins: [PullUpPin; NKEY],
    led: GpioPin<Gpio25, Output<PushPull>>,
}

impl Pins {
    pub fn poll(&self) -> [bool; NKEY] {
        let mut keystates = [false; NKEY];
        for (i, v) in self.pins.iter().map(|p| p.is_low()).enumerate() {
            keystates[i] = v;
        }
        keystates
    }

    pub fn led(&mut self, on: bool) {
        let _ = self
            .led
            .set_state(if on { PinState::High } else { PinState::Low });
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
