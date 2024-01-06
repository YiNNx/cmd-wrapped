use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::view::View;

#[derive(Debug, Clone)]
pub enum Shell {
    Zsh,
    Bash,
    // Fish,
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
            // "fish" => Self::Fish,
            _ => {
                View::content(&format!(
                    "Sorry, {} is not supported yet\n\n",
                    shell.split('/').last().unwrap_or("")
                ));
                std::process::exit(1);
            }
        }
    }

    pub fn history_file_path(&self) -> Result<String, Box<dyn Error>> {
        Ok(match self {
            Shell::Zsh => {
                env::var("HISTFILE").unwrap_or(format!("{}/.zsh_history", env::var("HOME")?))
            }
            Shell::Bash => {
                env::var("HISTFILE").unwrap_or(format!("{}/.bash_history", env::var("HOME")?))
            }
        })
    }
}

pub struct History {
    buff_reader: BufReader<File>,
    shell_type: Shell,
}

impl History {
    pub fn from(shell: &Shell) -> Result<Self, Box<dyn Error>> {
        Ok(History {
            shell_type: shell.clone(),
            buff_reader: BufReader::new(File::open(shell.history_file_path()?)?),
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
        }
    }
}
