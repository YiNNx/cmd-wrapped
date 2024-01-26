use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Cursor, Read},
    process::Command,
};

use crate::view::View;

#[derive(Debug, Clone)]
pub enum HistoryProvider {
    Zsh,
    Bash,
    Atuin,
    Fish,
}

impl HistoryProvider {
    pub fn from(provider: &String) -> Self {
        match provider.as_str() {
            "zsh" => Self::Zsh,
            "bash" => Self::Bash,
            "atuin" => Self::Atuin,
            "fish" => Self::Fish,
            _ => {
                View::content(&format!(
                    "Sorry, {} is not supported yet\n\n",
                    provider.split('/').last().unwrap_or("")
                ));
                std::process::exit(1);
            }
        }
    }

    pub fn history_stream(&self) -> Result<Box<dyn Read>, Box<dyn Error>> {
        match self {
            HistoryProvider::Zsh | HistoryProvider::Bash => {                
                let program = self.program_name();

                let output = Command::new(program)
                    .args(["-i", "-c", "echo $HISTFILE"])
                    .output()?;

                let file_path = std::str::from_utf8(&output.stdout)?.trim().to_string();
                Ok(Box::new(File::open(file_path.trim())?))
            }
            HistoryProvider::Atuin => {
                let output = Command::new("atuin")
                    .args(["history", "list", "--format", "{time};{command}"])
                    .output()?;
                Ok(Box::new(Cursor::new(output.stdout)))
            }
            HistoryProvider::Fish => {
                let output = Command::new("fish")
                    .arg("-c")
                    .arg("history -show-time='%s;'")
                    .output()?;
                Ok(Box::new(Cursor::new(output.stdout)))
            }
        }
    }

    pub fn program_name(&self) -> &'static str {
        match self {
            HistoryProvider::Zsh => "zsh",
            HistoryProvider::Bash => "bash",
            HistoryProvider::Atuin => "atuin",
            HistoryProvider::Fish => "fish",
        }
    } 
}

pub struct History {
    buff_reader: BufReader<Box<dyn Read>>,
    provider: HistoryProvider,
}

impl History {
    pub fn from(provider: &HistoryProvider) -> Result<Self, Box<dyn Error>> {
        Ok(History {
            provider: provider.clone(),
            buff_reader: BufReader::new(provider.history_stream()?),
        })
    }
}

impl Iterator for History {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.provider {
            HistoryProvider::Zsh | HistoryProvider::Atuin | HistoryProvider::Fish => {
                let mut block = String::new();
                let mut buf = vec![];
                loop {
                    self.buff_reader.read_until(b'\n', &mut buf).unwrap();
                    if buf.is_empty() {
                        return if block.is_empty() { None } else { Some(block) };
                    }
                    let str = String::from_utf8_lossy(&buf).trim_end().to_owned();
                    block += &str;
                    if str.is_empty() {
                        buf.clear();
                        continue;
                    }
                    if str.ends_with('\\') {
                        block = block.strip_suffix('\\')?.into();
                        buf.clear();
                        continue;
                    }
                    break Some(block);
                }
            }
            HistoryProvider::Bash => {
                let mut block = String::new();
                let mut buf = vec![];
                loop {
                    self.buff_reader.read_until(b'\n', &mut buf).unwrap();
                    if buf.is_empty() {
                        return None;
                    }
                    let str = String::from_utf8_lossy(&buf).to_owned();
                    block += &str;
                    if str.is_empty() || str.starts_with('#') {
                        buf.clear();
                        continue;
                    }
                    break Some(block);
                }
            }
        }
    }
}
