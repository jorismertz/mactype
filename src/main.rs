use enigo::{Enigo, KeyboardControllable};
use mactype::{Combinations, Leader};
use rdev::{listen, Event, EventType, Key};
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
};
use std::{error::Error, fs, path::PathBuf, process, thread};

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

fn lock_pid_file(filename: &str) -> std::io::Result<PathBuf> {
    let path = format!(
        "/var/run/user/{}/{}.pid",
        users::get_current_uid(),
        filename
    );

    if let Ok(_) = fs::read(&path) {
        panic!("Unable to lock PID file, file already exists");
    }

    fs::write(&path, process::id().to_string())?;
    Ok(PathBuf::from(path))
}

fn unlock_pid_file(path: &PathBuf) -> std::io::Result<()> {
    fs::remove_file(path)?;
    Ok(())
}

fn spawn_pid_file_unlocker(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut signals = Signals::new(&[SIGINT, SIGTERM])?;

    thread::spawn(move || {
        for _ in signals.forever() {
            unlock_pid_file(&path).expect(
                "Failed to spawn pid file unlocker thread, make sure to remove it manually",
            );
            std::process::exit(0);
        }
    });

    Ok(())
}

fn main() {
    let mut state = State {
        leader: None,
        keystrokes: Vec::new(),
        block_for: 0,
    };

    let pid_file_path = lock_pid_file("mactype").unwrap();

    // This will delete / 'unlock' the pid file upon receiving SIGTERM or SIGINT
    spawn_pid_file_unlocker(pid_file_path).unwrap();

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
