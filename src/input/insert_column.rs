// src/input/insert_column.rs
// Handle insert column mode input

use crossterm::event::{KeyEvent, KeyCode};
use crate::app::{ColumnInsertState, ColumnInsertPosition};
use crate::model::{ColumnType};
use crate::input::Intent;

#[derive(Debug, PartialEq, Eq)]
pub struct InsertColumnCommit {
    pub position: ColumnInsertPosition,
    pub name: String,
    pub col_type: ColumnType,
}

pub fn handle_insert_column_mode(
    key: KeyEvent,
    state: &mut ColumnInsertState,
) -> Intent<InsertColumnCommit> {
    match state {
        ColumnInsertState::Naming { position, buffer } => {
            match key.code {
                KeyCode::Esc => Intent::Cancel,

                KeyCode::Enter => {
                    if buffer.is_empty() {
                        Intent::Continue
                    } else {
                        let name = buffer.clone();
                        *state = ColumnInsertState::Typing {
                            position: *position,
                            name,
                        };
                        Intent::Continue
                    }
                }

                KeyCode::Backspace => {
                    buffer.pop();
                    Intent::Continue
                }

                KeyCode::Char(c) => {
                    buffer.push(c);
                    Intent::Continue
                }

                _ => Intent::Continue,
            }
        }

        ColumnInsertState::Typing { position, name } => {
            let col_type = match key.code {
                KeyCode::Char('i') => Some(ColumnType::Integer),
                KeyCode::Char('f') => Some(ColumnType::Float),
                KeyCode::Char('t') => Some(ColumnType::Text),
                KeyCode::Char('b') => Some(ColumnType::Boolean),
                KeyCode::Esc => return Intent::Cancel,
                _ => None,
            };

            match col_type {
                Some(col_type) => Intent::Commit(InsertColumnCommit {
                    position: *position,
                    name: name.clone(),
                    col_type,
                }),
                None => Intent::Continue,
            }
        }
    }
}
