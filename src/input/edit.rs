// src/input/edit.rs
// Handle edit mode input

use crossterm::event::{KeyEvent, KeyCode};
use crate::app::{EditBuffer};

pub enum EditResult {
    Continue,
    Commit,
    Cancel,
}

pub fn handle_edit_mode(
    key: KeyEvent,
    buffer: &mut EditBuffer,
) -> EditResult {
    match key.code {
        KeyCode::Char(c) => {
            buffer.text.push(c);
            EditResult::Continue
        }
        KeyCode::Backspace => {
            buffer.text.pop();
            EditResult::Continue
        }
        KeyCode::Enter => EditResult::Commit,
        KeyCode::Esc => EditResult::Cancel,
        _ => EditResult::Continue,
    }
}
