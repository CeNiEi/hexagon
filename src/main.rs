use app::App;

mod app;
mod board;
mod hexagon;
mod pieces;
mod unit;
mod utils;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "LEN")]
    len: Option<f64>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut terminal = ratatui::init();
    let res = App::new(cli.len.unwrap_or(3.)).run(&mut terminal);
    ratatui::restore();
    res
}
