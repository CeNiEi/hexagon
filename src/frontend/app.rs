use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::backend::{board::Board, direction::Direction};

pub(crate) struct App {
    pub(crate) terminate: bool,
    pub(crate) board: Board,
}

impl App {
    pub(crate) fn new() -> Self {
        let board = Board::new();

        Self {
            board,
            terminate: false,
        }
    }

    pub(crate) fn terminate(&mut self) {
        self.terminate = true;
    }

    pub(crate) fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => self.terminate(),
                    KeyCode::Up | KeyCode::Char('k') => self.board.move_current(Direction::Clock12),
                    KeyCode::Left | KeyCode::Char('h') => {
                        self.board.move_current(Direction::Clock10)
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        self.board.move_current(Direction::Clock2)
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.board.move_current(Direction::Clock6)
                    }
                    KeyCode::Enter => self.board.start_move(),
                    KeyCode::Esc => self.board.abort_move(),
                    _ => {}
                }
            }
            _ => {}
        };

        Ok(())
    }
}
