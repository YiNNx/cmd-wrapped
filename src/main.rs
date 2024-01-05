mod history;
mod parser;
mod stats;
mod view;

use std::env;

use chrono::{Datelike, Local};
use clap::{arg, command, value_parser, Arg};

use history::{History, Shell};
use parser::CommandParser;
use stats::Statistic;

pub struct Cli {
    year: i32,
    shell: String,
}

impl Cli {
    fn get_default_year() -> i32 {
        let now = Local::now();
        let year = now.year();
        return if now.month() <= 3 { year - 1 } else { year };
    }

    pub fn new() -> Self {
        let args = command!()
            .arg(Arg::new("year")
            .required(false)
            .value_parser(value_parser!(i32)))
            .arg(
                arg!(
                -s --shell <SHELL> "Set the shell path manually"
                )
                .required(false),
            )
            .get_matches();

        let year = *args
            .get_one::<i32>("year")
            .unwrap_or(&Self::get_default_year());

        let current_shell = match env::var("SHELL") {
            Ok(s) => s.split('/').last().unwrap_or_default().to_string(),
            _ => String::new(),
        };
        let shell = args.get_one::<String>("shell").unwrap_or(&current_shell);

        return Cli {
            year,
            shell: String::from(shell),
        };
    }
}

fn main() {
    let args = Cli::new();
    let mut stats = Statistic::from(args.year);
    let shell = Shell::from(&args.shell);
    let history = History::from(&shell).expect("failed to read history file");

    for block in history {
        CommandParser::from_raw(block)
            .parse(&shell)
            .unwrap_or_default()
            .finish()
            .iter()
            .for_each(|command| stats.analyze(command));
    }

    stats.output()
}
