use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stderr};

use super::app::App;
use super::ui::render_app;

pub(crate) struct Tui {
    terminal: Terminal<CrosstermBackend<Stderr>>,
}

impl Tui {
    pub(crate) fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let prev_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            Self::reset().unwrap();
            prev_hook(info);
        }));

        let backend = CrosstermBackend::new(std::io::stderr());
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }

    pub(crate) fn enter(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;

        Ok(())
    }

    pub(crate) fn reset() -> Result<(), Box<dyn std::error::Error>> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;

        Ok(())
    }

    pub(crate) fn exit(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Self::reset()?;
        self.terminal.show_cursor()?;

        Ok(())
    }

    pub(crate) fn draw(&mut self, app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.draw(|frame| render_app(app, frame))?;

        Ok(())
    }
}
