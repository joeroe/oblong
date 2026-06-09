// src/ui.rs
// UI rendering

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::app::{App, Mode, ColumnInsertState};
use crate::model::{CellValue, ColumnType};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Min(1),     // table
        Constraint::Length(2),  // status bar with border
    ])
    .split(f.size());

    draw_table(f, app, chunks[0]);
    draw_status(f, app, chunks[1]);
}

fn draw_table(f: &mut Frame, app: &App, area: Rect) {
    let table = &app.table;

    let widths: Vec<Constraint> =
        std::iter::once(Constraint::Length(4))
        .chain(table.columns.iter().map(|_| Constraint::Length(12)))
        .collect();

    let header_row = Row::new(
        std::iter::once(
            Cell::from(Span::styled(
                    " ",
                    Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            ))
        )
        .chain(table.columns.iter().map(|c| {
            Cell::from(Span::styled(
                    c.name.clone(),
                    Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ))
        })),
    );

    let type_row = Row::new(
        std::iter::once(
            Cell::from("") // empty cell under the row-number column
        )
        .chain(table.columns.iter().map(|c| {
            Cell::from(render_column_type(c.col_type)).style(
                Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
            )
        })),
    );

    let data_rows = table.rows.iter().enumerate().map(|(r, row)| {
        let row_number_cell = Cell::from(Span::styled(
                format!("{}", r + 1),
                Style::default().fg(Color::DarkGray),
        ));

        Row::new(
            std::iter::once(row_number_cell)
            .chain(row.iter().enumerate().map(|(c, value)| {
                let is_cursor = app.cursor.row == r && app.cursor.col == c;

                if is_cursor {
                    if let Mode::Editing(buffer) = &app.mode {
                        return Cell::from(buffer.text.clone()).style(
                            Style::default()
                            .fg(Color::Black)
                            .bg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                        );
                    }
                }

                let mut cell = Cell::from(render_cell(value));

                if is_cursor {
                    cell = cell.style(
                        Style::default()
                        .fg(Color::Black)
                        .bg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD),
                    );
                } else {
                    cell = cell.style(style_for_cell(value));
                }

                cell
            })),
        )
    });


    let rows = std::iter::once(type_row).chain(data_rows);

    let widget = Table::new(rows, widths)
        .header(header_row)
        .column_spacing(1);

    f.render_widget(widget, area);
}

fn render_column_type(col_type: ColumnType) -> &'static str {
    match col_type {
        ColumnType::Integer => "<Integer>",
        ColumnType::Float => "<Float>",
        ColumnType::Text => "<Text>",
        ColumnType::Boolean => "<Boolean>",
    }
}

fn render_cell(value: &CellValue) -> String {
    match value {
        CellValue::Int(v) => v.to_string(),
        CellValue::Float(v) => v.to_string(),
        CellValue::Text(v) => v.clone(),
        CellValue::Bool(v) => v.to_string(),
        CellValue::Empty => "".into(),
    }
}

fn style_for_cell(value: &CellValue) -> Style {
    match value {
        CellValue::Empty => Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC),
        _ => Style::default().fg(Color::White),
    }
}

fn draw_status(f: &mut Frame, app: &App, area: Rect) {
    let status_text = match &app.mode {
        Mode::Normal => {
            if let Some(msg) = &app.status {
                msg.clone()
            } else {
                format!(
                    "row {} col {}",
                    app.cursor.row + 1,
                    app.cursor.col + 1
                )
            }
        }

        Mode::Editing(_) => {
            if let Some(msg) = &app.status {
                msg.clone()
            } else {
                "-- INSERT --".to_string()
            }
        }

        Mode::InsertColumn(ColumnInsertState::Naming { buffer, .. }) => {
            format!("New column name: {}_", buffer)
        }

        Mode::InsertColumn(ColumnInsertState::Typing { name, .. }) => {
            format!(
                "New column: {} | type [i]nteger [f]loat [t]ext [b]oolean", 
                name
            )
        }
    };

    let paragraph = Paragraph::new(Line::from(Span::raw(status_text)))
        .block(Block::default().borders(Borders::TOP))
        .style(Style::default().fg(Color::Gray));

    f.render_widget(paragraph, area);
}
