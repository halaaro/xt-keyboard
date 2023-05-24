use embedded_graphics::{
    mono_font::{
        ascii::{FONT_6X10, FONT_9X15},
        MonoTextStyleBuilder,
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use embedded_hal::blocking::i2c::Write;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

pub fn setup<I>(
    i2c: I,
) -> Ssd1306<I2CInterface<I>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>
where
    I: Write,
{
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();
    display

    // let text_style = MonoTextStyleBuilder::new()
    //     .font(&FONT_9X15)
    //     .text_color(BinaryColor::On)
    //     .build();

    // display.clear();
    // let icon = [
    //     b" XXX   XXX ",
    //     b"XXXXX XXXXX",
    //     b" XXXXXXXXX ",
    //     b"   XXXXX   ",
    //     b"     X     ",
    // ];
    // for y in 0..icon.len() {
    //     for x in 0..icon[0].len() {
    //         display.set_pixel(x as u32, 20 + y as u32, icon[y][x] != b' ');
    //     }
    // }
    // display.set_pixel(127,0,true);
    // display.set_pixel(127,63,true);
    // display.set_pixel(0,63,true);

    // Text::with_baseline("Emma: SAVES $", Point::new(4, 0), text_style, Baseline::Top)
    //     .draw(&mut display)
    //     .unwrap();

    // Text::with_baseline("Anna: ALL GONE!", Point::new(4, 16), text_style, Baseline::Top)
    //     .draw(&mut display)
    //     .unwrap();

    // display.flush().unwrap();
}

// pub trait DrawTextIter {
//     type Iter;
//     fn draw_text_iter(&mut self, text: Self::Iter);
// }

// impl<'a, I> DrawTextIter
//     for Ssd1306<I2CInterface<I>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>
// where
//     I: Write,
// {
//     type Iter = &'a dyn FromIterator<&'static str>;
//     fn draw_text_iter(&mut self, text: Self::Iter) {}
// }

pub trait DrawIcon {
    fn draw_icon(&mut self, xleft: i32, ytop: i32);
}

impl<I> DrawIcon
    for Ssd1306<I2CInterface<I>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>
where
    I: Write,
{
    fn draw_icon(&mut self, xleft: i32, ytop: i32) {
        let icon = [
            b" XXX   XXX ",
            b"XXXXX XXXXX",
            b" XXXXXXXXX ",
            b"   XXXXX   ",
            b"     X     ",
        ];
        for y in 0..icon.len() {
            for x in 0..icon[0].len() {
                let xpos = x as i32 + xleft;
                let ypos = y as i32 + ytop;
                if (xpos > 127 || ypos > 63 || xpos < 0 || ypos < 0) {
                    continue;
                }
                self.set_pixel(xpos as u32, ypos as u32, icon[y][x] != b' ');
            }
        }
    }
}
