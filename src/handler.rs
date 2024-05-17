use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Down => {
            app.forward(1);
        }
        KeyCode::Up => {
            app.backward(1);
        }
        KeyCode::Right => {
            app.right(8);
        }
        KeyCode::Left => {
            app.left(8);
        }
        KeyCode::PageDown => {
            app.forward((app.page_size() / 2).into());
        }
        KeyCode::PageUp => {
            app.backward((app.page_size() / 2).into());
        }
        KeyCode::Char('G') => {
            app.end();
        }
        KeyCode::Char('g') => {
            app.begin();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
