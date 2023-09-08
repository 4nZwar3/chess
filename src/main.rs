mod chess;
#[allow(dead_code)]
#[allow(unused_imports)]
mod piece;
use crate::chess::*;
use crossterm::event::Event::Key;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::Backend,
    backend::CrosstermBackend,
    text::Span,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let mut game = Chess::new();
    game.init();
    enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let result = run_game(&mut terminal, &mut game);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    if let Err(e) = result {
        println!("{}", e.to_string());
    }
    Ok(())
}
fn run_game<B: Backend>(
    terminal: &mut Terminal<B>,
    game: &mut Chess,
) -> Result<(), std::io::Error> {
    loop {
        terminal.draw(|f| ui(f, game))?;
        if let Key(key) = event::read()? {
            match game.get_move_mode() {
                MoveMode::Select => match key.code {
                    event::KeyCode::Esc => return Ok(()),
                    event::KeyCode::Up => {} //Move selected up
                    event::KeyCode::Left => {} //Move selected left
                    event::KeyCode::Right => {} //Move selected right
                    event::KeyCode::Down => {} //Move selected down
                    event::KeyCode::Enter | event::KeyCode::Tab => game.switch_move_mode(), //Switch Mode to Mark
                    _ => {}
                },
                MoveMode::Mark => match key.code {
                    event::KeyCode::Esc => return Ok(()),
                    event::KeyCode::Up => {} //Move marked up
                    event::KeyCode::Left => {} //Move marked left
                    event::KeyCode::Right => {} //Move marked right
                    event::KeyCode::Down => {} //Move marked down
                    event::KeyCode::Enter => {}, //Make move
                    event::KeyCode::Tab => game.switch_move_mode(), //Switch Mode to Select
                    _ => {}
                },
            }
        }
    }
    Ok(())
}
fn ui<B: Backend>(f: &mut Frame<B>, game: &mut Chess) {}
