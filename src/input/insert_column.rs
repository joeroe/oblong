// src/input/insert_column.rs
// Handle input column mode input

use crossterm::event::{KeyEvent, KeyCode};
use crate::app::{ColumnInsertState, ColumnInsertPosition};
use crate::model::{ColumnType};

pub enum InsertColumnResult {
    Continue,
    Cancel,
    Commit {
        position: ColumnInsertPosition,
        name: String,
        col_type: ColumnType,
    },
}

pub fn handle_insert_column_mode(
    key: KeyEvent,
    state: &mut ColumnInsertState,
) -> InsertColumnResult {
    match state {
        ColumnInsertState::Naming { position, buffer } => {
            match key.code {
                KeyCode::Esc => InsertColumnResult::Cancel,

                KeyCode::Enter => {
                    if buffer.is_empty() {
                        InsertColumnResult::Continue
                    } else {
                        let name = buffer.clone();
                        *state = ColumnInsertState::Typing {
                            position: *position,
                            name,
                        };
                        InsertColumnResult::Continue
                    }
                }

                KeyCode::Backspace => {
                    buffer.pop();
                    InsertColumnResult::Continue
                }

                KeyCode::Char(c) => {
                    buffer.push(c);
                    InsertColumnResult::Continue
                }

                _ => InsertColumnResult::Continue,
            }
        }

        ColumnInsertState::Typing { position, name } => {
            let col_type = match key.code {
                KeyCode::Char('i') => Some(ColumnType::Integer),
                KeyCode::Char('f') => Some(ColumnType::Float),
                KeyCode::Char('t') => Some(ColumnType::Text),
                KeyCode::Char('b') => Some(ColumnType::Boolean),
                KeyCode::Esc => return InsertColumnResult::Cancel,
                _ => None,
            };

            match col_type {
                Some(col_type) => InsertColumnResult::Commit {
                    position: *position,
                    name: name.clone(),
                    col_type,
                },
                None => InsertColumnResult::Continue,
            }
        }
    }
}

