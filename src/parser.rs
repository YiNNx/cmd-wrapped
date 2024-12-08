use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use regex::{Captures, Match, Regex};
use std::{
    error::Error,
    time::{Duration, UNIX_EPOCH},
};

use crate::history::HistoryProvider;

lazy_static::lazy_static! {
    static ref RE_ZSH_HISTORY: Regex = Regex::new(r": (\d+):(\d+);(.+)").unwrap();
    static ref RE_BASH_HISTORY: Regex = Regex::new(r"#(.*)\n(.+)").unwrap();
    static ref RE_COMMAND: Regex = Regex::new(r"(?:\||&&)").unwrap();
}

#[derive(Debug, Default)]
pub struct Command {
    pub command_raw: String,
    pub time: Option<DateTime<Local>>,

    pub command: String,
    pub arguments: Vec<String>,
}

impl Command {
    fn from(commandline: String, time: Option<DateTime<Local>>) -> Self {
        Command {
            command_raw: commandline.trim().into(),
            time,
            ..Default::default()
        }
    }

    fn parse_line(mut self) -> Result<Self, Box<dyn Error>> {
        if self.command_raw.is_empty() || self.command_raw.starts_with('#') {
            return Ok(self);
        }
        let command = self
            .command_raw
            .strip_prefix("sudo")
            .unwrap_or(&self.command_raw);
        let args: Vec<_> = command.split_whitespace().map(String::from).collect();
        let c = args
            .iter()
            .find(|s| !s.is_empty() && !s.contains('=') && !s.contains('{'))
            .ok_or("invalid command")?;
        c.clone_into(&mut self.command);
        self.arguments = args;
        Ok(self)
    }
}

type ParsingData = (String, Option<DateTime<Local>>);

#[derive(Default)]
pub struct CommandParser {
    raw: String,
    commands: Vec<Command>,
}

impl CommandParser {
    pub fn from_raw(raw: String) -> Self {
        CommandParser {
            raw,
            ..Default::default()
        }
    }

    pub fn parse(mut self, provider: &HistoryProvider) -> Result<Self, Box<dyn Error>> {
        let (commands_combined, time) = match provider {
            HistoryProvider::Zsh => self.parse_zsh_raw(),
            HistoryProvider::Bash => self.parse_bash_raw(),
            HistoryProvider::Atuin => self.parse_atuin_raw(),
            HistoryProvider::Fish => self.parse_fish_raw(),
        }?;
        let commands_splitted = RE_COMMAND.split(&commands_combined);
        for commandline in commands_splitted {
            self.commands
                .push(Command::from(commandline.into(), time).parse_line()?);
        }
        Ok(self)
    }

    pub fn parse_zsh_raw(&self) -> Result<ParsingData, Box<dyn Error>> {
        let captures = Re::captures(&RE_ZSH_HISTORY, &self.raw)?;
        let (timestamp, commands_raw) = (
            Re::get(&captures, 1)?.as_str(),
            Re::get(&captures, 3)?.as_str().to_string(),
        );
        let time = Some(DateTime::<Local>::from(
            UNIX_EPOCH + Duration::from_secs(timestamp.parse::<u64>()?),
        ));
        Ok((commands_raw, time))
    }

    pub fn parse_bash_raw(&self) -> Result<ParsingData, Box<dyn Error>> {
        if !&self.raw.starts_with('#') {
            return Ok((self.raw.clone(), None));
        }
        let captures = Re::captures(&RE_BASH_HISTORY, &self.raw)?;
        let (timestamp, commands_raw) = (
            Re::get(&captures, 1)?.as_str(),
            Re::get(&captures, 2)?.as_str().to_string(),
        );
        let time = Some(DateTime::<Local>::from(
            UNIX_EPOCH + Duration::from_secs(timestamp.parse::<u64>()?),
        ));
        Ok((
            commands_raw
                .lines()
                .find(|item| !item.starts_with('#'))
                .ok_or("empty command found")?
                .into(),
            time,
        ))
    }

    pub fn parse_atuin_raw(&self) -> Result<ParsingData, Box<dyn Error>> {
        let (time_raw, commands_raw) = self
            .raw
            .split_once(';')
            .ok_or("failed to split atuin command")?;

        let time = NaiveDateTime::parse_from_str(time_raw, "%Y-%m-%d %H:%M:%S")
            .ok()
            .and_then(|naive_time| Local.from_local_datetime(&naive_time).single());

        Ok((commands_raw.into(), time))
    }

    pub fn parse_fish_raw(&self) -> Result<ParsingData, Box<dyn Error>> {
        let (timestamp, commands_raw) = self
            .raw
            .split_once(';')
            .ok_or("failed to split atuin command")?;

        let time = Some(DateTime::<Local>::from(
            UNIX_EPOCH + Duration::from_secs(timestamp.parse::<u64>()?),
        ));
        Ok((commands_raw.into(), time))
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
            .ok_or("failed to get match from re capture")?)
    }
}
