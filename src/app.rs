// src/app.rs
// Application state

use crate::model::Table;

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub enum Mode {
    Normal,
    Editing(EditBuffer),
    InsertColumn(ColumnInsertState),
}

#[derive(Debug)]
pub struct EditBuffer {
    pub text: String,
}

#[derive(Debug)]
pub enum ColumnInsertState {
    Naming {
        position: ColumnInsertPosition,
        buffer: String,
    },
    Typing {
        position: ColumnInsertPosition,
        name: String,
    },
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ColumnInsertPosition {
    Before,
    After,
}

pub struct App {
    pub table: Table,
    pub cursor: Cursor,
    pub mode: Mode,
    pub status: Option<String>,
}
