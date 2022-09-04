use std::{thread, time::Duration};
use std::process::Command;
use mki::{bind_key, Action, InhibitEvent, Keyboard, Sequence};

pub struct KeyBind {
    pub keys: Vec<Keyboard>,
    pub action: Box<dyn Fn() + Sync + Send>,
}
fn main() {
    println!("Hello, world!");
    let binds  = vec![
        KeyBind {
            keys: vec![Keyboard::Space, Keyboard::B],
            action: Box::new(|| {
                start_application("Start Chrome")
            })
        },
        KeyBind {
            keys: vec![Keyboard::Space, Keyboard::T],
            action : Box::new(|| {
                start_application("Start WindowsTerminal")
            })
        },

    ];
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

