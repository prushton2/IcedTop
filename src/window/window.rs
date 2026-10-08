use iced::Task;

use crate::window::{Message, Window, modules::ui_state::UIState};

impl Window {
    pub fn new() -> Self {
        Self {
            ui_state: UIState::Ps
        }
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UIUpdate(message) => {
                crate::window::modules::ui_state::update(self, message)
            }
        }
    }
    pub fn view(&self) -> iced::Element<'_, Message> {
        crate::window::modules::ui_state::view(self)
    }
}