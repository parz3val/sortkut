use std::{thread, time::Duration};
use std::process::Command;
use mki::{bind_key, Action, InhibitEvent, Keyboard, Sequence};
use super::types::KeyBind;

pub fn build_keybinds()-> Vec<KeyBind> {
    vec![
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
        KeyBind {
            keys: vec![Keyboard::Space, Keyboard::F],
            action : Box::new(|| {
                start_application("Start Firefox")
            })
        },
        KeyBind {
            keys: vec![Keyboard::Space, Keyboard::C],
            action : Box::new(|| {
                start_application(r#"Start "C:\Users\harri\AppData\Roaming\Telegram Desktop\Telegram.exe""#)
            })
        },
         KeyBind {
            keys: vec![Keyboard::Space, Keyboard::Z],
            action : Box::new(|| {
                start_application(r#"Start "C:\Users\harri\scoop\apps\zotero\current\zotero.exe""#)
            })
        },       
    ]
}

fn start_application(application: &str) {
    Command::new("powershell").arg(application).spawn().expect("Failed to execute command");

}