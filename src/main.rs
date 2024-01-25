mod cli;
mod config;
mod history;
mod parser;
mod stats;
mod view;

use cli::Cli;
use config::Config;
use history::{History, HistoryProvider};
use parser::CommandParser;
use stats::Statistic;

fn main() {
    let config = Config::read();
    let args = Cli::parse_or_default();
    let provider = HistoryProvider::from(&args.shell);
    let history = History::from(&provider).expect("failed to read history file");
    let mut stats = Statistic::from(args.year);

    for block in history {
        CommandParser::from_raw(block)
            .parse(&provider)
            .unwrap_or_default()
            .finish()
            .iter()
            .for_each(|command| stats.analyze(command));
    }

    stats.output()
}
