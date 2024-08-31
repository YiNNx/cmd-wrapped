use clap::{arg, command, value_parser, Arg};
use std::env;

pub struct Cli {
    pub year: i32,
    pub shell: String,
}

impl Cli {
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
                    .help("Display statistics for the specified year")
                    .value_parser(value_parser!(i32)),
            )
            .arg(
                arg!(
                -s --shell <SHELL> "Specify the target shell / history tool.\nSupported options - zsh, bash, fish, atuin"
                )
                .required(false),
            )
            .get_matches();

        let year = args
            .get_one::<i32>("year")
            .map(|ptr| ptr.to_owned())
            .unwrap_or_default();
        let shell = args
            .get_one::<String>("shell")
            .map(|ptr| ptr.to_owned())
            .unwrap_or(Self::current_shell());

        Cli { year, shell }
    }
}
