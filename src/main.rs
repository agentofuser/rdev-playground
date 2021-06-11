use rdev::{grab, simulate, Event, EventType, Key};

fn grab_tab(event: Event) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(Key::Tab) => {
            println!("Blocked a Tab!");
            simulate(&EventType::KeyPress(Key::KeyK))
                .expect("Failed to simulate");
            println!("Replaced Tab with k!");
            None
        }
        EventType::KeyRelease(Key::Tab) => None,
        _ => Some(event),
    }
}
fn main() {
    println!("Hello, world!");

    grab(grab_tab).expect("Could not grab");
}
