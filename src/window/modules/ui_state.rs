use iced::border::Radius;
use iced::{Border, Length, Shadow, Task, Vector};
use iced::widget::{text, button, row, column};

use crate::window;

#[derive(Copy, Clone, PartialEq)]
pub enum UIState {
    Hardware,
    Ps,
    Systemd,
}

#[derive(Copy, Clone)]
pub enum UIMessage {
    TabHardware,
    TabPs,
    TabSystemd
}

pub fn update(this: &mut window::Window, message: UIMessage) -> Task<window::Message> {
    match message {
        UIMessage::TabHardware => {
            this.ui_state = UIState::Hardware;   
            Task::none()
        }
        UIMessage::TabPs => {
            this.ui_state = UIState::Ps;
            Task::none()
        },
        UIMessage::TabSystemd => {
            this.ui_state = UIState::Systemd;
            Task::none()
        }
    }
}

pub fn view(this: &window::Window) -> iced::Element<'static, window::Message> {
    match this.ui_state {
        UIState::Ps => {
            return Widgets::ribbon(this, text("ps").into())
        },
        UIState::Systemd => {
            return Widgets::ribbon(this, text("systemd").into())
        },
        UIState::Hardware => {
            return Widgets::ribbon(this, text("hardware").into())
        }
    }
}

struct Widgets;

impl Widgets {
    pub fn ribbon<'a>(this: &window::Window, inner: iced::Element<'a, window::Message>) -> iced::Element<'a, window::Message> {
        return column![
            row![
                if this.ui_state == UIState::Ps {
                    button("Processes")
                        .style(|t, _| Styles::tab_selected(t))
                        .on_press(window::Message::UIUpdate(UIMessage::TabPs))
                        .width(Length::Fill)
                } else {
                    button("Processes")
                        .style(|t, _| Styles::tab_unselected(t))
                        .on_press(window::Message::UIUpdate(UIMessage::TabPs))
                        .width(Length::Fill)
                },

                if this.ui_state == UIState::Systemd {
                    button("Systemd")
                        .style(|t, _| Styles::tab_selected(t))
                        .on_press(window::Message::UIUpdate(UIMessage::TabSystemd))
                        .width(Length::Fill)
                } else {
                    button("Systemd")
                        .style(|t, _| Styles::tab_unselected(t))
                        .on_press(window::Message::UIUpdate(UIMessage::TabSystemd))
                        .width(Length::Fill)
                },

                if this.ui_state == UIState::Hardware {
                    button("Hardware")
                        .style(|t, _| Styles::tab_selected(t))
                        .on_press(window::Message::UIUpdate(UIMessage::TabHardware))
                        .width(Length::Fill)
                } else {
                    button("Hardware")
                        .style(|t, _| Styles::tab_unselected(t))
                        .on_press(window::Message::UIUpdate(UIMessage::TabHardware))
                        .width(Length::Fill)
                },
            ].width(Length::Fill),
            inner
        ].width(Length::Fill)
        .into()
    }
}

struct Styles;

impl Styles {
    pub fn tab_selected(theme: &iced::Theme) -> iced_widget::button::Style {
        iced_widget::button::Style {
            background: None,
            text_color: theme.palette().text,
            border: Border {
                color: theme.palette().success,
                width: 0.0,
                radius: Radius { top_left: 0.0, top_right: 0.0, bottom_right: 0.0, bottom_left: 0.0 }
            },
            shadow: Shadow {
                color: theme.palette().background,
                offset: Vector {
                    x: 0.0,
                    y: 0.0
                },
                blur_radius: 0.0
            },
            snap: false
        }
    }
    pub fn tab_unselected(theme: &iced::Theme) -> iced_widget::button::Style {
        iced_widget::button::Style {
            background: Some(iced::Background::Color(theme.palette().primary)),
            text_color: theme.extended_palette().primary.base.text,
            border: Border {
                color: theme.palette().success,
                width: 0.0,
                radius: Radius { top_left: 0.0, top_right: 0.0, bottom_right: 0.0, bottom_left: 0.0 }
            },
            shadow: Shadow {
                color: theme.palette().background,
                offset: Vector {
                    x: 0.0,
                    y: 0.0
                },
                blur_radius: 0.0
            },
            snap: false
        }
    }
}