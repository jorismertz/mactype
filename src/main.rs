use enigo::{Enigo, KeyboardControllable};
use mactype::{Combinations, Leader};
use rdev::{listen, Event, EventType, Key};

#[derive(Debug, Clone)]
struct State {
    leader: Option<Leader>,
    keystrokes: Vec<Key>,
    block_for: usize,
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
        block_for: 0,
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
            type_char(char, state, true);
        }
    }

    if let Some(leader) = leader {
        // Is seems enigo isn't able to type these niche characters.
        // might have to find a different library to handle this
        // type_char(leader.diacritic_char(), state, false);

        state.leader = Some(leader);
    }
}

fn callback(event: Event, state: &mut State) {
    match event.event_type {
        EventType::KeyPress(key) => {
            // Keypresses are simulated with a different library than we use for getting events
            // Because of this they will be caught by this function and can mess up some logic.
            // Incrementing this variable before sending off a keypress remedies this behaviouir
            if state.block_for > 0 {
                state.block_for -= 1;
                return;
            }

            if !state.has_keystroke(key) {
                state.keystrokes.push(key);
            }

            handle_keypress(state);
        }
        EventType::KeyRelease(key) => {
            if let Some(index) = state.keystroke_index(key) {
                state.keystrokes.remove(index);
            }
        }
        _ => {}
    }
}

fn type_char(char: &str, state: &mut State, replace: bool) {
    let mut enigo = Enigo::new();
    if replace {
        // Only keyup has to be accounted for since keydown doesn't have any
        // meaningfull logic tied to it
        state.block_for += 1;
        enigo.key_down(enigo::Key::Backspace);
        enigo.key_up(enigo::Key::Backspace);
    }
    state.block_for += 1;
    enigo.key_sequence(char);
}
