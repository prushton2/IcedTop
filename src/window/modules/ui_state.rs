use iced::border::Radius;
use iced::{Border, Length, Shadow, Task, Vector};
use iced::widget::{text, button, row, column};
use iced_widget::scrollable::Rail;

use crate::window;
use crate::window::modules::ps;

#[derive(Copy, Clone, PartialEq)]
pub enum UIState {
    Ps,
    Systemd,
    Hardware,
}

#[derive(Copy, Clone, Debug)]
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

pub fn view(this: &window::Window) -> iced::Element<'_, window::Message> {
    match this.ui_state {
        UIState::Ps => {
            return Widgets::ribbon(this, 
                row![
                    this.ps.view(ps::PsViewSortBy::Cpu)
                ]
                .padding(iced::Padding::new(0.0).horizontal(50.0).top(50.0))
                .into()
            )
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

pub struct Styles;

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

    pub fn processes_inner_box(theme: &iced::Theme) -> iced_widget::scrollable::Style {
        iced_widget::scrollable::Style {
            container: iced_widget::container::Style {
                text_color: Some(theme.extended_palette().background.weak.text),
                background: Some(iced::Background::Color(theme.extended_palette().background.weak.color)),
                border: Border { 
                    color: theme.extended_palette().background.base.color,
                    width: 0.0, 
                    radius: Radius::new(0.0) 
                },
                shadow: Shadow { 
                    color: theme.extended_palette().background.base.color,
                    offset: iced::Vector::ZERO, 
                    blur_radius: 0.0
                },
                snap: false,
            },
            vertical_rail: Rail { 
                background: Some(iced::Background::Color(theme.extended_palette().background.weak.color)),
                border: Border { 
                    color: theme.extended_palette().background.base.color, 
                    width: 0.0, 
                    radius: Radius::new(0.0)
                },
                scroller: iced_widget::scrollable::Scroller { 
                    background: iced::Background::Color(theme.extended_palette().primary.weak.color), 
                    border: Border { 
                        color: theme.extended_palette().background.base.color, 
                        width: 0.0, 
                        radius: Radius::new(0.0)
                    }
                },  
            },
            horizontal_rail: Rail { 
                background: Some(iced::Background::Color(theme.extended_palette().primary.base.color)),
                border: Border { 
                    color: theme.extended_palette().background.base.color, 
                    width: 0.0, 
                    radius: Radius::new(0.0)
                },
                scroller: iced_widget::scrollable::Scroller { 
                    background: iced::Background::Color(theme.extended_palette().primary.base.color), 
                    border: Border { 
                        color: theme.extended_palette().background.base.color, 
                        width: 0.0, 
                        radius: Radius::new(0.0)
                    }
                },  
            },
            gap: Some(iced::Background::Color(theme.extended_palette().primary.base.color)),
            auto_scroll: iced_widget::scrollable::AutoScroll {
                background: iced::Background::Color(theme.extended_palette().primary.base.color), 
                border: Border { 
                    color: theme.extended_palette().background.base.color, 
                    width: 0.0, 
                    radius: Radius::new(0.0)
                },
                shadow: Shadow { 
                    color: theme.extended_palette().background.base.color,
                    offset: iced::Vector::ZERO, 
                    blur_radius: 0.0
                },
                icon: theme.extended_palette().primary.strong.color
            },
        }
    }
}