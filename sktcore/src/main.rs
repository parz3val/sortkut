use std::{thread, time::Duration};
use std::process::Command;
use mki::{bind_key, Action, InhibitEvent, Keyboard, Sequence};
pub mod utils;


fn main() {
    println!("Hello, world!");
    let binds  = utils::utils::build_keybinds();
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
    loop{

    }
}

fn start_application(application: &str) {
    Command::new("powershell").arg(application).spawn().expect("Failed to execute command");

}

