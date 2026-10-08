use std::time::Duration;

use iced::Task;

use crate::window::{Message, Window, modules::ui_state::UIState};
use crate::window::modules::*;

impl Window {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                ps: ps::Ps::new(),
                ui_state: UIState::Ps
            },
            Task::done(Message::PsTick)
        )
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UIUpdate(message) => {
                crate::window::modules::ui_state::update(self, message)
            }
            Message::PsTick => {
                self.ps.fetch();
                Task::none()
            }
            Message::PsRefreshed(processes) => {
                self.ps.processes = processes;
                Task::none()
            }
        }
    }
    pub fn view(&self) -> iced::Element<'_, Message> {
        crate::window::modules::ui_state::view(self)
    }
    pub fn subscriptions(&self) -> iced::Subscription<Message> {
        let subscriptions = vec![
            iced::time::every(Duration::from_secs(2)).map(|_| Message::PsTick)
        ];

        return iced::Subscription::batch(subscriptions)
    }
}
