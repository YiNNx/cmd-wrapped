use chrono::{Datelike, Local};
use clap::{arg, command, value_parser, Arg, Command};
use std::env;

pub struct Cli {}

impl Cli {
    pub fn new() -> Command {
        command!()
        .arg(
            Arg::new("year")
                .required(false)
                .help("Specify the year")
                .value_parser(value_parser!(i32)),
        )
        .arg(
            arg!(
            -s --shell <SHELL> "Specify the history provider.\nSupported options - zsh, bash, fish, atuin"
            )
            .required(false),
        )
        .subcommand(
            Command::new("set")
                .about("Set multiple history provider")
                .arg(
                    Arg::new("provider")
                    .required(true)
                    .help("Add multiple history provider")
                    .value_parser(value_parser!(String)),
                )
                .arg(
                    arg!(
                        -f --file <FILE> "Set custom history file for the provider"
                        )
                        .required(false),
                )
                .arg_required_else_help(true),
        )
    }
}

pub struct Query {
    pub year: i32,
    pub shell: String,
}

impl Query {
    pub fn default_year() -> i32 {
        let now = Local::now();
        let year = now.year();
        return if now.month() <= 3 { year - 1 } else { year };
    }

    pub fn current_shell() -> String {
        env::var("SHELL")
            .unwrap_or("unknown".into())
            .split('/')
            .last()
            .unwrap_or("unknown")
            .into()
    }
}

pub struct Setting {
    pub provider: String,
    pub history_file: Option<String>,
}

pub struct Args {
    pub query: Option<Query>,
    pub setting: Option<Setting>,
}

impl Args {
    pub fn parse() -> Self {
        let args = Cli::new().get_matches();

        if let Some((sub_command, sub_args)) = args.subcommand() {
            match sub_command {
                "set" => {
                    let provider = sub_args.get_one::<String>("provider").unwrap().to_owned();
                    let history_file = sub_args.get_one::<String>("file").map(|ptr| ptr.to_owned());
                    return Args {
                        query: None,
                        setting: Some(Setting {
                            provider,
                            history_file,
                        }),
                    };
                }
                _ => unreachable!(),
            }
        } else {
            let year = *args
                .get_one::<i32>("year")
                .unwrap_or(&Query::default_year());
            let shell = args
                .get_one::<String>("shell")
                .map(|ptr| ptr.to_owned())
                .unwrap_or(Query::current_shell());
            return Args {
                query: Some(Query { year, shell }),
                setting: None,
            };
        }
    }
}
