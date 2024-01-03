use chrono::{DateTime, Datelike, Local, NaiveDate, Timelike};
use colored::*;
use num_traits::cast::FromPrimitive;
use std::{collections::HashMap, env};

use crate::{parser::Command, view::View};

const FAV_COMMANDS_IGNORE: &[&str] = &[
    // "cd", "ll", "ls", "mv", "cp", "rm", "cat", "less", "mkdir", "history", "",
];

#[derive(Default)]
pub struct Statistic {
    year: i32,

    list_daytime: Vec<usize>,
    list_weekday: Vec<usize>,
    list_day: Vec<usize>,
    list_month: Vec<usize>,
    list_month_total: Vec<usize>,

    map_command: HashMap<String, usize>,
    map_command_total: HashMap<String, usize>,

    command_count: usize,
    command_count_total: usize,

    first_command: String,
    first_command_time: DateTime<Local>,
}

impl Statistic {
    pub fn new() -> Statistic {
        let now = Local::now();
        let mut year = now.year();
        if now.month() <= 3 {
            year -= 1;
        }
        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            if let Ok(year_arg) = args[1].parse::<i32>() {
                year = year_arg;
            }
        }
        Statistic {
            year,
            first_command_time: Local::now(),
            list_daytime: vec![0; 24],
            list_weekday: vec![0; 7],
            list_month: vec![0; 12],
            list_month_total: vec![0; 12],
            list_day: vec![0; 365],
            ..Default::default()
        }
    }

    pub fn analyze(&mut self, c: &Command) {
        self.map_command_total
            .entry(c.command.clone())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        self.command_count_total += 1;

        if let Some(time) = c.time {
            let month = time.month0() as usize;
            self.list_month_total[month] += 1;

            if time.year() != self.year {
                return;
            }

            let hour = time.hour() as usize;
            self.list_daytime[hour] += 1;

            let weekday = time.weekday() as usize;
            self.list_weekday[weekday] += 1;

            self.list_month[month] += 1;

            let day = time.ordinal0() as usize;
            self.list_day[day] += 1;
            self.command_count += 1;

            if self.first_command_time > time {
                self.first_command = c.commandline.clone();
                self.first_command_time = time;
            }

            self.map_command
                .entry(c.command.clone())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }

    pub fn most_active_period(&self) -> (&str, usize) {
        let boundaries = vec![0, 6, 11, 14, 19, 24];

        let time_periods: Vec<usize> = boundaries
            .windows(2)
            .map(|window| {
                self.list_daytime[window[0]..window[1]]
                    .iter()
                    .sum::<usize>()
                    / (window[1] - window[0])
            })
            .collect();

        let (period, val) = time_periods
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item)
            .unwrap();

        (
            match period {
                0 => "Late Night",
                1 => "Morning",
                2 => "Noon",
                3 => "Afternoon",
                4 => "Evening",
                _ => "",
            },
            *val,
        )
    }

    pub fn most_active_weekday(&self) -> (usize, usize) {
        self.list_weekday
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item)
            .map(|(k, v)| (k, *v))
            .unwrap()
    }

    pub fn most_active_month(&self) -> (usize, usize) {
        self.list_month
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item)
            .map(|(i, v)| (i, *v))
            .unwrap()
    }

    pub fn output(&self) {
        View::display_title(self.year);

        View::sub_title(&format!(
            "Commands - {}",
            self.command_count.to_string().bold().italic().underline()
        ));
        View::content(&format!(
            "- In {}, you entered the very first command `{}` on {} at {}.\n",
            self.year,
            self.first_command.to_string().cyan().bold(),
            self.first_command_time
                .format("%m-%d")
                .to_string()
                .cyan()
                .bold(),
            self.first_command_time
                .format("%H:%M")
                .to_string()
                .cyan()
                .bold(),
        ));

        View::content(&format!(
            "- Throughout the year, a total of {} commands were entered. (Total in history: {})\n",
            self.command_count.to_string().cyan().bold(),
            self.command_count_total.to_string().cyan().bold()
        ));

        let (day, max) = self
            .list_day
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item)
            .unwrap();
        View::content(&format!(
            "- On {}, a peak of {} commands were entered in a single day.",
            NaiveDate::from_ymd_opt(self.year, 1, 1)
                .unwrap()
                .with_ordinal0(day as u32)
                .unwrap()
                .to_string()
                .cyan()
                .bold(),
            max.to_string().cyan().bold(),
        ));

        View::sub_title(&format!("Command Graph {}", self.year));
        View::typewriter_for_line(&View::graph(&self.list_day));

        let (most_active_month, max) = self.most_active_month();
        View::sub_title(&format!(
            "Most Active Month - {}",
            chrono::Month::from_u32((most_active_month + 1) as u32)
                .unwrap()
                .name()
                .bold()
                .italic()
                .underline()
        ));
        let gap = max / 80 + 1;
        for (month, &count) in self.list_month.iter().enumerate() {
            if count == 0 {
                continue;
            }
            View::content(&format!(
                "{:<125}{}",
                &format!(
                    "{} {}| {:<5}",
                    chrono::Month::from_u32((month + 1) as u32).unwrap().name()[0..3].bold(),
                    "#".repeat(count / gap).dimmed().bold(),
                    count.to_string().bold()
                ),
                format!("[{:<4} total]", self.list_month_total[month],).bright_black()
            ));
        }

        View::wait();

        let (most_active_weekday, max) = self.most_active_weekday();
        let str_weekday = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
        View::sub_title(&format!(
            "Most Active Weekday - {}",
            str_weekday[most_active_weekday]
                .to_string()
                .italic()
                .underline()
        ));
        let gap = max / 90 + 1;
        for (weekday, &count) in self.list_weekday.iter().enumerate() {
            View::content(&format!(
                "{} {}| {:<5}",
                str_weekday[weekday].bold(),
                "#".repeat(count / gap).dimmed().bold(),
                if count == max {
                    count.to_string().green().bold()
                } else {
                    count.to_string().bold()
                },
            ));
        }

        let (most_active_time, _) = self.most_active_period();
        View::sub_title(&format!(
            "Most Active Time - {}",
            most_active_time.to_string().italic().underline()
        ));

        let start = 7;
        let max = *self.list_daytime.iter().max().unwrap();
        let gap = max / 90 + 1;
        for i in 0..self.list_daytime.len() {
            let index = (start + i) % self.list_daytime.len();
            let count = self.list_daytime[index];
            View::content(&format!(
                "{:<2}  {}| {}",
                index.to_string().bold(),
                "#".repeat(count / gap).dimmed().bold(),
                if count == max {
                    count.to_string().green().bold()
                } else {
                    count.to_string().bold()
                },
            ));
        }
        View::wait();

        View::sub_title("Favorite Commands");

        let mut fav_command: Vec<_> = self.map_command.iter().collect();
        fav_command.sort_by(|a, b| b.1.cmp(&a.1));
        for (command, &count) in fav_command
            .iter()
            .filter(|(command, _)| !FAV_COMMANDS_IGNORE.contains(&command.as_str()))
            .take(10)
        {
            View::content(&format!(
                "- {:<50} {:<6}{}",
                command.green().bold(),
                count,
                format!(
                    "[{:<4} total]",
                    self.map_command_total.get(*command).unwrap(),
                )
                .bright_black()
            ));
        }

        View::sub_title("Also Frequently Used");

        for (command, count) in fav_command
            .iter()
            .filter(|(command, _)| !FAV_COMMANDS_IGNORE.contains(&command.as_str()))
            .skip(10)
            .take(15)
        {
            View::content(&format!(
                "- {:<50} {:<6}{}",
                command.green().bold(),
                count,
                format!(
                    "[{:<4} total]",
                    self.map_command_total.get(*command).unwrap(),
                )
                .bright_black()
            ));
        }
        View::content("...");

        View::wait();

        View::sub_title(&format!("All {} command line history wrapped!", self.year));

        View::typewriter_for_line(
            &(String::new()
                + "Specify other years with arguments, such as `./cmd-wrapped 2022`\n\n"
                + &format!(
                    "If you enjoy this open-source CLI, give it a star:  {}",
                    "https://github.com/YiNNx/cmd-wrapped\n\n"
                        .bold()
                        .to_string()
                        + "Also feel free to submit ideas or issues! :-D"
                )),
        );
        View::wait();
    }
}
