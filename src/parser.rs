use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use regex::{Captures, Match, Regex};
use std::{
    error::Error,
    time::{Duration, UNIX_EPOCH},
};

use crate::history::HistoryProvider;

lazy_static::lazy_static! {
    static ref RE_ZSH_HISTORY: Regex = Regex::new(r": (\d+):(\d+);(.*)").unwrap();
    static ref RE_BASH_HISTORY: Regex = Regex::new(r"(\d+)\n((?:[^#\n]|\n)*)").unwrap();
    static ref RE_COMMAND: Regex = Regex::new(r"(?:\|\||&&)").expect("Invalid regex");
}

#[derive(Debug, Default)]
pub struct Command {
    pub commandline: String,
    pub time: Option<DateTime<Local>>,

    pub command: String,
    pub arguments: Vec<String>,
    // files: Vec<String>,
}

impl Command {
    fn from(commandline: String, time: Option<DateTime<Local>>) -> Self {
        return Command {
            commandline,
            time,
            ..Default::default()
        };
    }

    fn parse_line(mut self) -> Result<Self, Box<dyn Error>> {
        let command = self
            .commandline
            .strip_prefix("sudo")
            .unwrap_or(&self.commandline);
        let args: Vec<_> = command.split_whitespace().map(String::from).collect();
        let c = args
            .clone()
            .into_iter()
            .find(|s| !s.contains('=') && !s.contains('{'))
            .unwrap_or("".into());
        self.command = c;
        self.arguments = args;
        Ok(self)
    }
}

#[derive(Debug, Default)]
pub struct CommandParser {
    raw: String,
    commands: Vec<Command>,
}

impl CommandParser {
    pub fn from_raw(raw: String) -> Self {
        CommandParser {
            raw: raw,
            ..Default::default()
        }
    }

    pub fn parse(self, shell: &HistoryProvider) -> Result<Self, Box<dyn Error>> {
        match shell {
            HistoryProvider::Zsh => self.parse_zsh(),
            HistoryProvider::Bash => self.parse_bash(),
            HistoryProvider::Atuin => self.parse_atuin(),
        }
    }

    pub fn parse_zsh(mut self) -> Result<Self, Box<dyn Error>> {
        let captures = Re::captures(&RE_ZSH_HISTORY, &self.raw)?;
        let (timestamp, commands_raw) = (
            Re::get(&captures, 1)?.as_str(),
            Re::get(&captures, 3)?.as_str(),
        );
        let time = Some(DateTime::<Local>::from(
            UNIX_EPOCH + Duration::from_secs(timestamp.parse::<u64>()?),
        ));
        let commands_raw_splitted: Vec<_> = RE_COMMAND.split(commands_raw).collect();
        for commandline in commands_raw_splitted {
            self.commands
                .push(Command::from(commandline.into(), time).parse_line()?);
        }
        Ok(self)
    }

    pub fn parse_bash(mut self) -> Result<Self, Box<dyn Error>> {
        let captures = Re::captures(&RE_BASH_HISTORY, &self.raw)?;
        let (start_line, mut commands_raw_list) = (
            Re::get(&captures, 1)?.as_str(),
            Re::get(&captures, 2)?.as_str().to_string(),
        );
        let time = match start_line.parse::<u64>() {
            Ok(timestamp) => Some(DateTime::<Local>::from(
                UNIX_EPOCH + Duration::from_secs(timestamp),
            )),
            Err(_) => {
                commands_raw_list += start_line;
                None
            }
        };
        for commands_raw in commands_raw_list.lines() {
            let commands_raw_splitted: Vec<_> = RE_COMMAND.split(commands_raw).collect();
            for commandline in commands_raw_splitted {
                self.commands
                    .push(Command::from(commandline.into(), time).parse_line()?);
            }
        }
        Ok(self)
    }

    pub fn parse_atuin(mut self) -> Result<Self, Box<dyn Error>> {
        let (time_raw, commands_raw) = self
            .raw
            .split_once(';')
            .ok_or_else(|| "failed to split atuin command")?;

        let time = NaiveDateTime::parse_from_str(time_raw, "%Y-%m-%d %H:%M:%S")
            .ok()
            .and_then(|naive_time| Local.from_local_datetime(&naive_time).single());

        let command_raw_splited = RE_COMMAND.split(commands_raw);
        for commandline in command_raw_splited {
            self.commands
                .push(Command::from(commandline.into(), time).parse_line()?);
        }
        Ok(self)
    }

    pub fn finish(self) -> Vec<Command> {
        self.commands
    }
}

struct Re;

impl Re {
    fn captures<'a>(re: &Regex, s: &'a str) -> Result<Captures<'a>, Box<dyn Error>> {
        Ok(re
            .captures(s)
            .ok_or_else(|| format!("incomplete match found: {}", s))?)
    }

    fn get<'a>(captures: &Captures<'a>, index: usize) -> Result<Match<'a>, Box<dyn Error>> {
        Ok(captures
            .get(index)
            .ok_or_else(|| format!("failed to get from re capture"))?)
    }
}
