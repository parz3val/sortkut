use mki::{bind_key, Action, InhibitEvent, Keyboard, Sequence};
use std::process::Command;
use std::{thread, time::Duration};
pub mod config;
pub mod utils;

fn main() {
    let cfg = config::setup::load_config_json();

    println!("{:?}", cfg);
    let binds = utils::utils::build_keybinds();
    for keybind in binds {
        mki::register_hotkey(&keybind.keys, keybind.action)
    }
    // mki::register_hotkey(&[Keyboard::LeftWindows, Keyboard::LeftShift, Keyboard::B], || {
    //     println!("Windows+Shift+B Pressed");
    //     start_application("Start Chrome");
    // });

    // mki::register_hotkey(&[Keyboard::LeftWindows, Keyboard::LeftShift, Keyboard::T], || {
    //     start_application("Start WindowsTerminal")

    // });
    loop {}
}
