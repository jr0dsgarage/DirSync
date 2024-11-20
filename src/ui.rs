use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use crate::app_state::AppState;

pub fn draw_ui(f: &mut tui::Frame<CrosstermBackend<std::io::Stdout>>, app_state: &mut AppState, button_chunks: &mut Vec<Rect>, main_chunks: &mut Vec<Rect>) {
    // Define the layout with three horizontal chunks and one vertical chunk at the bottom
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(90), // Main content
            Constraint::Percentage(10), // Output
        ].as_ref())
        .split(f.size());

    *main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Length(10), // Space for buttons
            Constraint::Percentage(45),
        ].as_ref())
        .split(chunks[0]);

    // Define the blocks for local and remote directories
    let local_list: Vec<ListItem> = app_state.local_directories.iter().map(|d| ListItem::new(d.as_str())).collect();
    let remote_list: Vec<ListItem> = app_state.remote_directories.iter().map(|d| ListItem::new(d.as_str())).collect();

    let local_block = List::new(local_list)
        .block(Block::default().title("Local Directory").borders(Borders::ALL))
        .highlight_style(tui::style::Style::default().bg(tui::style::Color::LightGreen));
    let remote_block = List::new(remote_list)
        .block(Block::default().title("Remote Directory").borders(Borders::ALL))
        .highlight_style(tui::style::Style::default().bg(tui::style::Color::LightGreen));
    let buttons = Block::default().borders(Borders::NONE);

    // Render the blocks
    f.render_stateful_widget(local_block, main_chunks[0], &mut app_state.local_state);
    f.render_stateful_widget(remote_block, main_chunks[2], &mut app_state.remote_state);
    f.render_widget(buttons, main_chunks[1]);

    // Define the layout for buttons inside the middle chunk
    *button_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40), // Flexible space above
            Constraint::Length(3),      // Button height
            Constraint::Length(3),      // Button height
            Constraint::Percentage(40), // Flexible space below
        ].as_ref())
        .split(main_chunks[1]);

    // Define the buttons
    let transfer_to_remote = Paragraph::new(">>")
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    let transfer_to_local = Paragraph::new("<<")
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    // Render the buttons
    f.render_widget(transfer_to_remote, button_chunks[1]);
    f.render_widget(transfer_to_local, button_chunks[2]);

    // Render the output
    let output_block = Paragraph::new(app_state.output.as_str())
        .block(Block::default().borders(Borders::NONE));
    f.render_widget(output_block, chunks[1]);
}