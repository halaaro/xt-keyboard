use usbd_human_interface_device::page::Keyboard;

use crate::pins::NKEY;
use Keyboard::*;

pub const MAX_KEYCODES: usize = 10;
pub const KEYMAP: [&[Keyboard]; NKEY] = [
    &[Tab], &[Q], &[W], &[E], &[R], &[T],
    &[Escape], &[A], &[S], &[D], &[F], &[G],
    &[LeftShift], &[Z], &[X], &[C], &[V], &[B],
            &[LeftControl], &[LeftAlt], &[Space]
];

pub fn mapkeys(keystates: [bool; NKEY]) -> [Keyboard; MAX_KEYCODES] {
    let keycodes = keystates.iter().zip(KEYMAP);
    let mut keys = [Keyboard::NoEventIndicated; MAX_KEYCODES];
    // TODO: handle conflicting keycodes
    for (i, &keycode) in keycodes
        .filter_map(|(&active, keycodes)| if active { Some(keycodes) } else { None })
        .flatten()
        .enumerate()
    {
        keys[i] = keycode;
    }
    keys
}
