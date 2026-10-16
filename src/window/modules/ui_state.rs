use iced::border::Radius;
use iced::{Border, Length, Padding, Shadow, Task, Vector};
use iced::widget::{text, button, row, column};
use iced_widget::container;

use crate::window;

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
                    this.ps.view().map(|e| window::Message::PsMessage(e))
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
        return row![
            container(
                column![
                    if this.ui_state == UIState::Ps {
                        button("Processes")
                            .style(|t, _| Styles::tab_selected(t))
                            .on_press(window::Message::UIUpdate(UIMessage::TabPs))
                            .width(Length::Fill)
                            .padding(Padding::new(20.0))
                    } else {
                        button("Processes")
                            .style(|t, _| Styles::tab_unselected(t))
                            .on_press(window::Message::UIUpdate(UIMessage::TabPs))
                            .width(Length::Fill)
                            .padding(Padding::new(20.0))
                    },
    
                    if this.ui_state == UIState::Systemd {
                        button("Systemd")
                            .style(|t, _| Styles::tab_selected(t))
                            .on_press(window::Message::UIUpdate(UIMessage::TabSystemd))
                            .width(Length::Fill)
                            .padding(Padding::new(20.0))
                    } else {
                        button("Systemd")
                            .style(|t, _| Styles::tab_unselected(t))
                            .on_press(window::Message::UIUpdate(UIMessage::TabSystemd))
                            .width(Length::Fill)
                            .padding(Padding::new(20.0))
                    },
    
                    if this.ui_state == UIState::Hardware {
                        button("Hardware")
                            .style(|t, _| Styles::tab_selected(t))
                            .on_press(window::Message::UIUpdate(UIMessage::TabHardware))
                            .width(Length::Fill)
                            .padding(Padding::new(20.0))
                    } else {
                        button("Hardware")
                            .style(|t, _| Styles::tab_unselected(t))
                            .on_press(window::Message::UIUpdate(UIMessage::TabHardware))
                            .width(Length::Fill)
                            .padding(Padding::new(20.0))
                    },
                ]    
            ).width(Length::FillPortion(1))
                .height(Length::Fill)
                .style(|t| Styles::sidebar(t)),
            row![
                inner
            ].width(Length::FillPortion(4))
                .padding(Padding::new(20.0))
        ].width(Length::Fill)
            .into()
    }
}

pub struct Styles;

impl Styles {
    pub fn tab_selected(theme: &iced::Theme) -> iced_widget::button::Style {
        iced_widget::button::Style {
            background: Some(iced::Background::Color(theme.extended_palette().background.strong.color)),
            text_color: theme.extended_palette().background.strong.text,
            border: Border {
                color: theme.palette().success,
                width: 0.0,
                radius: Radius { top_left: 0.0, top_right: 0.0, bottom_right: 0.0, bottom_left: 0.0 }
            },
            shadow: Shadow {
                color: theme.palette().background,
                offset: Vector {
                    x: 0.0,
                    y: 1.0
                },
                blur_radius: 0.0
            },
            snap: false
        }
    }
    pub fn tab_unselected(theme: &iced::Theme) -> iced_widget::button::Style {
        iced_widget::button::Style {
            background: None,
            text_color: theme.extended_palette().background.weak.text,
            border: Border {
                color: theme.palette().success,
                width: 0.0,
                radius: Radius { top_left: 0.0, top_right: 0.0, bottom_right: 0.0, bottom_left: 0.0 }
            },
            shadow: Shadow {
                color: theme.palette().background,
                offset: Vector {
                    x: 0.0,
                    y: 1.0
                },
                blur_radius: 0.0
            },
            snap: false
        }
    }

    pub fn sidebar(theme: &iced::Theme) -> iced_widget::container::Style {
        iced_widget::container::Style {
            text_color: Some(theme.extended_palette().background.weak.text),
            background: Some(iced::Background::Color(theme.extended_palette().background.weak.color)),
            border: Border { 
                color: theme.extended_palette().background.weak.color,
                width: 0.0, 
                radius: Radius::new(0.0) 
            },
            shadow: Shadow { 
                color: theme.extended_palette().background.weak.color,
                offset: iced::Vector::ZERO, 
                blur_radius: 0.0
            },
            snap: false,
        }
    }

