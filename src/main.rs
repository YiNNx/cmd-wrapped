mod cli;
mod config;
mod history;
mod parser;
mod stats;
mod view;

use cli::Args;
use config::Config;
use history::{History, HistoryProvider};
use parser::CommandParser;
use stats::Statistic;

fn main() {
    let config = Config::read().expect("failed to read from local config path");
    let args = Args::parse();
    if let Some(query) = args.query {
        let provider = HistoryProvider::from(&query.shell);
        let history = History::from(&provider).expect("failed to read history file");
        let mut stats = Statistic::from(query.year);

        for block in history {
            CommandParser::from_raw(block)
                .parse(&provider)
                .unwrap_or_default()
                .finish()
                .iter()
                .for_each(|command| stats.analyze(command));
        }
        stats.output()
    } else if let Some(setting) = args.setting {
        match config {
            Some(c) => todo!(),
            None => todo!(),
        }
    }
}
