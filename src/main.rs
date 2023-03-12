use rdev::{grab, simulate, Event, EventType, Key};
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;
use std::time::{self, Duration, Instant};

// use std::process::Command;

#[derive(Debug)]
struct CapsState {
    pressed_down_at: Instant,
    is_pressed_down: bool,
}

fn main() {
    let snapshot = Rc::new(Mutex::new(CapsState {
        pressed_down_at: Instant::now(),
        is_pressed_down: false,
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

fn callback(event: Event, snapshot: Rc<Mutex<CapsState>>) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(Key::CapsLock) => {
            let mut locked_snapshot = snapshot.lock().unwrap();

            if !locked_snapshot.is_pressed_down {
                locked_snapshot.pressed_down_at = Instant::now();
                locked_snapshot.is_pressed_down = true;

                send(&EventType::KeyPress(Key::MetaLeft));
            }

            return None;
        }
        EventType::KeyRelease(Key::CapsLock) => {
            send(&EventType::KeyRelease(Key::MetaLeft));

            let mut locked_snapshot = snapshot.lock().unwrap();

            locked_snapshot.is_pressed_down = false;

            let time_elapsed_since_pressed_down = locked_snapshot.pressed_down_at.elapsed();

            if time_elapsed_since_pressed_down < Duration::from_millis(176) {
                send(&EventType::KeyPress(Key::Escape));
                send(&EventType::KeyRelease(Key::Escape));
                println!("Just a keypress");
            } else {
                println!("Held Down");
            }

            return None;
        }
        _ => Some(event),
    }
}
