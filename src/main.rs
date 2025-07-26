use app::App;

mod app;
mod board;
mod hexagon;
mod pieces;
mod unit;
mod utils;
mod state;

use anyhow::Result;
use clap::{value_parser, Parser, ValueEnum};
use utils::{fill_mode::FillMode, depth::Depth};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "LEN", default_value_t = 6.)]
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
        default_value_t = FillMode::Wireframe
    )]
    color_mode: FillMode,

    #[arg(short, long, value_name = "HIDE_PIECES")]
    hide_pieces: bool,

    #[arg(short, long, value_name = "HIDE_HIGHLIGHTS")]
    hide_highlights: bool,

    #[arg(short, long, value_name = "LOGGING")]
    logging: bool


}

fn setup_logger() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("output.log")?)
        .apply()?;

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.logging {
        setup_logger()?;
    }

    let mut terminal = ratatui::init();
    let res = App::new(cli.len, cli.padding, Depth::new(cli.depth)?, cli.color_mode, cli.hide_pieces, cli.hide_highlights).run(&mut terminal);
    ratatui::restore();
    res
}
