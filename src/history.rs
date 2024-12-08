use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Cursor, Read},
    process::Command,
    str::FromStr,
};

#[derive(Debug, Clone, strum::Display, strum::EnumString)]
pub enum HistoryProvider {
    #[strum(serialize = "zsh")]
    Zsh,
    #[strum(serialize = "bash")]
    Bash,
    #[strum(serialize = "atuin")]
    Atuin,
    #[strum(serialize = "fish")]
    Fish,
}

impl HistoryProvider {
    pub fn from(provider: &String) -> Self {
        HistoryProvider::from_str(provider)
            .unwrap_or_else(|_| panic!("Sorry, {} is not supported yet\n\n", provider))
    }

    pub fn history_stream(&self) -> Result<Box<dyn Read>, Box<dyn Error>> {
        match self {
            HistoryProvider::Zsh | HistoryProvider::Bash => {
                let shell = self.to_string();
                let output = Command::new(shell)
                    .args(["-i", "-c", r#"echo -e "\n$HISTFILE""#])
                    .output()?;

                let file_path = std::str::from_utf8(&output.stdout)?
                    .trim()
                    .lines()
                    .last()
                    .expect("env $HISTFILE not set");
                Ok(Box::new(File::open(file_path)?))
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
                    .arg("history --show-time='%s;'")
                    .output()?;
                Ok(Box::new(Cursor::new(output.stdout)))
            }
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
                    let str = String::from_utf8_lossy(&buf).into_owned();
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
