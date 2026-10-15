use iced::Length;
use iced_widget::{button, row, scrollable, table, text};
use iced_aw::menu::{self, Item};
use std::{cmp::Ordering, collections::{HashMap, HashSet}};

use crate::window::modules::ui_state::Styles;
use super::{Delta, Process, StatReading};

pub struct Ps {
    pub reversed: bool,
    pub sort_by: PsViewSortBy,
    pub keyed_processes: HashMap<u32, Process>,
    clock_ticks: u64,
    page_size: u64,
}

impl Ps {
    pub fn new() -> Self {
        Self {
            reversed: false,
            sort_by: PsViewSortBy::Cpu,
            keyed_processes: [].into(),
            clock_ticks: unsafe { libc::sysconf(libc::_SC_CLK_TCK) as u64 },
            page_size: unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as u64
        }
    }

    pub fn fetch(&mut self) {
        let proc = std::fs::read_dir("/proc").unwrap();

        let mut running_processes: HashSet<u32> = [].into();

        for process in proc.flatten() {
            // Some entries are non-processes, so ignore them
            let Some(_) = process.file_name().to_str().and_then(|s| s.parse::<u32>().ok()) else {
                continue;
            };

            // The process may have exited since read_dir
            let Ok(stat) = std::fs::read_to_string(process.path().join("stat")) else {
                continue;
            };
            let stat_reading: StatReading = stat.into();
            running_processes.insert(stat_reading.pid);

            let kernel_time_millis = stat_reading.stime * 1000 / self.clock_ticks;
            let user_time_millis   = stat_reading.utime * 1000 / self.clock_ticks;

            let process: &mut Process = self.keyed_processes.entry(stat_reading.pid).or_default();

            // New process, or if the PID was reused by a different process, then restart
            if process.pid != stat_reading.pid || process.start_time != stat_reading.starttime {
                *process = Process {
                    start_time: stat_reading.starttime,
                    kernel_time_millis: Delta::new(kernel_time_millis),
                    user_time_millis: Delta::new(user_time_millis),
                    ..Default::default()
                };
            } else {
                process.kernel_time_millis.push(kernel_time_millis);
                process.user_time_millis  .push(user_time_millis);
            }

            process.name = stat_reading.comm;
            process.pid = stat_reading.pid;
            process.pages_used = stat_reading.rss as u64;
        }

        self.keyed_processes.retain(|pid, _| running_processes.contains(pid));
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        let mut processes: Vec<&Process> = self.keyed_processes.iter().map(|(_, e)| e).collect();
        
        processes.sort_by(|a, b| {
          match self.sort_by {
            PsViewSortBy::App =>  order_from_bool(a.name < b.name),
            PsViewSortBy::Cpu =>  order_from_bool(a.cpu_pct(1000) < b.cpu_pct(1000)),
            PsViewSortBy::Mem =>  order_from_bool(a.pages_used < b.pages_used),
            PsViewSortBy::Pid =>  order_from_bool(a.pid > b.pid),
            _ =>  order_from_bool(a.cpu_pct(1000) < b.cpu_pct(1000)),
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
                    |e: &Process| Self::process_context_menu(&e.name, e.pid)),
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
                    |e: &Process| row![text(&e.cpu_pct(1000)), text("%")])
                    .align_x(iced::alignment::Horizontal::Right),
                table::column(
                    row![
                        button("Memory")
                            .style(|t, _| Styles::ps_column_header_button(t))
                            .on_press(Message::UpdateSortBy(PsViewSortBy::Mem)),
                        self.sort_label(PsViewSortBy::Mem)
                    ],
                    |e: &Process| {
                        let mem = e.mem_h(self.page_size);
                        row![text(mem.0), text(" "), text(mem.1)]
                    })
                    .align_x(iced::alignment::Horizontal::Right),
            ],
            processes
        );

        scrollable(elements.width(Length::Fill)).spacing(4).into()
        // text("Empty").into()
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