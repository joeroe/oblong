// src/app.rs
// Application state

use crate::model::Table;

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct EditBuffer {
    pub text: String,
}

#[derive(Debug)]
pub enum Mode {
    Normal,
    Editing(EditBuffer),
}

pub struct App {
    pub table: Table,
    pub cursor: Cursor,
    pub mode: Mode,
    pub status: Option<String>,
}
