use enigo::{Enigo, Key, KeyboardControllable};
use std::thread;
use std::time::Duration;

fn main() {
    thread::sleep(Duration::from_millis(500));
    let mut enigo = Enigo::new();

    enigo.key_down(Key::Raw(0x37));
    enigo.key_down(Key::Raw(0x09));
}
