use crate::window::modules::ui_state::{UIMessage, UIState};
use crate::window::modules::*;

pub mod modules;
pub mod window;

pub struct Window {
    ps: ps::Ps,
    ui_state: UIState
}

#[derive(Clone, Debug)]
pub enum Message {
    UIUpdate(UIMessage),
    PsTick,
    PsRefreshed(Vec<ps::Process>),
}