use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, Timelike};
use num_traits::cast::FromPrimitive;
use std::collections::HashMap;

use crate::{
    parser::Command,
    view::{Component, View, STR_WEEKDAY},
};

#[derive(Default)]
pub struct Statistic {
    year: i32,

    list_daytime: Vec<usize>,
    list_daytime_today: Vec<usize>,
    list_weekday: Vec<usize>,
    list_day: Vec<usize>,
    list_month: Vec<usize>,
    list_month_total: Vec<usize>,

    map_command_total: HashMap<String, usize>,
    map_command_daily: HashMap<String, usize>,
    map_command_monthly: Vec<HashMap<String, usize>>,
    map_command_annual: HashMap<String, usize>,

    today_command_count: usize,
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
            list_daytime_today: vec![0; 24],
            list_weekday: vec![0; 7],
            list_month: vec![0; 12],
            list_month_total: vec![0; 12],
            list_day: vec![0; 366],
            map_command_monthly: vec![HashMap::new(); 12],
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
            let now = Local::now();
            let month = time.month0() as usize;
            self.list_month_total[month] += 1;

            if (self.year != 0 && time.year() != self.year)
                || (self.year == 0 && time.year() != now.year())
            {
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
                self.first_command.clone_from(&c.command_raw);
                self.first_command_time = time;
            }

            self.map_command_annual
                .entry(c.command.clone())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
            self.map_command_monthly[month]
                .entry(c.command.clone())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);

            let delta = if now.hour() < 6 { 18 } else { -6 };
            if self.year == 0 && (time + Duration::hours(delta)).ordinal0() == now.ordinal0() {
                self.today_command_count += 1;
                self.list_daytime_today[hour] += 1;
                self.map_command_daily
                    .entry(c.command.clone())
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
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

    pub fn output_annual(&self) {
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

        let mut fav_command: Vec<_> = self.map_command_annual.iter().collect();
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

    pub fn output_recent(&self) {
        let mut component = Component::new(61, 6, View::display);
        component.edge();
        component.break_line();

        component.content(&format!(
            "Today - {} commands / {} unique commands",
            self.today_command_count,
            self.map_command_daily.len()
        ));
        component.break_line();

        component.daytime_graph(&self.list_daytime_today);
        component.break_line();

        let mut fav_commands: Vec<_> = self.map_command_daily.iter().collect();
        fav_commands.sort_by(|a, b| b.1.cmp(a.1));
        let top_fav_commands: Vec<_> = fav_commands.iter().take(5).collect();

        let max = top_fav_commands
            .first()
            .map(|(_, b)| **b)
            .unwrap_or_default();
        let len_max = top_fav_commands
            .iter()
            .map(|(key, _)| key.len())
            .max()
            .unwrap_or_default();
        for (command, &count) in top_fav_commands {
            component.command_rank(command, count, max, len_max);
        }

        component.break_line();
        component.edge();
        component.padding(4);
        component.break_line();

        component.content(&format!(
            "This year - {} commands / {} unique commands",
            self.command_count,
            self.map_command_annual.len()
        ));
        component.break_line();

        component.graph2(&self.list_day);
        component.break_line();

        let month = Local::now().month0() as isize;
        for m in (month - 1..=month).rev() {
            if m < 0 {
                continue;
            }
            let mut monthly_commands: Vec<_> =
                self.map_command_monthly[m as usize].iter().collect();
            monthly_commands.sort_by(|a, b| b.1.cmp(a.1));
            let fav_commands: Vec<_> = monthly_commands.iter().take(3).cloned().collect();
            component.monthly_stat(
                m,
                self.list_month[m as usize],
                self.map_command_monthly[m as usize].len(),
                fav_commands,
                m != month,
            );
        }

        component.break_line();
        component.edge();
        println!()
    }
}
