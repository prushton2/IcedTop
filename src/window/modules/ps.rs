use iced::Length;
use iced_widget::{button, row, scrollable, table, text};
use iced_aw::menu::{self, Item};
use std::cmp::Ordering;

use crate::window::modules::ui_state::Styles;

#[derive(Clone, Debug)]
pub struct Process {
    pub user: String,
    pub pid: u32,
    pub cpu: f32,
    pub mem: f32,
    pub command: String,
    pub app: String,
}

pub struct Ps {
    pub reversed: bool,
    pub sort_by: PsViewSortBy,
    pub processes: Vec<Process>
}

impl Ps {
    pub fn new() -> Self {
        Self {
            reversed: false,
            sort_by: PsViewSortBy::Cpu,
            processes: vec![]
        }
    }

    pub fn fetch(&mut self) {
        let command = std::process::Command::new("ps")
            .arg("aux")
            .output()
            .unwrap();

        let str = String::from_utf8_lossy(&command.stdout[..]);
        let mut lines = str.split("\n").collect::<Vec<&str>>();

        // drop first line because iti isnt a process
        let _ = lines.remove(0);

        self.processes = Vec::with_capacity(lines.len());

        for line in lines {
            if line == "" {
                continue;
            }

            let mut split = line.split(" ").filter(|e| *e != "");

            self.processes.push(Process {
                user:     split.next().unwrap().to_owned(),
                pid:      split.next().unwrap().parse::<u32>().unwrap(),
                cpu:      split.next().unwrap().parse::<f32>().unwrap(),
                mem:      split.next().unwrap().parse::<f32>().unwrap(),
                command:  {
                    let _ = split.nth(5);
                    split.collect::<Vec<&str>>().join(" ")
                },
                app: String::from("")
            });

            let command = self.processes.last().unwrap().command.clone();
            let len = self.processes.len();

            self.processes[len-1].app = command.split(" ").nth(0).unwrap().split("/").last().unwrap().to_owned();
            
            if self.processes[len-1].app == "ps" {
                self.processes.remove(len-1);
            }
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        let mut processes: Vec<&Process> = self.processes.iter().map(|e| e).collect();
        
        processes.sort_by(|a, b| {
          match self.sort_by {
            PsViewSortBy::App =>  order_from_bool(a.app < b.app),
            PsViewSortBy::Cpu =>  order_from_bool(a.cpu < b.cpu),
            PsViewSortBy::Mem =>  order_from_bool(a.mem < b.mem),
            PsViewSortBy::User => order_from_bool(a.user < b.user),
            PsViewSortBy::Pid =>  order_from_bool(a.pid > b.pid)
          }
        });

        if self.reversed {
            processes.reverse();
        }

        let elements: table::Table<'_, Message, iced::Theme, iced::Renderer> = table(
            vec![
                table::column(
                    row![
                        button("Program")
                            .style(|t, _| Styles::ps_column_header_button(t))
                            .on_press(Message::UpdateSortBy(PsViewSortBy::App)),
                        self.sort_label(PsViewSortBy::App)
                    ],
                    |e: &Process| Self::process_context_menu(&e.app, e.pid)),
                table::column(
                    row![
                        button("PID")
                            .style(|t, _| Styles::ps_column_header_button(t))
                            .on_press(Message::UpdateSortBy(PsViewSortBy::Pid)),
                        self.sort_label(PsViewSortBy::Pid)
                    ],
                    |e: &Process| text(&e.pid)),
                table::column(
                    row![
                        button("CPU")
                            .style(|t, _| Styles::ps_column_header_button(t))
                            .on_press(Message::UpdateSortBy(PsViewSortBy::Cpu)),
                        self.sort_label(PsViewSortBy::Cpu)
                    ],
                    |e: &Process| row![text(&e.cpu), text("%")]),
                table::column(
                    row![
                        button("Memory")
                            .style(|t, _| Styles::ps_column_header_button(t))
                            .on_press(Message::UpdateSortBy(PsViewSortBy::Mem)),
                        self.sort_label(PsViewSortBy::Mem)
                    ],
                    |e: &Process| row![text(&e.mem), text("%")]),
                table::column(
                    row![
                        button("Owner")
                            .style(|t, _| Styles::ps_column_header_button(t))
                            .on_press(Message::UpdateSortBy(PsViewSortBy::User)),
                        self.sort_label(PsViewSortBy::User)
                    ],
                    |e: &Process| text(&e.user)),
            ],
            processes
        );

        scrollable(elements.width(Length::Fill)).into()
    }

    fn process_context_menu<'a>(label: &'a str, pid: u32) -> iced::Element<'a, Message> {

        fn button_style(button: iced_widget::Button<'static, Message>) -> iced_widget::Button<'static, Message> {
            button.width(Length::Fill).style(|t, s| Styles::ps_dropdown_button(t, s))
        }
        
        menu::MenuBar::new(vec![
            Item::with_menu(text(label), menu::Menu::new(vec![
                menu::Item::new(button_style(button("Close"))     .on_press(Message::SendSignal(pid, "SIGTERM"))),
                menu::Item::new(button_style(button("Force Quit")).on_press(Message::SendSignal(pid, "SIGKILL"))),
                menu::Item::new(button_style(button("Pause"))     .on_press(Message::SendSignal(pid, "SIGSTOP"))),
                menu::Item::new(button_style(button("Resume"))    .on_press(Message::SendSignal(pid, "SIGCONT"))),
            ]).width(Length::Fixed(110.0)))
        ]).into()
    }

    fn sort_label(&self, read: PsViewSortBy) -> iced::Element<'static, Message> {
        if self.sort_by == read {
            if self.reversed {
                text("^").into()
            } else {
                text("v").into()
            }
        } else {
            text("").into()
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PsViewSortBy {
    User,
    Cpu,
    Mem,
    App,
    Pid
}

#[derive(Clone, Copy, Debug)]
pub enum Message {
    UpdateSortBy(PsViewSortBy),
    /// PID, Signal
    SendSignal(u32, &'static str)
}

fn order_from_bool(bool: bool) -> Ordering {
    if bool {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

pub fn update(this: &mut Ps, message: Message) {
    match message {
        Message::UpdateSortBy(new_sort) => {
            if this.sort_by == new_sort {
                this.reversed = !this.reversed
            } else {
                this.sort_by = new_sort;
                this.reversed = false;
            }
        },
        Message::SendSignal(pid, sig) => {
            let _ = std::process::Command::new("kill")
                .arg("-s")
                .arg(sig)
                .arg(format!("{}", pid))
                .output()
                .unwrap();
        }
    }
}