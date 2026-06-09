// src/input.rs
// User input handling

use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Mode};

pub fn handle_key(key: KeyEvent, app: &mut App) -> bool {
    match app.mode {
        Mode::Normal => handle_normal_mode(key, app),
        Mode::Editing(_) => false,
    }
}

fn handle_normal_mode(key: KeyEvent, app: &mut App) -> bool {
    match key.code {
        KeyCode::Char('q') => return true,

        KeyCode::Char('h') | KeyCode::Left  => move_cursor(app, 0, -1),
        KeyCode::Char('l') | KeyCode::Right => move_cursor(app, 0,  1),
        KeyCode::Char('k') | KeyCode::Up    => move_cursor(app, -1, 0),
        KeyCode::Char('j') | KeyCode::Down  => move_cursor(app,  1, 0),

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
