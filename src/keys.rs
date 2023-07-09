use defmt::{dbg, debug, info};
use usbd_human_interface_device::page::Keyboard;

use crate::pins::NKEY;
use Keyboard::NoEventIndicated as NEI;
use Keyboard::*;

type Layer = [&'static [Keyboard]; NKEY];
const MAX_TAP_COUNT: usize = 15;
pub const MAX_KEYCODES: usize = 10;

pub const LAYER1: Layer = [
    &[Tab], &[Q], &[W], &[E], &[R], &[T],
    &[Escape], &[A], &[S], &[D], &[F], &[G],
    &[LeftShift], &[Z], &[X], &[C], &[V], &[B],
    &[LeftGUI], &[LeftAlt], &[LeftControl],
];

pub const LAYER2: Layer = [
    &[Grave], &[Keyboard1], &[Keyboard2], &[Keyboard3], &[Keyboard4], &[Keyboard5],
    &[Escape], &[LeftShift, Keyboard1], &[LeftShift, Keyboard2], &[LeftShift, Keyboard3], &[LeftShift, Keyboard4], &[LeftShift, Keyboard5],
    &[LeftShift], &[], &[], &[], &[], &[],
    &[LeftGUI], &[LeftAlt], &[LeftControl],
];

pub const LAYER3: Layer = [
    &[F1], &[F2], &[F3], &[F4], &[F4], &[F5],
    &[Escape], &[], &[], &[], &[], &[],
    &[LeftShift], &[], &[], &[], &[], &[],
    &[LeftGUI], &[LeftAlt], &[LeftControl],
];

pub struct KeyMap {
    active_layer: &'static Layer,
    old_keys: [Keyboard; MAX_KEYCODES],
    tapstart: Option<Keyboard>,
    tap_count: usize,
}

impl KeyMap {
    pub fn new() -> Self {
        KeyMap {
            active_layer: &LAYER1,
            old_keys: [NEI; MAX_KEYCODES],
            tapstart: None,
            tap_count: 0,
        }
    }

    pub fn mapkeys(&mut self, keystates: [bool; NKEY]) -> [Keyboard; MAX_KEYCODES] {
        let keycodes = keystates.iter().zip(self.active_layer);
        let mut keys = [NEI; MAX_KEYCODES];
        // TODO: handle conflicting keycodes
        for (i, keycode) in keycodes
            .filter_map(|(&active, &keycodes)| if active { Some(keycodes) } else { None })
            .flatten()
            .enumerate()
        {
            keys[i] = *keycode;
        }
        self.tap_count += 1;

        if self.old_keys != keys {
            let was_no_keys_pressed = self.old_keys[0] == NEI;
            let has_key_pressed_now = keys[0] != NEI;
            let key_count = keys.iter().filter(|k| **k != NEI).count();

            dbg!(was_no_keys_pressed);
            dbg!(has_key_pressed_now);
            dbg!(key_count);
            dbg!(self.tap_count);

            // key press check
            if was_no_keys_pressed && has_key_pressed_now {
                self.tapstart = Some(keys[0]);
                self.tap_count = 0;
                debug!("setting tap start: {}", keys[0] as usize);
            }

            // only handle single key taps
            if key_count > 1 {
                self.tapstart = None;
                debug!("multiple keys pressed, clearing tap start");
            }

            // single key release check
            if let (Some(tapkey), 0, 0..=MAX_TAP_COUNT) = (self.tapstart, key_count, self.tap_count)
            {
                match (self.active_layer, tapkey) {
                    (&LAYER1, LeftAlt) => {
                        self.active_layer = &LAYER2;
                        info!("SWITCH -> layer 2");
                    }
                    (&LAYER2, LeftAlt) => {
                        self.active_layer = &LAYER3;
                        info!("SWITCH -> layer 3");
                    }
                    (active, LeftControl) if active != &LAYER1 => {
                        self.active_layer = &LAYER1;
                        info!("SWITCH -> layer 1");
                    }
                    _ => {
                        debug!("ignoring tap");
                    }
                }

                debug!("tap end");
                self.tapstart = None;
            }
        }

        self.old_keys = keys;

        if let Some(LeftAlt) = self.tapstart {
            keys[0] = NEI; // ignore quick alt taps
        }

        keys
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn layer_switch() {
//     }

// }
