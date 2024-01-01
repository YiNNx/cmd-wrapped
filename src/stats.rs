use chrono::{DateTime, Datelike, Local, NaiveDate, Timelike};
use colored::*;
use num_traits::cast::FromPrimitive;
use std::collections::HashMap;

use crate::{parser::Command, view::View};

const FAV_COMMANDS_IGNORE: &[&str] = &[
    // "cd", "ll", "ls", "mv", "cp", "rm", "cat", "less", "mkdir", "history", "",
];

#[derive(Default)]
pub struct Statistic {
    daytime_count_list: Vec<usize>,
    month_count_list: Vec<usize>,
    fav_counts: HashMap<String, usize>,
    command_counts: HashMap<String, usize>,
    graph: Vec<usize>,
    year_command_count: usize,
    total_command_count: usize,
    first_command_time: DateTime<Local>,
    first_command_day: usize,
    first_command: String,
}

impl Statistic {
    pub fn new() -> Statistic {
        Statistic {
            daytime_count_list: vec![0; 24],
            month_count_list: vec![0; 12],
            graph: vec![0; 365],
            ..Default::default()
        }
    }

    pub fn analyze(&mut self, c: &Command) {
        self.command_counts
            .entry(c.command.clone())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        self.total_command_count += 1;

        if let Some(time) = c.time {
            if time.year() != 2023 {
                return;
            }

            let hour = time.hour() as usize;
            self.daytime_count_list[hour] += 1;

            let month = time.month0() as usize;
            self.month_count_list[month] += 1;

            let day = time.ordinal0() as usize;
            self.graph[day] += 1;
            self.year_command_count += 1;

            if self.first_command_day == 0 || self.first_command_day > day {
                self.first_command_day = day;
                self.first_command = c.commandline.clone();
                self.first_command_time = time;
            }

            self.fav_counts
                .entry(c.command.clone())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }

    pub fn most_active_time(&self) -> (&str, usize) {
        let boundaries = vec![0, 6, 11, 14, 19, 24];

        let time_periods: Vec<usize> = boundaries
            .windows(2)
            .map(|window| {
                self.daytime_count_list[window[0]..window[1]]
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
                4 => "Night",
                _ => "",
            },
            *val,
        )
    }

    pub fn most_active_month(&self) -> (usize, usize) {
        self.month_count_list
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item)
            .map(|(i, v)| (i, *v))
            .unwrap()
    }

    pub fn output(&self) {

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
        let gap = max / 90 + 1;
        for (month, &count) in self.month_count_list.iter().enumerate() {
            View::content(&format!(
                "{:<9} {}| {:<5}",
                chrono::Month::from_u32((month + 1) as u32)
                    .unwrap()
                    .name()
                    .bold(),
                "#".repeat(count / gap),
                count,
            ));
        }
        
        View::sub_title("Command Graph");
        View::typewriter_for_line(&self.graph());

        View::line_break();
        View::content(
            &format!(
                "- Your First Command in 2023 happens in {}. It is `{}`.\n",
                self.first_command_time.to_string().cyan(),
                self.first_command.to_string().cyan()
            )
            .white(),
        );
        View::content(
            &format!(
                "- Command Count in the year - {}   ({} totally in the past)\n",
                self.year_command_count.to_string().cyan(),
                self.total_command_count.to_string().cyan()
            )
            .white(),
        );

        let (day, max) = self
            .graph
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item)
            .unwrap();
        View::content(
            &format!(
                "- The Max Count in one day is {}  ({})",
                max.to_string().cyan(),
                NaiveDate::from_ymd_opt(2023, 1, 1)
                    .unwrap()
                    .with_ordinal0(day as u32)
                    .unwrap().to_string().cyan(),
            )
            .white(),
        );


        View::wait();


        let (most_active_time, _) = self.most_active_time();
        View::sub_title(&format!(
            "Most Active Time - {}",
            most_active_time.to_string().italic().underline()
        ));

        let start = 7;
        let gap = self.daytime_count_list.iter().max().unwrap() / 90 + 1;
        for i in 0..self.daytime_count_list.len() {
            let index = (start + i) % self.daytime_count_list.len();
            let count = self.daytime_count_list[index];
            View::content(&format!(
                "{:<2}  {}| {}",
                index.to_string().bold(),
                "#".repeat(count / gap),
                count
            ));
        }
        View::wait();

        View::sub_title("Favorite Commands");

        let mut fav_command: Vec<_> = self.fav_counts.iter().collect();
        fav_command.sort_by(|a, b| b.1.cmp(&a.1));
        View::content(&format!(
            "[{}]    [{}] / [{}]",
            "Command".green(),
            "2023".green(),
            "Total".green()
        ));
        for (command, &count) in fav_command
            .iter()
            .filter(|(command, _)| !FAV_COMMANDS_IGNORE.contains(&command.as_str()))
            .take(10)
        {
            View::content(
                format!(
                    "- {:<11} {:<5} /  {:<4}",
                    command.green().bold(),
                    count,
                    self.command_counts.get(*command).unwrap()
                )
                .as_str(),
            );
        }

        View::sub_title("Also Frequently Used");

        for (command, _) in fav_command
            .iter()
            .filter(|(command, _)| !FAV_COMMANDS_IGNORE.contains(&command.as_str()))
            .skip(10)
            .take(15)
        {
            View::content(&format!("- {:<8}", command.bold()));
        }
        View::content("...");

        View::wait();
    }

    pub fn graph(&self) -> String {
        let mut res = format!(" {}\n", "―".repeat(110))
        +&format!("│  Jan.      Feb.    Mar.    Apr.      May     Jun.    Jul.      Aug.    Sep.    Oct.      Nov.    Dec.        │\n");
        for i in 0..=6 {
            res += &format!("│ ");
            for j in 0..=52 {
                let ordinal = i + j * 7;
                if ordinal >= 365 {
                    res += "  "
                } else {
                    res += &format!(
                        "{:>2}",
                        match self.graph[ordinal] {
                            0 => "  ".white(),
                            1..=30 => "▩".cyan(),
                            _ => "▩".cyan().bold(),
                        }
                    )
                }
            }
            res += &format!("   │\n");
        }
        res += &format!(" {}", "_".repeat(110));
        res
    }
}
