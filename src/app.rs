use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    widgets::{
        Block, Widget,
        canvas::{Canvas, Shape},
    },
};

use anyhow::Result;

use crate::{board::Board, utils::direction::Direction};

pub struct App {
    terminate: bool,
    board: Board<()>,
}

impl App {
    pub fn new(len: f64) -> App {
        Self {
            terminate: false,
            board: Board::new(len),
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
            KeyCode::Char('q') | KeyCode::Esc => self.terminate = true,
            KeyCode::Left => self.board.move_current(Direction::Clock10),
            KeyCode::Right => self.board.move_current(Direction::Clock2),
            KeyCode::Up => self.board.move_current(Direction::Clock12),
            KeyCode::Down => self.board.move_current(Direction::Clock6),
            _ => {}
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(&self.board, frame.area());
    }
}
