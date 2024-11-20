mod app_state;
mod ui;
mod events;

use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture};
use std::io::{self, stdout};
use app_state::AppState;
use ui::draw_ui;
use events::handle_events;

fn main() -> Result<(), io::Error> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All))?;
    execute!(stdout, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app_state = AppState::new();
    let mut button_chunks: Vec<tui::layout::Rect> = vec![];
    let mut main_chunks: Vec<tui::layout::Rect> = vec![];

    loop {
        terminal.draw(|f| {
            draw_ui(f, &mut app_state, &mut button_chunks, &mut main_chunks);
        })?;

        let terminal_size = terminal.size()?;
        if let Err(_) = handle_events(&mut app_state, &button_chunks, &main_chunks, terminal_size) {
            break;
        }
    }

    execute!(terminal.backend_mut(), DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}
