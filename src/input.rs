// src/input.rs
// User input handling

use crossterm::event::{KeyCode, KeyEvent};
use crate::app::{App, Mode, EditBuffer};
use crate::model::{commit_edit};

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
        }
    }
}

fn handle_normal_mode(key: KeyEvent, app: &mut App) -> bool {
    match key.code {
        // Quit (q)
        KeyCode::Char('q') => return true,

        // Navigation (arrows or hjkl)
        KeyCode::Char('h') | KeyCode::Left  => move_cursor(app, 0, -1),
        KeyCode::Char('l') | KeyCode::Right => move_cursor(app, 0,  1),
        KeyCode::Char('k') | KeyCode::Up    => move_cursor(app, -1, 0),
        KeyCode::Char('j') | KeyCode::Down  => move_cursor(app,  1, 0),

        // Enter edit mode (i)
        KeyCode::Char('i') | KeyCode::Enter => {
            let cell = &app.table.rows[app.cursor.row][app.cursor.col];
            let text = cell.to_edit_string();
            app.mode = Mode::Editing(EditBuffer { text });
        }

        // Insert below and enter edit mode (o)
        KeyCode::Char('o') => {
            let insert_at = if app.table.height() == 0 {
                0
            } else {
                app.cursor.row + 1
            };

            app.table.insert_empty_row(insert_at);

            app.cursor.row = insert_at;

            app.mode = Mode::Editing(EditBuffer {
                text: String::new(),
            });
        }

        // Insert above and enter edit mode (O)
        KeyCode::Char('O') => {
            let insert_at = app.cursor.row.min(app.table.height());

            app.table.insert_empty_row(insert_at);

            app.cursor.row = insert_at;

            app.mode = Mode::Editing(EditBuffer {
                text: String::new(),
            });
        }

        _ => {}
    }

    false
}

fn move_cursor(app: &mut App, d_row: isize, d_col: isize) {
    let max_row = app.table.height().saturating_sub(1) as isize;
    let max_col = app.table.width().saturating_sub(1) as isize;

    let new_row = (app.cursor.row as isize + d_row).clamp(0, max_row);
    let new_col = (app.cursor.col as isize + d_col).clamp(0, max_col);

    app.cursor.row = new_row as usize;
    app.cursor.col = new_col as usize;
}

enum EditResult {
    Continue,
    Commit,
    Cancel,
}

fn handle_edit_mode(
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{App, Cursor};
    use crate::model::{Column, ColumnType, Table};

    fn sample_app(rows: usize, cols: usize) -> App {
        let columns = (0..cols)
            .map(|i| Column {
                name: format!("c{}", i),
                col_type: ColumnType::Integer,
            })
            .collect();

        let mut table = Table::new(columns);
        for _ in 0..rows {
            table.push_empty_row();
        }

        App {
            table,
            cursor: Cursor { row: 0, col: 0 },
            mode: Mode::Normal,
            status: None,
        }
    }

    #[test]
    fn cursor_moves_right() {
        let mut app = sample_app(3, 3);
        move_cursor(&mut app, 0, 1);
        assert_eq!(app.cursor.col, 1);
    }

    #[test]
    fn cursor_clamps_left() {
        let mut app = sample_app(3, 3);
        move_cursor(&mut app, 0, -1);
        assert_eq!(app.cursor.col, 0);
    }

    #[test]
    fn cursor_clamps_bottom() {
        let mut app = sample_app(2, 2);
        app.cursor.row = 1;
        move_cursor(&mut app, 1, 0);
        assert_eq!(app.cursor.row, 1);
    }

    #[test]
    fn empty_table_does_not_move_cursor() {
        let mut app = sample_app(0, 1);
        move_cursor(&mut app, 1, 1);
        assert_eq!(app.cursor.row, 0);
        assert_eq!(app.cursor.col, 0);
    }
}
