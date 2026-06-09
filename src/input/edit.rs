// src/input/edit.rs
// Handle edit mode input

use crossterm::event::{KeyEvent, KeyCode};
use crate::app::{EditBuffer};
use crate::input::Intent;

#[derive(Debug, PartialEq, Eq)]
pub struct EditCommit {
    pub text: String,
}

pub fn handle_edit_mode(
    key: KeyEvent,
    buffer: &mut EditBuffer,
) -> Intent<EditCommit> {
    match key.code {
        KeyCode::Char(c) => {
            buffer.text.push(c);
            Intent::Continue
        }
        KeyCode::Backspace => {
            buffer.text.pop();
            Intent::Continue
        }
        KeyCode::Enter => Intent::Commit(EditCommit {
            text: buffer.text.clone()
        }),
        KeyCode::Esc => Intent::Cancel,
        _ => Intent::Continue,
    }
}

