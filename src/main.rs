use enigo::{Enigo, KeyboardControllable};
use mactype::{Combinations, Leader, Shortcut};
use rdev::{listen, Event, EventType, Key};

#[derive(Debug, Clone)]
struct State {
    leader: Option<Leader>,
    keystrokes: Vec<Key>,
}

impl State {
    fn keystroke_index(&self, key: Key) -> Option<usize> {
        self.keystrokes.iter().position(|&x| x == key)
    }

    fn has_keystroke(&self, key: Key) -> bool {
        self.keystroke_index(key).is_some()
    }
}

fn main() {
    let mut state = State {
        leader: None,
        keystrokes: Vec::new(),
    };

    if let Err(error) = listen(move |event| callback(event, &mut state)) {
        println!("Error: {:?}", error);
    }
}

fn handle_keypress(state: &mut State) {
    let leader = Leader::from_keystrokes(&state.keystrokes);

    if state.keystrokes.len() == 1 {
        if let Some(key) = state.keystrokes.get(0) {
            if key == &Key::ShiftLeft {
                return;
            }
        }
    }

    if let Some(leader) = &state.leader {
        let result = Combinations::from_leader_and_key(leader, state.keystrokes.clone());
        state.leader = None;

        if let Some((shortcut, case)) = result {
            let char = shortcut.char_from_case(case);
            dbg!(char);
            type_char(char, true);
        }
    }

    if let Some(leader) = leader {
        // This is causing some breakage, rdev will catch the key sent with enigo.
        // Might be able to work around this by setting a blocking_for variable in the state
        // that way we can just ignore the string being sent in. maybe?

        // type_char(leader.diacritic_char(), false);
        state.leader = Some(leader);
    }
}

fn callback(event: Event, state: &mut State) {
    match event.event_type {
        EventType::KeyPress(key) => {
            if !state.has_keystroke(key) {
                state.keystrokes.push(key);
            }

            // dbg!(&state.keystrokes);
            handle_keypress(state);

            dbg!(&state);
        }
        EventType::KeyRelease(key) => {
            if let Some(index) = state.keystroke_index(key) {
                state.keystrokes.remove(index);
            }
        }
        _ => {}
    }
}

fn type_char(char: &str, replace: bool) {
    let mut enigo = Enigo::new();
    if replace {
        enigo.key_down(enigo::Key::Backspace);
        enigo.key_up(enigo::Key::Backspace);
    }
    enigo.key_sequence(char);
}
