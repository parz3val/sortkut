use mki::{bind_key, Action, InhibitEvent, Keyboard, Sequence};

pub struct KeyBind {
    pub keys: Vec<Keyboard>,
    pub action: Box<dyn Fn() + Sync + Send>,
}