    pub fn ps_column_header_button(theme: &iced::Theme) -> iced_widget::button::Style {
        iced_widget::button::Style {
            background: None,
            text_color: theme.extended_palette().background.base.text,
            border: Border {
                color: theme.palette().success,
                width: 0.0,
                radius: Radius { top_left: 0.0, top_right: 0.0, bottom_right: 0.0, bottom_left: 0.0 }
            },
            shadow: Shadow {
                color: theme.palette().background,
                offset: Vector {
                    x: 0.0,
                    y: 1.0
                },
                blur_radius: 0.0
            },
            snap: false
        }
    }

    pub fn ps_dropdown_button(theme: &iced::Theme, status: iced::widget::button::Status) -> iced_widget::button::Style {
        match status {
            iced::widget::button::Status::Hovered => iced_widget::button::Style {
                background: Some(iced::Background::Color(theme.extended_palette().background.weak.color)),
                text_color: theme.extended_palette().background.base.text,
                border: Border {
                    color: theme.palette().success,
                    width: 0.0,
                    radius: Radius { top_left: 0.0, top_right: 0.0, bottom_right: 0.0, bottom_left: 0.0 }
                },
                shadow: Shadow {
                    color: theme.palette().background,
                    offset: Vector {
                        x: 0.0,
                        y: 1.0
                    },
                    blur_radius: 0.0
                },
                snap: false
            },
            _ => iced_widget::button::Style {
                background: Some(iced::Background::Color(theme.extended_palette().background.base.color)),
                text_color: theme.extended_palette().background.base.text,
                border: Border {
                    color: theme.palette().success,
                    width: 0.0,
                    radius: Radius { top_left: 0.0, top_right: 0.0, bottom_right: 0.0, bottom_left: 0.0 }
                },
                shadow: Shadow {
                    color: theme.palette().background,
                    offset: Vector {
                        x: 0.0,
                        y: 1.0
                    },
                    blur_radius: 0.0
                },
                snap: false
            }
        }
    }

    pub fn ps_unselected_row(theme: &iced::Theme) -> iced_widget::button::Style {
        iced_widget::button::Style {
            background: None,
            text_color: theme.extended_palette().background.base.text,
            border: Border {
                color: theme.palette().success,
                width: 0.0,
                radius: Radius { top_left: 0.0, top_right: 0.0, bottom_right: 0.0, bottom_left: 0.0 }
            },
            shadow: Shadow {
                color: theme.palette().background,
                offset: Vector {
                    x: 0.0,
                    y: 1.0
                },
                blur_radius: 0.0
            },
            snap: false
        }
    }

    pub fn ps_selected_row(theme: &iced::Theme) -> iced_widget::button::Style {
        iced_widget::button::Style {
            background: Some(iced::Background::Color(theme.extended_palette().warning.weak.color)),
            text_color: theme.extended_palette().warning.weak.text,
            border: Border {
                color: theme.palette().success,
                width: 0.0,
                radius: Radius { top_left: 0.0, top_right: 0.0, bottom_right: 0.0, bottom_left: 0.0 }
            },
            shadow: Shadow {
                color: theme.palette().background,
                offset: Vector {
                    x: 0.0,
                    y: 1.0
                },
                blur_radius: 0.0
            },
            snap: false
        }
    }

    pub fn enabled_button(theme: &iced::Theme) -> iced_widget::button::Style {
        iced_widget::button::Style {
            background: Some(iced::Background::Color(theme.extended_palette().primary.base.color)),
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
                    y: 1.0
                },
                blur_radius: 0.0
            },
            snap: false
        }
    }

    pub fn disabled_button(theme: &iced::Theme) -> iced_widget::button::Style {
        iced_widget::button::Style {
            background: Some(iced::Background::Color(theme.extended_palette().background.weak.color)),
            text_color: theme.extended_palette().primary.weak.text,
            border: Border {
                color: theme.palette().success,
                width: 0.0,
                radius: Radius { top_left: 0.0, top_right: 0.0, bottom_right: 0.0, bottom_left: 0.0 }
            },
            shadow: Shadow {
                color: theme.palette().background,
                offset: Vector {
                    x: 0.0,
                    y: 1.0
                },
                blur_radius: 0.0
            },
            snap: false
        }
    }
}