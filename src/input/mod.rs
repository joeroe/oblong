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
use edit::{handle_edit_mode, EditCommit};
use insert_column::{handle_insert_column_mode, InsertColumnCommit};

pub enum ControllerAction {
    Continue,
    Quit,
}

/// Result of handling a key in a specific mode
///
/// Mode handlers return an `Intent<T>` describing what should happen next.
/// The controller (`handle_key`) is responsible for applying the intent.
#[derive(Debug, PartialEq, Eq)]
pub enum Intent<T> {
    /// Continue expecting more input
    Continue,  
    /// Exit without changing data
    Cancel,    
    /// Exit changing data
    Commit(T), 
}

fn handle_intent<T, F>(
    intent: Intent<T>,
    app: &mut App,
    on_commit: F,
)
where
    F: FnOnce(&mut App, T),
{
    match intent {
        Intent::Continue => {
            // Nothing to do
        }

        Intent::Cancel => {
            app.mode = Mode::Normal;
            app.status = None;
        }

        Intent::Commit(payload) => {
            on_commit(app, payload);
            app.mode = Mode::Normal;
            app.status = None;
        }
    }
}

pub fn handle_key(key: KeyEvent, app: &mut App) -> ControllerAction {
    match &mut app.mode {
        Mode::Normal => handle_normal_mode(key, app),
        Mode::Editing(buffer) => {
            let intent = handle_edit_mode(key, buffer);
            handle_intent(intent, app, apply_edit_commit);
            ControllerAction::Continue
        },
        Mode::InsertColumn(state) => {
            let intent = handle_insert_column_mode(key, state);
            handle_intent(intent, app, apply_insert_column_commit);
            ControllerAction::Continue
        },
    }
}

pub fn apply_edit_commit(
    app: &mut App,
    commit: EditCommit,
) {
    let result = commit_edit(
        &mut app.table,
        app.cursor.row,
        app.cursor.col,
        &commit.text,
    );

    if let Err(err) = result {
        app.status = Some(err);
    }
}

pub fn apply_insert_column_commit(
    app: &mut App,
    commit: InsertColumnCommit,
) {
    let insert_at = match commit.position {
        ColumnInsertPosition::Before => app.cursor.col,
        ColumnInsertPosition::After => app.cursor.col + 1,
    };

    app.table.insert_column(
        insert_at,
        Column {
            name: commit.name,
            col_type: commit.col_type,
        },
    );

    app.cursor.col = insert_at;
}

