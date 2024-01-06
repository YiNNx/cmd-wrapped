use chrono::{Datelike, Local};
use clap::{arg, command, value_parser, Arg};
use std::env;

pub struct Cli {
    pub year: i32,
    pub shell: String,
}

impl Cli {
    fn default_year() -> i32 {
        let now = Local::now();
        let year = now.year();
        return if now.month() <= 3 { year - 1 } else { year };
    }

    fn current_shell() -> String {
        env::var("SHELL")
            .unwrap_or("unknown".into())
            .split('/')
            .last()
            .unwrap_or("unknown")
            .into()
    }

    pub fn parse_or_default() -> Self {
        let args = command!()
            .arg(
                Arg::new("year")
                    .required(false)
                    .value_parser(value_parser!(i32)),
            )
            .arg(
                arg!(
                -s --shell <SHELL> "Set the shell path manually"
                )
                .required(false),
            )
            .get_matches();

        let year = *args.get_one::<i32>("year").unwrap_or(&Self::default_year());
        let shell = args
            .get_one::<String>("shell")
            .map(|ptr| ptr.to_owned())
            .unwrap_or(Self::current_shell());

        return Cli { year, shell };
    }
}
