use enigo::*;
use rdev::{listen, Event, EventType, Key};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

struct PressedKeys {
    history_length: usize,
    keys: Vec<Key>,
}

#[derive(Debug)]
struct Combination {
    leader: Vec<Key>,
    key: Key,
    lowercase_char: &'static str,
    uppercase_char: &'static str,
}

impl Combination {
    fn get_char(&self, case: Case) -> &str {
        match case {
            Case::Lower => {
                return self.lowercase_char;
            }
            Case::Upper => {
                return self.uppercase_char;
            }
        }
    }
}

#[derive(Debug, EnumIter)]
enum Combinations {
    AcuteAccentE,
    GraveAccentE,
    CircumflexE,
    UmlautE,
    AcuteAccentU,
    GraveAccentU,
    CircumflexU,
    UmlautU,
}

#[derive(Debug)]
enum Case {
    Lower,
    Upper,
}

impl Combinations {
    fn get(&self) -> Combination {
        return match self {
            /* E */
            Self::AcuteAccentE => Combination {
                leader: vec![Key::Alt, Key::KeyE],
                key: Key::KeyE,
                lowercase_char: "é",
                uppercase_char: "É",
            },
            Self::GraveAccentE => Combination {
                leader: vec![Key::Alt, Key::BackQuote],
                key: Key::KeyE,
                lowercase_char: "è",
                uppercase_char: "È",
            },
            Self::CircumflexE => Combination {
                leader: vec![Key::Alt, Key::KeyI],
                key: Key::KeyE,
                lowercase_char: "ê",
                uppercase_char: "Ê",
            },
            Self::UmlautE => Combination {
                leader: vec![Key::Alt, Key::KeyU],
                key: Key::KeyE,
                lowercase_char: "ë",
                uppercase_char: "Ë",
            },

            /* u */
            Self::AcuteAccentU => Combination {
                leader: vec![Key::Alt, Key::KeyE],
                key: Key::KeyU,
                lowercase_char: "ú",
                uppercase_char: "Ú",
            },
            Self::GraveAccentU => Combination {
                leader: vec![Key::Alt, Key::BackQuote],
                key: Key::KeyU,
                lowercase_char: "ù",
                uppercase_char: "Ù",
            },
            Self::CircumflexU => Combination {
                leader: vec![Key::Alt, Key::KeyI],
                key: Key::KeyU,
                lowercase_char: "û",
                uppercase_char: "Û",
            },
            Self::UmlautU => Combination {
                leader: vec![Key::Alt, Key::KeyU],
                key: Key::KeyU,
                lowercase_char: "ü",
                uppercase_char: "Ü",
            },
        };
    }
    fn all() -> Vec<Combination> {
        let mut combinations = Vec::new();
        for combination in Self::iter() {
            combinations.push(combination.get());
        }
        return combinations;
    }

    fn match_combination(pressed_keys: &PressedKeys) -> Option<(Combination, Case)> {
        let combinations = Self::all();
        for special_char in combinations {
            if special_char.leader.len() > pressed_keys.keys.len() {
                continue;
            }

            let has_uppercase_combination =
                pressed_keys.get_last(2) == vec![Key::ShiftLeft, special_char.key];
            let has_lowercase_combination = pressed_keys.get_last(1) == vec![special_char.key];

            if has_lowercase_combination {
                let full_length = special_char.leader.len() + 1;
                let has_leader = pressed_keys.get_last(full_length)
                    == [special_char.leader.as_slice(), &[special_char.key]]
                        .concat()
                        .as_slice();

                if has_leader {
                    return Some((special_char, Case::Lower));
                }
            }

            if has_uppercase_combination {
                let has_leader = pressed_keys.get_first(special_char.leader.len())
                    == special_char.leader.as_slice();

                if has_leader {
                    return Some((special_char, Case::Upper));
                }
            }

            // let has_lowercase_combination =
            //     pressed_keys.get_last(1) == special_char.lowercase_combination;

            // dbg!(has_uppercase_combination);

            // for (index, key) in pressed_keys.keys.iter().rev().enumerate() {
            //     if
            // }
        }

        return None;
    }
}

impl PressedKeys {
    fn new(size: usize) -> PressedKeys {
        return PressedKeys {
            history_length: size,
            keys: Vec::with_capacity(size),
        };
    }
    fn push(&mut self, key: Key) {
        if self.keys.len() >= self.history_length && !self.keys.is_empty() {
            self.keys.remove(0);
        }
        self.keys.push(key);
        // dbg!(&self.keys);
    }

