use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Cursor, Read},
    process::Command,
};

use crate::view::View;

#[derive(Debug, Clone)]
pub enum Shell {
    Zsh,
    Bash,
    Fish,
}

impl Shell {
    pub fn from(shell: &String) -> Self {
        match shell.as_str() {
            "zsh" => Self::Zsh,
            "bash" => {
                View::clear();
                View::content("It appears that you are using Bash");
                View::content(
                    "If you haven't configured the $HISTTIMEFORMAT for Bash, the time-related statistics may be INVALID :(",
                );
                View::content("(but other components will remain unaffected.)");
                View::content("Press [Enter] to continue");
                View::wait();
                Self::Bash
            }
            "fish" => Self::Fish,
            _ => {
                View::content(&format!(
                    "Sorry, {} is not supported yet\n\n",
                    shell.split('/').last().unwrap_or("")
                ));
                std::process::exit(1);
            }
        }
    }

    pub fn history_stream(&self) -> Result<Box<dyn Read>, Box<dyn Error>> {
        let file_path = match self {
            Shell::Zsh | Shell::Bash => {
                let shell_name = match self {
                    Shell::Zsh => ".zsh_history",
                    Shell::Bash => ".bash_history",
                    _ => unreachable!(),
                };
                format!("{}/{}", env::var("HOME")?, shell_name)
            }
            Shell::Fish => {
                let output = Command::new("fish")
                    .arg("-c")
                    .arg("history -show-time='%s# '")
                    .output()?;
                return Ok(Box::new(Cursor::new(output.stdout)));
            }
        };
        Ok(Box::new(File::open(file_path)?))
    }
}

pub struct History {
    buff_reader: BufReader<Box<dyn Read>>,
    shell_type: Shell,
}

impl History {
    pub fn from(shell: &Shell) -> Result<Self, Box<dyn Error>> {
        Ok(History {
            shell_type: shell.clone(),
            buff_reader: BufReader::new(shell.history_stream()?),
        })
    }
}

impl Iterator for History {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.shell_type {
            Shell::Zsh => {
                let mut ended = false;
                let mut line = String::new();
                while !ended {
                    let mut buf = vec![];
                    self.buff_reader.read_until(b'\n', &mut buf).ok()?;
                    if buf.is_empty() {
                        return if line.is_empty() { None } else { Some(line) };
                    }
                    line += String::from_utf8_lossy(&buf).into_owned().trim();
                    ended = !line.ends_with('\\');
                    if !ended {
                        line = line.strip_suffix('\\')?.into();
                    }
                }
                Some(line.trim().into())
            }
            Shell::Bash => {
                let mut buf = vec![];
                self.buff_reader.read_until(b'#', &mut buf).ok()?;
                if buf.is_empty() {
                    return None;
                }
                let str = String::from_utf8_lossy(&buf).into_owned();
                Some(str.strip_suffix("#").unwrap_or(&str).trim().into())
            }
            Shell::Fish => {
                let mut buf = vec![];

                loop {
                    self.buff_reader.read_until(b'\n', &mut buf).unwrap();
                    if buf.is_empty() {
                        return None;
                    }
                    let str = String::from_utf8_lossy(&buf).trim().to_owned();
                    if str.is_empty() {
                        buf.clear();
                        continue;
                    }
                    break Some(str);
                }
            }
        }
    }
}
