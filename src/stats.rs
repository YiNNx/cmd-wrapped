use chrono::{Datelike, Timelike, Utc};
use std::collections::HashMap;

use crate::parser::Command;

const FAV_COMMANDS_IGNORE: &[&str] = &[
    "cd", "ll", "ls", "mv", "cp", "rm", "cat", "less", "mkdir", "history", "",
];

pub struct Statistic {
    day_period_counts: Vec<u32>,
    month_counts: Vec<u32>,
    fav_command_counts: HashMap<String, usize>,
}

impl Statistic {
    pub fn new() -> Statistic {
        Statistic {
            day_period_counts: vec![0; 24],
            month_counts: vec![0; 12],
            fav_command_counts: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, c: &Command) {
        if c.time.year() != Utc::now().year() {
            return;
        }
        let hour = c.time.hour() as usize;
        self.day_period_counts[hour] += 1;

        let month = c.time.month0() as usize;
        self.month_counts[month] += 1;

        self.fav_command_counts
            .entry(c.command.clone())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    pub fn output(&self) {
         println!("各个时间段活跃程度");

        for (period, count) in self.day_period_counts.iter().enumerate() {
            println!("{} - {}", period, count);
        }

        println!("命令月度使用情况：");

        for (month, count) in self.month_counts.iter().enumerate() {
            println!("{} - {}", month + 1, count);
        }
        let mut fav_command: Vec<_> = self.fav_command_counts.iter().collect();
        fav_command.sort_by(|a, b| b.1.cmp(&a.1));

        println!("最爱的命令：");

        for (command, count) in fav_command
            .iter()
            .filter(|(command, _)| !FAV_COMMANDS_IGNORE.contains(&command.as_str()))
            .take(10)
        {
            println!("{} - {}", command, count);
        }
    }
}
