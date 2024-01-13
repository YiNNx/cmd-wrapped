use chrono::{DateTime, Local, TimeZone};
use regex::{Captures, Match, Regex};
use std::{
    error::Error,
    time::{Duration, UNIX_EPOCH},
};

use crate::history::Shell;

lazy_static::lazy_static! {
    static ref RE_ZSH_HISTORY: Regex = Regex::new(r": (\d+):(\d+);(.*)").unwrap();
    static ref RE_BASH_HISTORY: Regex = Regex::new(r"(\d+)\n((?:[^#\n]|\n)*)").unwrap();
    static ref RE_AUTIN_HISTORY: Regex = Regex::new(r"(?P<y>\d+)-(?P<m>\d+)-(?P<d>\d+) (?P<H>\d+):(?P<M>\d+):(?P<S>\d+) (?P<cmd>.+)").unwrap();
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

    pub fn parse(self, shell: &Shell) -> Result<Self, Box<dyn Error>> {
        match shell {
            Shell::Zsh => self.parse_zsh(),
            Shell::Bash => self.parse_bash(),
            Shell::Atuin => self.parse_atuin(),
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
        let captures = Re::captures(&RE_AUTIN_HISTORY, &self.raw)?;

        let year = captures.name("y").unwrap().as_str().parse::<i32>()?;
        let month = captures.name("m").unwrap().as_str().parse::<u32>()?;
        let day = captures.name("d").unwrap().as_str().parse::<u32>()?;
        let hour = captures.name("H").unwrap().as_str().parse::<u32>()?;
        let min = captures.name("M").unwrap().as_str().parse::<u32>()?;
        let sec = captures.name("S").unwrap().as_str().parse::<u32>()?;

        let time = Local
            .with_ymd_and_hms(year, month, day, hour, min, sec)
            .single();

        let command = captures
            .name("cmd")
            .ok_or_else(|| format!("Incomplete match found: {}", self.raw))?
            .as_str();

        let command_raw_splited = RE_COMMAND.split(command);

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
