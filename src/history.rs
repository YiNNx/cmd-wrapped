use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
pub enum Shell {
    Zsh,
    // Fish,
    // Bash,
}

impl Shell {
    pub fn init() -> Result<Self, Box<dyn Error>> {
        let shell = env::var("SHELL")?;
        Ok(match shell.split('/').last().unwrap_or("") {
            "zsh" => Self::Zsh,
            // "fish" => Self::Fish,
            // "bash" => Self::Bash,
            _ => return Err(format!("shell type not supported yet").into()),
        })
    }

    pub fn history_file_path(&self) -> Result<String, Box<dyn Error>> {
        Ok(match self {
            Shell::Zsh => {
                env::var("HISTFILE").unwrap_or(format!("{}/.zsh_history", env::var("HOME")?))
            }
            // Shell::Fish => todo!(),
            // Shell::Bash => todo!(),
        })
    }
}

pub struct History {
    buff_reader: BufReader<File>,
}

impl History {
    pub fn from(shell: &Shell) -> Result<Self, Box<dyn Error>> {
        Ok(History {
            buff_reader: BufReader::new(File::open(shell.history_file_path()?)?),
        })
    }
}

impl Iterator for History {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ended = false;
        let mut line = String::new();
        while !ended {
            let mut buf = vec![];
            self.buff_reader.read_until(b'\n', &mut buf).ok()?;
            if buf.is_empty() {
                return if line.is_empty() { None } else { Some(line) };
            }
            line += String::from_utf8_lossy(&buf).into_owned().as_str().trim();
            ended = !line.ends_with('\\');
            if !ended {
                line = line.strip_suffix('\\')?.into();
            }
            buf.clear();
        }
        Some(line.trim().into())
    }
}
