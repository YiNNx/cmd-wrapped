use chrono::{DateTime, Local};
use regex::Regex;
use std::{
    error::Error,
    time::{Duration, UNIX_EPOCH},
};

use crate::history::Shell;

lazy_static::lazy_static! {
    static ref RE_ZSH_HISTORY: Regex = Regex::new(r": (\d+):(\d+);(.+)").unwrap();
    static ref RE_BASH_HISTORY: Regex = Regex::new(r"(\d+)\n((?:[^#\n]|\n)+)").unwrap();
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
            .find(|s| !s.contains('=') && !s.contains('{') && !s.is_empty())
            .ok_or_else(|| "invalid command")?;
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

    pub fn parse(self, shell: &Shell) -> Result<Self, Box<dyn Error>> {
        match shell {
            Shell::Zsh => self.parse_zsh(),
            Shell::Bash => self.parse_bash(),
        }
    }

    pub fn parse_zsh(mut self) -> Result<Self, Box<dyn Error>> {
        let captures = RE_ZSH_HISTORY
            .captures(&self.raw)
            .ok_or_else(|| format!("Incomplete match found: {}", self.raw))?;
        let (timestamp, commands_raw) = (
            captures
                .get(1)
                .ok_or_else(|| format!("Incomplete match found: {}", self.raw))?
                .as_str(),
            captures
                .get(3)
                .ok_or_else(|| format!("Incomplete match found: {}", self.raw))?
                .as_str(),
        );
        let time = Some(DateTime::<Local>::from(
            UNIX_EPOCH + Duration::from_secs(timestamp.parse::<u64>()?),
        ));
        let commands_raw_splitted: Vec<_> = RE_COMMAND
            .split(commands_raw)
            .filter(|s| !s.is_empty())
            .collect();
        for commandline in commands_raw_splitted {
            self.commands
                .push(Command::from(commandline.into(), time).parse_line()?);
        }
        Ok(self)
    }

    pub fn parse_bash(mut self) -> Result<Self, Box<dyn Error>> {
        let captures = RE_BASH_HISTORY
            .captures(&self.raw)
            .ok_or_else(|| format!("Incomplete match found: {}", self.raw))?;
        let (start_line, mut commands_raw_list) = (
            captures
                .get(1)
                .ok_or_else(|| format!("Incomplete match found: {}", self.raw))?
                .as_str(),
            captures
                .get(2)
                .ok_or_else(|| format!("Incomplete match found: {}", self.raw))?
                .as_str()
                .to_string(),
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
            let commands_raw_splitted: Vec<_> = RE_COMMAND
                .split(commands_raw)
                .filter(|s| !s.is_empty())
                .collect();
            for commandline in commands_raw_splitted {
                self.commands
                    .push(Command::from(commandline.into(), time).parse_line()?);
            }
        }
        Ok(self)
    }

    pub fn finish(self) -> Vec<Command> {
        self.commands
    }
}