    fn get_last(&self, n: usize) -> &[rdev::Key] {
        // Check if the requested size exceeds the vector length
        let start_index = if n >= self.keys.len() {
            0
        } else {
            self.keys.len() - n
        };

        // Return the slice from the calculated start index to the end of the vector
        &self.keys[start_index..]
    }

    fn get_first(&self, n: usize) -> &[rdev::Key] {
        // Check if the requested size exceeds the vector length
        let end_index = if n >= self.keys.len() {
            self.keys.len()
        } else {
            n
        };

        // Return the slice from the calculated start index to the end of the vector
        &self.keys[..end_index]
    }

    // fn match_combination(&self, combination: Vec<Key>) -> bool {
    //     if combination.len() > self.keys.len() {
    //         return false;
    //     }

    //     let keypresses = self.get_last(3);

    //     let mut matches = true;

    //     for (index, key) in combination.into_iter().enumerate() {
    //         dbg!(keypresses[index], &key);
    //         if keypresses[index] != key {
    //             matches = false;
    //         }
    //     }

    //     dbg!(keypresses);
    //     return matches;
    // }
}

fn main() {
    // dbg!(Combinations::all());
    let mut pressed_keys = PressedKeys::new(4);

    if let Err(error) = listen(move |event| callback(event, &mut pressed_keys)) {
        println!("Error: {:?}", error);
    }
}

// fn safe_index

fn handle_shortcut(pressed_keys: &PressedKeys) {
    let keys = pressed_keys.get_last(3);
    if keys.len() != 3 {
        return;
    }

    let result = Combinations::match_combination(pressed_keys);

    if let Some((combination, case)) = result {
        let char = combination.get_char(case);
        dbg!(char);
        type_char(char);
    }

    // let result = match keys.get(0).unwrap() {
    //     Key::Alt => match keys.get(1).unwrap() {
    //         // Key::Num2 => Some("€"),
    //         Key::KeyE => match keys.get(2).unwrap() {
    //             Key::KeyA => Some("á"),
    //             Key::KeyE => Some("é"),
    //             Key::KeyI => Some("í"),
    //             Key::KeyO => Some("ó"),
    //             Key::KeyU => Some("ú"),
    //             Key::KeyY => Some("ý"),
    //             _ => None,
    //         },
    //         Key::KeyI => match keys.get(2).unwrap() {
    //             Key::KeyI => Some("î"),
    //             Key::KeyA => Some("â"),
    //             Key::KeyE => Some("ê"),
    //             Key::KeyO => Some("ô"),
    //             Key::KeyU => Some("û"),
    //             _ => None,
    //         },
    //         Key::KeyU => match keys.get(2).unwrap() {
    //             Key::KeyA => Some("ä"),
    //             Key::KeyE => Some("ë"),
    //             Key::KeyI => Some("ï"),
    //             Key::KeyO => Some("ö"),
    //             Key::KeyU => Some("ö"),
    //             Key::KeyY => Some("ÿ"),
    //             _ => None,
    //         },
    //         _ => None,
    //     },
    //     _ => None,
    // };

    // if let Some(key) = result {
    //     dbg!(key);
    // }
}

fn callback(event: Event, pressed_keys: &mut PressedKeys) {
    // println!("testing");
    // let store = pressed_keys.push(Key::Alt);

    match event.event_type {
        EventType::KeyPress(key) => {
            // println!("Key pressed: {:?}", key);
            pressed_keys.push(key);
            // let combination: Vec<Key> = vec![Key::Alt, Key::KeyU, Key::KeyE];

            // let matches = pressed_keys.match_combination(combination);

            handle_shortcut(&pressed_keys)

            // dbg!(matches);
            // if let Some(_) = pressed_keys.iter().position(|&x| x == key) {
            //     return;
            // }
            // pressed_keys.push(key);
            // dbg!(&pressed_keys);
        }
        EventType::KeyRelease(key) => {
            // println!("Key released: {:?}", key);
            // if let Some(index) = pressed_keys.iter().position(|&x| x == key) {
            //     pressed_keys.remove(index);
            // }
            // dbg!(&pressed_keys);
        }
        _ => (),
    }
    // Ik ga nu eventjes typen een ik hoop dat
    // fn handle_shortcut() {

    // }
}

fn type_char(char: &str) {
    let mut enigo = Enigo::new();
    enigo.key_down(enigo::Key::Backspace);
    enigo.key_up(enigo::Key::Backspace);
    enigo.key_sequence(char);
}
