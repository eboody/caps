use rdev::{grab, simulate, Event, EventType, Key};
use std::rc::Rc;
use std::sync::Mutex;
// use std::thread;

#[derive(Debug)]
struct CapsState {
    is_pressed_down: bool,
    something_else_was_pressed: bool,
}

fn main() {
    let snapshot = Rc::new(Mutex::new(CapsState {
        is_pressed_down: false,
        something_else_was_pressed: false,
    }));
    // This will block.
    if let Err(error) = grab(move |event| callback(event, snapshot.clone())) {
        println!("Error: {:?}", error)
    }
}

fn send(event_type: &EventType) {
    // let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(_) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    // thread::sleep(delay);
}

fn callback(event: Event, caps_state: Rc<Mutex<CapsState>>) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(Key::CapsLock) => {
            let mut locked_caps_state = caps_state.lock().unwrap();

            if !locked_caps_state.is_pressed_down {
                locked_caps_state.is_pressed_down = true;
            }

            return None;
        }
        EventType::KeyRelease(Key::CapsLock) => {
            send(&EventType::KeyRelease(Key::MetaLeft));

            let mut locked_caps_state = caps_state.lock().unwrap();

            if locked_caps_state.something_else_was_pressed {
                send(&EventType::KeyRelease(Key::MetaLeft));
            } else {
                println!("{:#?}", locked_caps_state);
                send(&EventType::KeyPress(Key::Escape));
                send(&EventType::KeyRelease(Key::Escape));
            }

            locked_caps_state.is_pressed_down = false;
            locked_caps_state.something_else_was_pressed = false;

            return None;
        }
        EventType::KeyPress(key) => {
            let mut locked_caps_state = caps_state.lock().unwrap();

            if locked_caps_state.is_pressed_down {
                send(&EventType::KeyPress(Key::MetaLeft));
                send(&EventType::KeyPress(key));
                if key != Key::CapsLock {
                    locked_caps_state.something_else_was_pressed = true;
                }
                return None;
            } else {
                return Some(event);
            }
        }
        EventType::KeyRelease(key) => {
            let locked_caps_state = caps_state.lock().unwrap();
            send(&EventType::KeyRelease(key));

            if locked_caps_state.is_pressed_down {
                return None;
            } else {
                return Some(event);
            }
        }
        _ => Some(event),
    }
}
