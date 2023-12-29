use chrono::{Datelike, Timelike, Utc};
use colored::*;
use num_traits::cast::FromPrimitive;
use std::collections::HashMap;

use crate::{parser::Command, view::View};

const FAV_COMMANDS_IGNORE: &[&str] = &[
    "cd", "ll", "ls", "mv", "cp", "rm", "cat", "less", "mkdir", "history", "",
];

pub struct Statistic {
    daytime_counts: Vec<usize>,
    month_counts: Vec<usize>,
    fav_counts: HashMap<String, usize>,
}

impl Statistic {
    pub fn new() -> Statistic {
        Statistic {
            daytime_counts: vec![0; 24],
            month_counts: vec![0; 12],
            fav_counts: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, c: &Command) {
        if c.time.year() != Utc::now().year() {
            return;
        }
        let hour = c.time.hour() as usize;
        self.daytime_counts[hour] += 1;

        let month = c.time.month0() as usize;
        self.month_counts[month] += 1;

        self.fav_counts
            .entry(c.command.clone())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    pub fn most_active_time(&self) -> (&str, usize) {
        let boundaries = vec![0, 6, 11, 14, 19, 24];

        let time_periods: Vec<usize> = boundaries
            .windows(2)
            .map(|window| {
                self.daytime_counts[window[0]..window[1]]
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
        self.month_counts
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item)
            .map(|(i, v)| (i, *v))
            .unwrap()
    }

    pub fn output(&self) {
        View::display_title();

        let (most_active_time, max) = self.most_active_time();
        View::sub_title(
            format!(
                "Most Active Time - {}",
                most_active_time.to_string().italic().underline()
            )
            .as_str(),
        );

        let start = 7;
        let gap = max / 40;
        for i in 0..self.daytime_counts.len() {
            let index = (start + i) % self.daytime_counts.len();
            let count = self.daytime_counts[index];
            View::content(
                format!(
                    "{:<2}  {}| {}",
                    index.to_string().bold(),
                    "#".repeat(count / gap),
                    count
                )
                .as_str(),
            );
        }

        View::wait();

        let (most_active_month, max) = self.most_active_month();
        View::sub_title(
            format!(
                "Most Active Month - {}",
                chrono::Month::from_u32((most_active_month + 1) as u32)
                    .unwrap()
                    .name()
                    .bold()
                    .italic()
                    .underline()
            )
            .as_str(),
        );
        let gap = max / 45;
        for (month, &count) in self.month_counts.iter().enumerate() {
            View::content(
                format!(
                    "{:<9} {}| {:<5}\n",
                    chrono::Month::from_u32((month + 1) as u32)
                        .unwrap()
                        .name()
                        .bold(),
                    "#".repeat(count / gap),
                    count,
                )
                .as_str(),
            );
        }

        View::wait();

        View::sub_title("Favorite Commands");

        let mut fav_command: Vec<_> = self.fav_counts.iter().collect();
        fav_command.sort_by(|a, b| b.1.cmp(&a.1));

        for (command, &count) in fav_command
            .iter()
            .filter(|(command, _)| !FAV_COMMANDS_IGNORE.contains(&command.as_str()))
            .take(10)
        {
            View::content(format!("- {:<8} {:<4} times", command.green().bold(), count,).as_str());
        }
        View::wait();
    }
}
