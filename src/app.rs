use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
};

use anyhow::Result;

use crate::{
    board::{Board, BoardView},
    pieces::PieceType,
    state::{Panel, State},
    utils::{depth::Depth, direction::Direction, fill_mode::FillMode},
};

pub struct App {
    terminate: bool,
    board: Board,
    state: State,
}

impl App {
    pub fn new(len: f64, padding: f64, color_mode: FillMode, hide_highlights: bool) -> App {
        Self::from_board(Board::new(len, padding, color_mode, hide_highlights))
    }

    pub fn preview(
        len: f64,
        padding: f64,
        depth: Depth,
        color_mode: FillMode,
        hide_highlights: bool,
    ) -> App {
        Self::from_board(Board::preview(
            len,
            padding,
            depth,
            color_mode,
            hide_highlights,
        ))
    }

    fn from_board(board: Board) -> App {
        Self {
            terminate: false,
            board,
            state: State::new(),
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

        if self.state.is_promoting() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Enter => self
                    .state
                    .select_promotion(&mut self.board, PieceType::Queen),
                KeyCode::Char('r') => self
                    .state
                    .select_promotion(&mut self.board, PieceType::Rook),
                KeyCode::Char('b') => self
                    .state
                    .select_promotion(&mut self.board, PieceType::Bishop),
                KeyCode::Char('n') => self
                    .state
                    .select_promotion(&mut self.board, PieceType::Knight),
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Char('q') => self.terminate = true,
            KeyCode::Char('p') => self.state.toggle_panel(),
            KeyCode::Left => self.state.move_current(&mut self.board, Direction::Clock10),
            KeyCode::Right => self.state.move_current(&mut self.board, Direction::Clock2),
            KeyCode::Up => self.state.move_current(&mut self.board, Direction::Clock12),
            KeyCode::Down => self.state.move_current(&mut self.board, Direction::Clock6),
            KeyCode::Enter => self.state.toggle_help_or_move(&mut self.board),
            _ => {}
        }
    }

    fn draw(&self, frame: &mut Frame) {
        match self.state.panel() {
            Panel::Hidden => {
                let board_view = BoardView {
                    board: &self.board,
                    div: 2.,
                };

                frame.render_widget(&board_view, frame.area());
            }
            Panel::Visible { width_percentage } => {
                let [board_area, state_area] = Layout::horizontal([
                    Constraint::Percentage(100 - *width_percentage),
                    Constraint::Percentage(*width_percentage),
                ])
                .areas(frame.area());

                let board_view = BoardView {
                    board: &self.board,
                    div: 2.75,
                };

                frame.render_widget(&board_view, board_area);
                frame.render_widget(&self.state, state_area);
            }
        }
    }
}
