use defmt::{dbg, debug, info};
use keyberon::key_code::KeyCode::{self, *};

use crate::pins::NKEY;

type Layer = [&'static [KeyCode]; NKEY];
const MAX_TAP_COUNT: usize = 15;
pub const MAX_KEYCODES: usize = 10;

pub const LAYER1: Layer = [
    &[Tab], &[Q], &[W], &[E], &[R], &[T],
    &[Escape], &[A], &[S], &[D], &[F], &[G],
    &[LShift], &[Z], &[X], &[C], &[V], &[B],
    &[Space], &[LAlt], &[LCtrl],
];

pub const LAYER2: Layer = [
    &[Grave], &[Kb1], &[Kb2], &[Kb3], &[Kb4], &[Kb5],
    &[Escape], &[LShift, Kb1], &[LShift, Kb2], &[LShift, Kb3], &[LShift, Kb4], &[LShift, Kb5],
    &[LShift], &[], &[], &[], &[], &[],
    &[LGui], &[LAlt], &[LCtrl],
];

pub const LAYER3: Layer = [
    &[F1], &[F2], &[F3], &[F4], &[F4], &[F5],
    &[Escape], &[Q], &[W], &[E], &[R], &[T],
    &[LShift], &[A], &[S], &[D], &[F], &[G],
    &[Space], &[LAlt], &[LCtrl],
];

pub struct KeyMap {
    active_layer: &'static Layer,
    old_keys: [KeyCode; MAX_KEYCODES],
    tapstart: Option<KeyCode>,
    tap_count: usize,
}

impl KeyMap {
    pub fn new() -> Self {
        KeyMap {
            active_layer: &LAYER1,
            old_keys: [No; MAX_KEYCODES],
            tapstart: None,
            tap_count: 0,
        }
    }

    pub fn mapkeys(&mut self, keystates: [bool; NKEY]) -> [KeyCode; MAX_KEYCODES] {
        let keycodes = keystates.iter().zip(self.active_layer);
        let mut keys = [No; MAX_KEYCODES];
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
            let was_no_keys_pressed = self.old_keys[0] == No;
            let has_key_pressed_now = keys[0] != No;
            let key_count = keys.iter().filter(|k| **k != No).count();

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
                    (&LAYER1, LAlt) => {
                        self.active_layer = &LAYER2;
                        info!("SWITCH -> layer 2");
                    }
                    (&LAYER2, LAlt) => {
                        self.active_layer = &LAYER3;
                        info!("SWITCH -> layer 3");
                    }
                    (active, LCtrl) if active != &LAYER1 => {
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

        if let Some(LAlt) = self.tapstart {
            keys[0] = No; // ignore quick alt taps
        }

        keys
    }
}
