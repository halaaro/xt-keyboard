use embedded_graphics::{
    mono_font::{
        ascii::{FONT_6X10, FONT_9X15},
        MonoTextStyle, MonoTextStyleBuilder,
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::{
        renderer::{CharacterStyle, TextRenderer},
        Baseline, Text,
    },
};
use embedded_hal::blocking::i2c::Write;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

use crate::icons::Icon;

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

pub trait DrawTextParts<S, P> {
    fn draw_text_parts(
        &mut self,
        parts: P,
        position: Point,
        character_style: S,
        baseline: Baseline,
    );
}

impl<'a, I, S, P> DrawTextParts<S, P>
    for Ssd1306<I2CInterface<I>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>
where
    I: Write,
    S: TextRenderer<Color = BinaryColor> + Copy,
    P: IntoIterator<Item = &'a str>,
{
    fn draw_text_parts(
        &mut self,
        parts: P,
        position: Point,
        character_style: S,
        baseline: Baseline,
    ) {
        let mut position = position;

        for part in parts {
            Text::with_baseline(part, position, character_style, baseline)
                .draw(self)
                .unwrap();
            let metrics = character_style.measure_string(part, position, baseline);
            position = metrics.next_position
        }
    }
}

pub trait DrawIcon {
    fn draw_icon_with_scale(&mut self, icon: &Icon, xleft: i32, ytop: i32, scale: usize);
    fn draw_icon(&mut self, icon: &Icon, xleft: i32, ytop: i32) { 
        self.draw_icon_with_scale(icon, xleft, ytop, 1);
    }
}

impl<I> DrawIcon
    for Ssd1306<I2CInterface<I>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>
where
    I: Write,
{
    fn draw_icon_with_scale(&mut self, icon: &Icon, xleft: i32, ytop: i32, scale: usize) {
        for y in 0..icon.height()*scale {
            for x in 0..icon.width()*scale {
                let xpos = x as i32 + xleft;
                let ypos = y as i32 + ytop;
                if (xpos > 127 || ypos > 63 || xpos < 0 || ypos < 0) {
                    continue;
                }
                self.set_pixel(
                    xpos as u32,
                    ypos as u32,
                    icon.pixels[y/scale].as_bytes()[x/scale] != b' ',
                );
            }
        }
    }
}
