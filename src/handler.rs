use crate::app::{App, AppResult, AppState};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if app.app_state == AppState::Main {
        match key_event.code {
            // Exit application on `ESC` or `q`
            KeyCode::Char('q') => {
                app.quit();
            }
            // Exit application on `Ctrl-C`
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }
            }
            // Counter handlers
            KeyCode::Char('k') => {
                app.move_up();
            }
            KeyCode::Char('j') => {
                app.move_down();
            }
            KeyCode::Delete | KeyCode::Char('D') => {
                app.delete_current_task();
            }
            KeyCode::Enter | KeyCode::Char('a') => {
                app.create_task();
            }
            KeyCode::Tab => {
                app.mark_current_task_as_completed();
            }

            KeyCode::Char('o') => {
                app.start_typing();
            }

            KeyCode::Esc => {
                app.stop_typing();
            }
            // Other handlers you could add here.
            _ => {}
        }
    } else if app.app_state == AppState::Typing {
        match key_event.code {
            KeyCode::Esc => {
                app.stop_typing();
            }
            KeyCode::Backspace => {
                app.buffer.pop();
            }
            KeyCode::Char(c) => {
                app.buffer.push(c);
            }
            KeyCode::Enter => {
                let result = app.rename_current_task();
                if let Ok(_) = result {
                    app.stop_typing();
                }
            }
            _ => {}
        }
    }

    Ok(())
}
