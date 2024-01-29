use chrono::{DateTime, Datelike, Local, NaiveDate, Timelike};
use num_traits::cast::FromPrimitive;
use std::collections::HashMap;

use crate::{
    parser::Command,
    view::{View, STR_WEEKDAY},
};

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
    pub fn from(year: i32) -> Statistic {
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
            let weekday = time.weekday() as usize;
            let day = time.ordinal0() as usize;

            self.list_daytime[hour] += 1;
            self.list_weekday[weekday] += 1;
            self.list_month[month] += 1;
            self.list_day[day] += 1;
            self.command_count += 1;

            if self.first_command_time > time {
                self.first_command = c.command_raw.clone();
                self.first_command_time = time;
            }

            self.map_command
                .entry(c.command.clone())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }

    pub fn most_active_period(&self) -> &str {
        let boundaries = [0, 6, 11, 14, 19, 24];

        let time_periods: Vec<usize> = boundaries
            .windows(2)
            .map(|window| {
                self.list_daytime[window[0]..window[1]]
                    .iter()
                    .sum::<usize>()
                    / (window[1] - window[0])
            })
            .collect();

        let (period, _) = time_periods
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item)
            .unwrap();

        match period {
            0 => "Late Night",
            1 => "Morning",
            2 => "Noon",
            3 => "Afternoon",
            4 => "Evening",
            _ => "",
        }
    }

    pub fn most_active_weekday(&self) -> (usize, usize) {
        self.list_weekday
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item)
            .map(|(k, v)| (k, *v))
            .unwrap_or_default()
    }

    pub fn most_active_month(&self) -> (usize, usize) {
        self.list_month
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item)
            .map(|(i, v)| (i, *v))
            .unwrap_or_default()
    }

    pub fn output(&self) {
        // Cover
        View::display_cover(self.year);

        // Basic Stats

        View::sub_title_with_keyword("Commands", self.command_count);

        View::content(&format!(
            "- In {}, you entered the very first command `{}` on {} at {}.\n",
            self.year,
            View::style_keyword(&self.first_command),
            View::style_keyword(self.first_command_time.format("%m-%d")),
            View::style_keyword(self.first_command_time.format("%H:%M")),
        ));
        View::content(&format!(
            "- Throughout the year, a total of {} commands were entered. (Total in history: {})\n",
            View::style_keyword(self.command_count),
            View::style_keyword(self.command_count_total)
        ));

        let (day, max) = self
            .list_day
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item)
            .unwrap();

        View::content(&format!(
            "- On {}, a peak of {} commands were entered in a single day.",
            View::style_keyword(
                NaiveDate::from_ymd_opt(self.year, 1, 1)
                    .unwrap_or_default()
                    .with_ordinal0(day as u32)
                    .unwrap_or_default()
            ),
            View::style_keyword(max),
        ));

        // Command Distribution Graph

        View::sub_title(&format!("Command Graph {}", self.year));

        View::typewriter_for_line(&View::graph(&self.list_day));

        // Most Active Month

        let (most_active_month, max) = self.most_active_month();

        View::sub_title_with_keyword(
            "Most Active Month",
            chrono::Month::from_u32((most_active_month + 1) as u32)
                .unwrap()
                .name(),
        );

        for (month, &count) in self.list_month.iter().enumerate() {
            View::histogram_with_total(
                &chrono::Month::from_u32((month + 1) as u32).unwrap().name()[0..3],
                count,
                self.list_month_total[month],
                max,
            )
        }
        View::wait();

        // Most Active Weekday

        let (most_active_weekday, max) = self.most_active_weekday();

        View::sub_title_with_keyword("Most Active Weekday", STR_WEEKDAY[most_active_weekday]);

        for (weekday, &count) in self.list_weekday.iter().enumerate() {
            View::histogram(STR_WEEKDAY[weekday], count, max)
        }

        // Most Active Time

        let most_active_time = self.most_active_period();
        View::sub_title_with_keyword("Most Active Time", most_active_time);

        let start = 7;
        let max = *self.list_daytime.iter().max().unwrap();
        for i in 0..self.list_daytime.len() {
            let index = (start + i) % self.list_daytime.len();
            View::histogram(index, self.list_daytime[index], max)
        }
        View::wait();

        // Favorite Commands

        View::sub_title("Favorite Commands");

        let mut fav_command: Vec<_> = self.map_command.iter().collect();
        fav_command.sort_by(|a, b| b.1.cmp(a.1));
        for (command, &count) in fav_command.iter().take(10) {
            View::display_count_and_total(
                command,
                count,
                *self.map_command_total.get(*command).unwrap(),
            );
        }

        View::sub_title("Also Frequently Used");

        for (command, &count) in fav_command.iter().skip(10).take(15) {
            View::display_count_and_total(
                command,
                count,
                *self.map_command_total.get(*command).unwrap(),
            );
        }
        View::content("...");
        View::wait();

        View::hint_finish(self.year);
        View::wait();
    }
}
