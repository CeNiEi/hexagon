use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    widgets::{
        Block, Widget,
        canvas::{Canvas, Shape},
    },
};

use anyhow::Result;

use crate::{
    board::Board,
    pieces::Piece,
    state::State,
    utils::{depth::Depth, direction::Direction, fill_mode::FillMode},
};

pub struct App {
    terminate: bool,
    board: Board<Box<dyn Piece>>,
    state: State,
}

impl App {
    pub fn new(
        len: f64,
        padding: f64,
        depth: Depth,
        color_mode: FillMode,
        hide_pieces: bool,
        hide_highlights: bool,
    ) -> App {
        Self {
            terminate: false,
            board: Board::new(
                len,
                padding,
                depth,
                color_mode,
                hide_pieces,
                hide_highlights,
            ),
            state: State::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.terminate {
            terminal.draw(|frame| self.draw(frame))?;
            match event::read()? {
                Event::Key(event) => self.handle_key_event(event),
                _ => (),
            }
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if !key.is_press() {
            return;
        }

        match key.code {
            KeyCode::Char('q') => self.terminate = true,
            KeyCode::Left => self.state.move_current(&mut self.board, Direction::Clock10),
            KeyCode::Right => self.state.move_current(&mut self.board, Direction::Clock2),
            KeyCode::Up => self.state.move_current(&mut self.board, Direction::Clock12),
            KeyCode::Down => self.state.move_current(&mut self.board, Direction::Clock6),
            KeyCode::Enter => self.state.toggle_valid_moves(&mut self.board),
            _ => {}
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(&self.board, frame.area());
    }
}
