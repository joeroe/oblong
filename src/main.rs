use std::io;

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod input;
mod model;
mod ui;

use app::{App, Cursor, Mode};
use model::{Column, ColumnType, Table};
use input::{ControllerAction};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initial table: single integer column "id"
    let columns = vec![Column {
        name: "id".into(),
        col_type: ColumnType::Integer,
    }];

    let mut table = Table::new(columns);
    table.push_empty_row();

    let mut app = App {
        table,
        cursor: Cursor { row: 0, col: 0 },
        mode: Mode::Normal,
        status: None,
    };

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match input::handle_key(key, &mut app) {
                ControllerAction::Continue => {}
                ControllerAction::Quit => break,
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
