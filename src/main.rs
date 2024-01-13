mod cli;
mod history;
mod parser;
mod stats;
mod view;

use cli::Cli;
use history::{History, HistoryProvider};
use parser::CommandParser;
use stats::Statistic;

fn main() {
    let args = Cli::parse_or_default();
    let shell = HistoryProvider::from(&args.shell);
    let history = History::from(&shell).expect("failed to read history file");
    let mut stats = Statistic::from(args.year);

    for block in history {
        CommandParser::from_raw(block)
            .parse(&shell)
            .unwrap_or_default()
            .finish()
            .iter()
            .for_each(|command| stats.analyze(command));
    }

    stats.output()
}
