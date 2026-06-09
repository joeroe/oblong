// src/ui.rs
// UI rendering

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::app::{App, Mode};
use crate::model::CellValue;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(f.size());

    draw_table(f, app, chunks[0]);
    draw_status(f, app, chunks[1]);
}

fn draw_table(f: &mut Frame, app: &App, area: Rect) {
    let table = &app.table;

    let widths: Vec<Constraint> = table
        .columns
        .iter()
        .map(|_| Constraint::Length(12))
        .collect();

    let header = Row::new(
        table.columns.iter().map(|c| {
            Cell::from(Span::styled(
                c.name.clone(),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ))
        }),
    );

    let rows = table.rows.iter().enumerate().map(|(r, row)| {
        Row::new(row.iter().enumerate().map(|(c, value)| {
            let mut style = style_for_cell(value);

            if app.cursor.row == r && app.cursor.col == c {
                style = style
                    .fg(Color::Black)
                    .bg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD);
            }

            Cell::from(Span::styled(render_cell(value), style))
        }))
    });

    let widget = Table::new(rows, widths)
        .header(header)
        .block(Block::default().title("Table").borders(Borders::ALL))
        .column_spacing(1);

    f.render_widget(widget, area);
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
    let mode = match app.mode {
        Mode::Normal => "NORMAL",
        Mode::Editing(_) => "EDIT",
    };

    let text = if let Some(msg) = &app.status {
        format!("[{}] {}", mode, msg)
    } else {
        format!(
            "[{}] row {} col {}",
            mode,
            app.cursor.row + 1,
            app.cursor.col + 1
        )
    };

    let paragraph = Paragraph::new(Line::from(Span::raw(text)))
        .block(Block::default().borders(Borders::TOP))
        .style(Style::default().fg(Color::Gray));

    f.render_widget(paragraph, area);
}
