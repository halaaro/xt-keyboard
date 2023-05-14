use usbd_human_interface_device::page::Keyboard;

use crate::pins::NKEY;

pub const MAX_KEYCODES: usize = 10;
pub const KEYMAP: [&[Keyboard]; NKEY] = [
    // &[Keyboard::LeftGUI, Keyboard::LeftShift, Keyboard::A], // powertoys mute
    // &[Keyboard::UpArrow],
    // &[Keyboard::PrintScreen],
    // &[Keyboard::LeftArrow],
    // &[Keyboard::DownArrow],
    // &[Keyboard::RightArrow],
    &[Keyboard::Escape],
    &[Keyboard::Q],
    &[Keyboard::W],
    &[Keyboard::E],
    &[Keyboard::R],
    &[Keyboard::T],
];

pub fn mapkeys(keystates: [bool; NKEY]) -> [Keyboard; MAX_KEYCODES] {
    let keycodes = keystates.iter().zip(KEYMAP);
    let mut keys = [Keyboard::NoEventIndicated; MAX_KEYCODES];
    // TODO: handle conflicting keycodes
    for (i, keycode) in keycodes
        .filter_map(|(active, keycodes)| if *active { Some(keycodes) } else { None })
        .flatten()
        .enumerate()
    {
        keys[i] = *keycode;
    }
    keys
}
