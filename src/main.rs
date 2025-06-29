use app::App;

mod app;
mod board;
mod hexagon;
mod pieces;
mod unit;
mod utils;

use anyhow::Result;
use clap::{value_parser, Parser, ValueEnum};
use utils::{color_mode::ColorMode, depth::Depth};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "LEN", default_value_t = 3.)]
    len: f64,

    #[arg(short, long, value_name = "PADDING", default_value_t = 0.8)]
    padding: f64,

    #[arg(
        short, 
        long, 
        value_name = "DEPTH", 
        value_parser = value_parser!(u8).range(1..=6), 
        default_value_t = 6
    )]
    depth: u8,

    #[arg(
        short, 
        long, 
        value_name = "COLOR_MODE", 
        value_enum,
        default_value_t = ColorMode::Filled
    )]
    color_mode: ColorMode
}


fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut terminal = ratatui::init();
    let res = App::new(cli.len, cli.padding, Depth::new(cli.depth)?, cli.color_mode).run(&mut terminal);
    ratatui::restore();
    res
}
