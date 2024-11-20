use crossterm::event::{self, Event, KeyCode, MouseEventKind};
use crate::app_state::AppState;
use tui::layout::Rect;

pub fn handle_events(app_state: &mut AppState, button_chunks: &Vec<Rect>, main_chunks: &Vec<Rect>, terminal_size: Rect) -> Result<(), std::io::Error> {
    // Handle mouse events
    if let Ok(Event::Mouse(mouse_event)) = event::read() {
        if mouse_event.column < terminal_size.width && mouse_event.row < terminal_size.height {
            match mouse_event.kind {
                MouseEventKind::Down(_) => {
                    let now = std::time::Instant::now();
                    let x = mouse_event.column;
                    let y = mouse_event.row;
                    let middle_start = terminal_size.width / 2 - 5;
                    let middle_end = terminal_size.width / 2 + 5;

                    // Clear the output before appending the new message
                    app_state.output.clear();

                    if x >= middle_start && x <= middle_end {
                        if y >= button_chunks[1].y && y < button_chunks[1].y + button_chunks[1].height {
                            // Clicked on "Transfer to Remote" button
                            app_state.output.push_str("Transfer to Remote button clicked\n");
                        } else if y >= button_chunks[2].y && y < button_chunks[2].y + button_chunks[2].height {
                            // Clicked on "Transfer to Local" button
                            app_state.output.push_str("Transfer to Local button clicked\n");
                        }
                    } else if x < middle_start {
                        // Clicked in the local directory pane
                        let local_block_y = main_chunks[0].y;
                        let index = (y as usize).saturating_sub(local_block_y as usize).saturating_sub(1);
                        if index < app_state.local_directories.len() && y >= local_block_y && y < local_block_y + main_chunks[0].height {
                            let clicked_dir = app_state.local_directories[index].clone();
                            app_state.output.push_str(&format!("Clicked on local directory: {}\n", clicked_dir));
                            if now.duration_since(app_state.last_click_time) < app_state.double_click_threshold {
                                // Handle double-click
                                app_state.current_local_dir = clicked_dir;
                                app_state.local_directories = AppState::get_directories(&app_state.current_local_dir);
                            }
                            app_state.local_state.select(Some(index));
                        }
                    } else {
                        // Clicked in the remote directory pane
                        let remote_block_y = main_chunks[2].y;
                        let index = (y as usize).saturating_sub(remote_block_y as usize).saturating_sub(1);
                        if index < app_state.remote_directories.len() && y >= remote_block_y && y < remote_block_y + main_chunks[2].height {
                            let clicked_dir = app_state.remote_directories[index].clone();
                            app_state.output.push_str(&format!("Clicked on remote directory: {}\n", clicked_dir));
                            if now.duration_since(app_state.last_click_time) < app_state.double_click_threshold {
                                // Handle double-click
                                app_state.current_remote_dir = clicked_dir;
                                app_state.remote_directories = AppState::get_directories(&app_state.current_remote_dir);
                            }
                            app_state.remote_state.select(Some(index));
                        }
                    }
                    app_state.last_click_time = now;
                }
                _ => {}
            }
        }
    }

    // Handle keyboard events
    if let Event::Key(key) = event::read()? {
        if key.code == KeyCode::Char('q') {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Quit"));
        }
    }

    Ok(())
}