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

    pub fn output(&self) {
        View::display_title();

        View::sub_title("Most Active Time");

        for (period, &count) in self.daytime_counts.iter().enumerate() {
            View::content(format!("{:<2} {}| {}", period, "#".repeat(count / 20), count,).as_str());
        }
        View::wait();

        // cyan_println("Most Active Day");
        View::sub_title("Most Active Month");

        for (month, &count) in self.month_counts.iter().enumerate() {
            View::content(
                format!(
                    "{:<9} - {}| {:<5}",
                    chrono::Month::from_u32((month + 1) as u32).unwrap().name(),
                    "#".repeat(count / 20),
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
            View::content(
                format!(
                    "{:<10} - {}| {}",
                    command.cyan(),
                    "#".repeat(count / 20),
                    count,
                )
                .as_str(),
            );
        }
        View::wait();
    }
}
