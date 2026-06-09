// src/input/mod.rs
// User input controller

use crossterm::event::KeyEvent;
use crate::app::{App, Mode, ColumnInsertPosition};
use crate::model::{Column, commit_edit};

// Edit modes
mod normal;
mod edit;
mod insert_column;

use normal::handle_normal_mode;
use edit::{handle_edit_mode, EditResult};
use insert_column::{
    handle_insert_column_mode,
    InsertColumnResult,
};

pub fn handle_key(key: KeyEvent, app: &mut App) -> bool {
    match &mut app.mode {
        Mode::Normal => handle_normal_mode(key, app),
        Mode::Editing(buffer) => {
            match handle_edit_mode(key, buffer) {
                EditResult::Continue => {}

                EditResult::Cancel => {
                    app.mode = Mode::Normal;
                    app.status = None;
                }

                EditResult::Commit => {
                    let result = commit_edit(
                        &mut app.table,
                        app.cursor.row,
                        app.cursor.col,
                        &buffer.text,
                    );

                    match result {
                        Ok(()) => {
                            app.mode = Mode::Normal;
                            app.status = None;
                        }
                        Err(err) => {
                            app.status = Some(err);
                        }
                    }
                }
            }
            false
        },
        Mode::InsertColumn(state) => {
            match handle_insert_column_mode(key, state) {
                InsertColumnResult::Continue => {}

                InsertColumnResult::Cancel => {
                    app.mode = Mode::Normal;
                    app.status = None;
                }

                InsertColumnResult::Commit { position, name, col_type } => {
                    let insert_at = match position {
                        ColumnInsertPosition::Before => app.cursor.col,
                        ColumnInsertPosition::After => app.cursor.col + 1,
                    };

                    app.table.insert_column(
                        insert_at,
                        Column { name, col_type },
                    );

                    app.cursor.col = insert_at;
                    app.mode = Mode::Normal;
                    app.status = None;
                }
            }
            false
        },
    }
}
