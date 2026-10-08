use crate::window::modules::ui_state::{UIMessage, UIState};

pub mod modules;
pub mod window;

pub struct Window {
    ui_state: UIState
}

#[derive(Copy, Clone)]
pub enum Message {
    UIUpdate(UIMessage)
}