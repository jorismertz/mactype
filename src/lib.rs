use rdev::{listen, Event, EventType, Key};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
pub enum Case {
    Lower,
    Upper,
}

#[derive(Debug, EnumIter, PartialEq, Clone)]
pub enum Leader {
    AltE,
    AltU,
    AltI,
    AltN,
    AltBackquote,
}

impl Leader {
    pub fn diacritic_char(&self) -> &str {
        match self {
            Leader::AltE => "´",         // Acute Accent
            Leader::AltU => " ̈",         // Combining Diaeresis
            Leader::AltI => " ̂",         // Combining Circumflex Accent
            Leader::AltBackquote => "`", // Grave Accent
            Leader::AltN => "~",         // This is the best i could do :(
        }
    }

    pub fn from_keystrokes(keystrokes: &Vec<Key>) -> Option<Leader> {
        match keystrokes.as_slice() {
            [Key::Alt, Key::KeyE] => Some(Leader::AltE),
            [Key::Alt, Key::KeyU] => Some(Leader::AltU),
            [Key::Alt, Key::KeyI] => Some(Leader::AltI),
            [Key::Alt, Key::KeyN] => Some(Leader::AltN),
            [Key::Alt, Key::BackQuote] => Some(Leader::AltBackquote),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Shortcut {
    lowercase: &'static str,
    uppercase: &'static str,
    leader: Leader,
    key: Key,
}

impl Shortcut {
    pub fn char_from_case(&self, case: Case) -> &str {
        match case {
            Case::Lower => self.lowercase,
            Case::Upper => self.uppercase,
        }
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
    // RingA,
    AcuteAccentI,
    GraveAccentI,
    CircumflexI,
    UmlautI,
    AcuteAccentO,
    GraveAccentO,
    CircumflexO,
    UmlautO,
    TildeO,
    // SlashO,
    // CedillaC,
    TildeN,
    // Eszett,
}

// thank you chatgpt
impl Combinations {
    pub fn get(&self) -> Shortcut {
        match self {
            Self::AcuteAccentE => Shortcut {
                leader: Leader::AltE,
                key: Key::KeyE,
                lowercase: "é",
                uppercase: "É",
            },
            Self::GraveAccentE => Shortcut {
                leader: Leader::AltBackquote,
                key: Key::KeyE,
                lowercase: "è",
                uppercase: "È",
            },
            Self::CircumflexE => Shortcut {
                leader: Leader::AltI,
                key: Key::KeyE,
                lowercase: "ê",
                uppercase: "Ê",
            },
            Self::UmlautE => Shortcut {
                leader: Leader::AltU,
                key: Key::KeyE,
                lowercase: "ë",
                uppercase: "Ë",
            },

            /* U */
            Self::AcuteAccentU => Shortcut {
                leader: Leader::AltE,
                key: Key::KeyU,
                lowercase: "ú",
                uppercase: "Ú",
            },
            Self::GraveAccentU => Shortcut {
                leader: Leader::AltBackquote,
                key: Key::KeyU,
                lowercase: "ù",
                uppercase: "Ù",
            },
            Self::CircumflexU => Shortcut {
                leader: Leader::AltI,
                key: Key::KeyU,
                lowercase: "û",
                uppercase: "Û",
            },
            Self::UmlautU => Shortcut {
                leader: Leader::AltU,
                key: Key::KeyU,
                lowercase: "ü",
                uppercase: "Ü",
            },
            /* A */
            Self::AcuteAccentA => Shortcut {
                leader: Leader::AltE,
                key: Key::KeyA,
                lowercase: "á",
                uppercase: "Á",
            },
            Self::GraveAccentA => Shortcut {
                leader: Leader::AltBackquote,
                key: Key::KeyA,
                lowercase: "à",
                uppercase: "À",
            },
            Self::CircumflexA => Shortcut {
                leader: Leader::AltI,
                key: Key::KeyA,
                lowercase: "â",
                uppercase: "Â",
            },
            Self::UmlautA => Shortcut {
                leader: Leader::AltU,
                key: Key::KeyA,
                lowercase: "ä",
                uppercase: "Ä",
            },
            Self::TildeA => Shortcut {
                leader: Leader::AltN,
                key: Key::KeyA,
                lowercase: "ã",
                uppercase: "Ã",
            },
            // Self::RingA => Shortcut {
            //     leader: Leader::new(vec![Key::Alt]),
            //     key: Key::KeyA,
            //     lowercase: "å",
            //     uppercase: "Å",
            // },

            /* I */
            Self::AcuteAccentI => Shortcut {
                leader: Leader::AltE,
                key: Key::KeyI,
                lowercase: "í",
                uppercase: "Í",
            },
            Self::GraveAccentI => Shortcut {
                leader: Leader::AltBackquote,
                key: Key::KeyI,
                lowercase: "ì",
                uppercase: "Ì",
            },
            Self::CircumflexI => Shortcut {
                leader: Leader::AltI,
                key: Key::KeyI,
                lowercase: "î",
                uppercase: "Î",
            },
            Self::UmlautI => Shortcut {
                leader: Leader::AltU,
                key: Key::KeyI,
                lowercase: "ï",
                uppercase: "Ï",
            },

            /* O */
            Self::AcuteAccentO => Shortcut {
                leader: Leader::AltE,
                key: Key::KeyO,
                lowercase: "ó",
                uppercase: "Ó",
            },
            Self::GraveAccentO => Shortcut {
                leader: Leader::AltBackquote,
                key: Key::KeyO,
                lowercase: "ò",
                uppercase: "Ò",
            },
            Self::CircumflexO => Shortcut {
                leader: Leader::AltI,
                key: Key::KeyO,
                lowercase: "ô",
                uppercase: "Ô",
            },
            Self::UmlautO => Shortcut {
                leader: Leader::AltU,
                key: Key::KeyO,
                lowercase: "ö",
                uppercase: "Ö",
            },
            Self::TildeO => Shortcut {
                leader: Leader::AltN,
                key: Key::KeyO,
                lowercase: "õ",
                uppercase: "Õ",
            },
            // Self::SlashO => Shortcut {
            //     leader: Leader::new(vec![Key::Alt]),
            //     key: Key::KeyO,
            //     lowercase: "ø",
            //     uppercase: "Ø",
            // },

            // /* Misc */
            // Self::CedillaC => Shortcut {
            //     leader: Leader::new(vec![Key::Alt]),
            //     key: Key::KeyC,
            //     lowercase: "ç",
            //     uppercase: "Ç",
            // },
            Self::TildeN => Shortcut {
                leader: Leader::AltN,
                key: Key::KeyN,
                lowercase: "ñ",
                uppercase: "Ñ",
            },
            // Self::Eszett => Shortcut {
            //     leader: Leader::new(vec![Key::Alt]),
            //     key: Key::KeyS,
            //     lowercase: "ß",
            //     uppercase: "ß", // Note: ß traditionally does not have an uppercase variant, though an uppercase variant (ẞ) exists.
            // },

            // _ => Shortcut{key: Key::Alt}
        }
    }
    pub fn all() -> Vec<Shortcut> {
        Self::iter().map(|x| x.get()).collect::<Vec<Shortcut>>()
    }

    pub fn with_leader(leader: &Leader) -> Vec<Shortcut> {
        let combinations = Self::all();
        combinations
            .iter()
            .filter(|&x| &x.leader == leader)
            .cloned()
            .collect()
    }

    pub fn from_leader_and_key(leader: &Leader, keys: Vec<Key>) -> Option<(Shortcut, Case)> {
        let all = Self::all();
        let mut case: Case = Case::Lower;

        let result = all.iter().find(|&x| {
            let same_leader = &x.leader == leader;
            let same_key = &x.key == keys.last().unwrap();

            if [Key::ShiftLeft, x.key] == keys.as_slice() {
                case = Case::Upper;
            }

            return same_key && same_leader;
        });

        if let Some(shortcut) = result {
            return Some((shortcut.clone(), case));
        }

        None
    }
}
