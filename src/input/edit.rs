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

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, KeyEventKind};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        }
    }

    #[test]
    fn char_appends_to_buffer_and_continues() {
        let mut buffer = EditBuffer { text: String::new() };

        let intent = handle_edit_mode(key(KeyCode::Char('a')), &mut buffer);

        assert_eq!(buffer.text, "a");
        assert_eq!(intent, Intent::Continue);
    }

    #[test]
    fn backspace_removes_last_character() {
        let mut buffer = EditBuffer { text: "ab".into() };

        let intent = handle_edit_mode(key(KeyCode::Backspace), &mut buffer);

        assert_eq!(buffer.text, "a");
        assert_eq!(intent, Intent::Continue);
    }

    #[test]
    fn enter_commits_current_text() {
        let mut buffer = EditBuffer { text: "value".into() };

        let intent = handle_edit_mode(key(KeyCode::Enter), &mut buffer);

        assert_eq!(
            intent,
            Intent::Commit(EditCommit {
                text: "value".into()
            })
        );
    }

    #[test]
    fn esc_cancels_editing() {
        let mut buffer = EditBuffer { text: "value".into() };

        let intent = handle_edit_mode(key(KeyCode::Esc), &mut buffer);

        assert_eq!(intent, Intent::Cancel);
    }
}
