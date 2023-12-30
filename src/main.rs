mod history;
mod parser;
mod stats;
mod view;

use history::{History, Shell};
use parser::CommandParser;
use stats::Statistic;
use view::View;

fn main() {
    View::display_title();

    let mut stats = Statistic::new();
    let shell = Shell::init().expect("fail to load shell type");
    let history = History::from(&shell).expect("failed to read history file");

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
