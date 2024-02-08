use enigo::*;
use mactype::{Combinations, PressedKeys};
use rdev::{listen, Event, EventType};

fn main() {
    let mut pressed_keys = PressedKeys::new(4);

    if let Err(error) = listen(move |event| callback(event, &mut pressed_keys)) {
        println!("Error: {:?}", error);
    }
}

fn handle_shortcut(pressed_keys: &PressedKeys) {
    let keys = pressed_keys.get_last(3);
    if keys.len() != 3 {
        return;
    }

    let result = Combinations::match_combination(pressed_keys);

    if let Some((combination, case)) = result {
        let char = combination.get_char(case);
        type_char(char);
    }
}

fn callback(event: Event, pressed_keys: &mut PressedKeys) {
    if let EventType::KeyPress(key) = event.event_type {
        pressed_keys.push(key);
        handle_shortcut(pressed_keys)
    }

    // i do want to add some logic here later to capture the alt + char combination and use that as the leader.
    // This way i wouldn't have to simulate a backspace either
}

fn type_char(char: &str) {
    let mut enigo = Enigo::new();
    enigo.key_down(enigo::Key::Backspace);
    enigo.key_up(enigo::Key::Backspace);
    enigo.key_sequence(char);
}
