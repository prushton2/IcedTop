use iced::{Length, Padding};
use iced_widget::{Column, text, row, scrollable};
use std::cmp::Ordering;

use crate::window::{Message, modules::ui_state::Styles};

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
    pub processes: Vec<Process>
}

impl Ps {
    pub fn new() -> Self {
        Self {
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
            
        }
    }

    pub fn view(&self, sort_by: PsViewSortBy) -> iced::Element<'_, Message> {
        let mut processes: Vec<&Process> = self.processes.iter().map(|e| e).collect();
        
        processes.sort_by(|a, b| {
          match sort_by {
            PsViewSortBy::Cpu => order_from_bool(a.cpu < b.cpu),
            PsViewSortBy::Mem => order_from_bool(a.mem < b.mem),
            PsViewSortBy::User => order_from_bool(a.user < b.user),
            _ => order_from_bool(a.pid > b.pid)
          }
        });
        
        let mut elements_vec: Vec<iced::Element<'_, Message>> = processes.iter().filter(|e| e.app != "ps aux").map(|e| {
            row![
                text(&e.user)                 .width(Length::FillPortion(1)),
                text(&e.pid)                  .width(Length::FillPortion(1)),
                row![text(&e.cpu), text("%")] .width(Length::FillPortion(1)),
                row![text(&e.mem), text("%")] .width(Length::FillPortion(1)),
                text(&e.app)                  .width(Length::FillPortion(3)).wrapping(text::Wrapping::None)
            ]
            // .width(Length::Fill)
            .into()
        }).collect();

        elements_vec.insert(0, row![
            text("Owner")  .width(Length::FillPortion(1)),
            text("PID")    .width(Length::FillPortion(1)),
            text("CPU %")  .width(Length::FillPortion(1)),
            text("MEM %")  .width(Length::FillPortion(1)),
            text("Program").width(Length::FillPortion(3)),
        ]
        .padding(Padding::new(0.0).bottom(10.0))
        .into());

        scrollable(
            Column::from_vec(elements_vec)
                .width(Length::Fill)
                .padding(Padding::new(20.0))
        )
            .style(|t, _| Styles::processes_inner_box(t))
            .into()
    }
}

pub enum PsViewSortBy {
    User,
    Pid,
    Cpu,
    Mem,
    Command,
}

fn order_from_bool(bool: bool) -> Ordering {
    if bool {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}