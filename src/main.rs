use rdev::{grab, simulate, Event, EventType, Key};
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let grab_keyboard = |event: Event| {
        let keylog_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("user_data/keylog.cbor")
            .expect("Could not open log file.");
        match event.event_type {
            // EventType::KeyPress(Key::Tab) => {
            //     println!("Blocked a Tab!");
            //     simulate(&EventType::KeyPress(Key::KeyK))
            //         .expect("Failed to simulate");
            //     println!("Replaced Tab with k!");
            //     None
            // }
            // EventType::KeyRelease(Key::Tab) => None,
            EventType::KeyPress(_) | EventType::KeyRelease(_) => {
                println!("{:?}", event);
                serde_cbor::to_writer(keylog_file, &event)
                    .expect("Could not write to log file.");
                Some(event)
            }
            _ => Some(event),
        }
    };

    grab(grab_keyboard).expect("Could not grab");
    Ok(())
}
