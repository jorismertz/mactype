use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use rdev::Key;

pub struct PressedKeys {
    history_length: usize,
    keys: Vec<rdev::Key>,
}

#[derive(Debug)]
pub struct Combination {
    leader: Vec<Key>,
    key: Key,
    lowercase_char: &'static str,
    uppercase_char: &'static str,
}

impl Combination {
    pub fn get_char(&self, case: Case) -> &str {
        if let Case::Lower = case {
            return self.lowercase_char;
        }
        self.uppercase_char
    }
}

#[derive(Debug, EnumIter)]
pub enum Combinations {
    AcuteAccentE,
    GraveAccentE,
    CircumflexE,
    UmlautE,
    AcuteAccentU,
    GraveAccentU,
    CircumflexU,
    UmlautU,
    AcuteAccentA,
    GraveAccentA,
    CircumflexA,
    UmlautA,
    TildeA,
    RingA,
    AcuteAccentI,
    GraveAccentI,
    CircumflexI,
    UmlautI,
    AcuteAccentO,
    GraveAccentO,
    CircumflexO,
    UmlautO,
    TildeO,
    SlashO,
    CedillaC,
    TildeN,
    // Eszett,
}

#[derive(Debug)]
pub enum Case {
    Lower,
    Upper,
}

// thank you chatgpt
impl Combinations {
    pub fn get(&self) -> Combination {
        match self {
            /* E */
            Self::AcuteAccentE => Combination {
                leader: vec![rdev::Key::Alt, rdev::Key::KeyE],
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

            /* misc */
            Self::AcuteAccentA => Combination {
                leader: vec![Key::Alt, Key::KeyE],
                key: Key::KeyA,
                lowercase_char: "á",
                uppercase_char: "Á",
            },
            Self::GraveAccentA => Combination {
                leader: vec![Key::Alt, Key::BackQuote],
                key: Key::KeyA,
                lowercase_char: "à",
                uppercase_char: "À",
            },
            Self::CircumflexA => Combination {
                leader: vec![Key::Alt, Key::KeyI],
                key: Key::KeyA,
                lowercase_char: "â",
                uppercase_char: "Â",
            },
            Self::UmlautA => Combination {
                leader: vec![Key::Alt, Key::KeyU],
                key: Key::KeyA,
                lowercase_char: "ä",
                uppercase_char: "Ä",
            },
            Self::TildeA => Combination {
                leader: vec![Key::Alt, Key::KeyN],
                key: Key::KeyA,
                lowercase_char: "ã",
                uppercase_char: "Ã",
            },
            Self::RingA => Combination {
                leader: vec![Key::Alt],
                key: Key::KeyA,
                lowercase_char: "å",
                uppercase_char: "Å",
            },
            Self::AcuteAccentI => Combination {
                leader: vec![Key::Alt, Key::KeyE],
                key: Key::KeyI,
                lowercase_char: "í",
                uppercase_char: "Í",
            },
            Self::GraveAccentI => Combination {
                leader: vec![Key::Alt, Key::BackQuote],
                key: Key::KeyI,
                lowercase_char: "ì",
                uppercase_char: "Ì",
            },
            Self::CircumflexI => Combination {
                leader: vec![Key::Alt, Key::KeyI],
                key: Key::KeyI,
                lowercase_char: "î",
                uppercase_char: "Î",
            },
            Self::UmlautI => Combination {
                leader: vec![Key::Alt, Key::KeyU],
                key: Key::KeyI,
                lowercase_char: "ï",
                uppercase_char: "Ï",
            },
            Self::AcuteAccentO => Combination {
                leader: vec![Key::Alt, Key::KeyE],
                key: Key::KeyO,
                lowercase_char: "ó",
                uppercase_char: "Ó",
            },
            Self::GraveAccentO => Combination {
                leader: vec![Key::Alt, Key::BackQuote],
                key: Key::KeyO,
                lowercase_char: "ò",
                uppercase_char: "Ò",
            },
            Self::CircumflexO => Combination {
                leader: vec![Key::Alt, Key::KeyI],
                key: Key::KeyO,
                lowercase_char: "ô",
                uppercase_char: "Ô",
            },
            Self::UmlautO => Combination {
                leader: vec![Key::Alt, Key::KeyU],
                key: Key::KeyO,
                lowercase_char: "ö",
                uppercase_char: "Ö",
            },
            Self::TildeO => Combination {
                leader: vec![Key::Alt, Key::KeyN],
                key: Key::KeyO,
                lowercase_char: "õ",
                uppercase_char: "Õ",
            },
            Self::SlashO => Combination {
                leader: vec![Key::Alt],
                key: Key::KeyO,
                lowercase_char: "ø",
                uppercase_char: "Ø",
            },
            Self::CedillaC => Combination {
                leader: vec![Key::Alt],
                key: Key::KeyC,
                lowercase_char: "ç",
                uppercase_char: "Ç",
            },
            Self::TildeN => Combination {
                leader: vec![Key::Alt, Key::KeyN],
                key: Key::KeyN,
                lowercase_char: "ñ",
                uppercase_char: "Ñ",
            },
            // This one don't work yet.
            // Add some functionality to allow for shorter shortcuts
            // Self::Eszett => Combination {
            //     leader: vec![Key::Alt],
            //     key: Key::KeyS,
            //     lowercase_char: "ß",
            //     uppercase_char: "ß", // Note: ß does not have an uppercase variant in traditional use, though an uppercase variant (ẞ) exists.
            // },
        }
    }
    pub fn all() -> Vec<Combination> {
        let mut combinations = Vec::new();
        for combination in Self::iter() {
            combinations.push(combination.get());
        }
        combinations
    }

    // god help me
    pub fn match_combination(pressed_keys: &PressedKeys) -> Option<(Combination, Case)> {
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
        }

        None
    }
}

impl PressedKeys {
    pub fn new(size: usize) -> PressedKeys {
        PressedKeys {
            history_length: size,
            keys: Vec::with_capacity(size),
        }
    }
    pub fn push(&mut self, key: Key) {
        if self.keys.len() >= self.history_length && !self.keys.is_empty() {
            self.keys.remove(0);
        }
        self.keys.push(key);
    }

    pub fn get_last(&self, n: usize) -> &[rdev::Key] {
        let start_index = if n >= self.keys.len() {
            0
        } else {
            self.keys.len() - n
        };

        &self.keys[start_index..]
    }

    pub fn get_first(&self, n: usize) -> &[rdev::Key] {
        let end_index = if n >= self.keys.len() {
            self.keys.len()
        } else {
            n
        };

        &self.keys[..end_index]
    }
}
