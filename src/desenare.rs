use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::ToSpan,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Widget},
};
use std::time::Duration;

use crate::nave::{Celula, Tabla};
fn desenare_radar_inamic(frame: &mut Frame, zona: Rect, radar: &[[Celula; 10]; 10]) {
    let pozitii = Row::new((0..=10).map(|i| {
        if i == 0 {
            Cell::from(" ")
        } else if i < 10 {
            let mut celula = String::new();
            celula.push(' ');
            celula += &i.to_string();
            Cell::from(celula)
        } else {
            Cell::from(i.to_string())
        }
    }));

    let mut rows: Vec<Row> = radar
        .iter()
        .enumerate()
        .map(|(r, row)| {
            let mut cells: Vec<Cell> = Vec::new();
            cells.push(Cell::from((r + 1).to_string()));
            for celula in row.iter() {
                let symbol = match celula {
                    Celula::Liber => "🔵",
                    Celula::Nava => "🟢",
                    Celula::Lovit => "🔴",
                    Celula::Ratat => "🔘",
                    Celula::Distrus => "⚫",
                };
                cells.push(Cell::from(symbol).style(match celula {
                    Celula::Lovit => Style::default().fg(Color::Red),
                    Celula::Ratat => Style::default().fg(Color::Blue),
                    Celula::Nava => Style::default().fg(Color::Green),
                    Celula::Liber => Style::default().fg(Color::Gray),
                    Celula::Distrus => Style::default().fg(Color::Black),
                }));
            }
            Row::new(cells)
        })
        .collect();
    rows.insert(0, pozitii);
    let widths = [Constraint::Length(2); 11];
    let table = Table::new(rows, &widths).block(
        Block::default()
            .fg(Color::Red)
            .title("Enemy Radar".to_span().into_centered_line())
            .borders(Borders::ALL),
    );
    frame.render_widget(table, zona);
}

fn desenare_radar_prieten(frame: &mut Frame, zona: Rect, radar: &[[Celula; 10]; 10]) {
    let pozitii = Row::new((0..=10).map(|i| {
        if i == 0 {
            Cell::from(" ")
        } else if i < 10 {
            let mut celula = String::new();
            celula.push(' ');
            celula += &i.to_string();
            Cell::from(celula)
        } else {
            Cell::from(i.to_string())
        }
    }));

    let mut rows: Vec<Row> = radar
        .iter()
        .enumerate()
        .map(|(r, row)| {
            let mut cells: Vec<Cell> = Vec::new();
            cells.push(Cell::from((r + 1).to_string()));
            for celula in row.iter() {
                let symbol = match celula {
                    Celula::Liber => "🔵",
                    Celula::Nava => "🟢",
                    Celula::Lovit => "🔴",
                    Celula::Ratat => "🔘",
                    Celula::Distrus => "⚫",
                };
                cells.push(Cell::from(symbol).style(match celula {
                    Celula::Lovit => Style::default().fg(Color::Red),
                    Celula::Ratat => Style::default().fg(Color::Blue),
                    Celula::Nava => Style::default().fg(Color::Green),
                    Celula::Liber => Style::default().fg(Color::Gray),
                    Celula::Distrus => Style::default().fg(Color::Black),
                }));
            }
            Row::new(cells)
        })
        .collect();
    rows.insert(0, pozitii);
    let widths = [Constraint::Length(2); 11];
    let table = Table::new(rows, &widths).block(
        Block::default()
            .fg(Color::Green)
            .title("Friendly Radar".to_span().into_centered_line())
            .borders(Borders::ALL),
    );
    frame.render_widget(table, zona);
}

pub fn desenare_input(frame: &mut Frame, state: &str, input: &str, tabla: &Tabla, timp: &Duration) {
    let outer_layer = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    let radar = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer_layer[0]);

    let info = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(50),
        ])
        .split(outer_layer[1]);

    desenare_radar_inamic(frame, radar[1], &tabla.radar_inamic);
    desenare_radar_prieten(frame, radar[0], &tabla.radar_prieten);

    Paragraph::new(format!("{}s", timp.as_secs(),))
        .block(Block::default().borders(Borders::ALL).title("Timp ramas"))
        .render(info[1], frame.buffer_mut());

    Paragraph::new(input)
        .block(
            Block::bordered()
                .fg(Color::Cyan)
                .title("Input".to_span().into_centered_line()),
        )
        .render(info[2], frame.buffer_mut());
    Paragraph::new(state)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .fg(Color::Yellow)
                .title("State".to_span().into_centered_line()),
        )
        .render(info[0], frame.buffer_mut());
}

pub fn desenare_stare(frame: &mut Frame, state: &str, tabla: &Tabla, timp: &Duration) {
    let outer_layer = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    let radar = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer_layer[0]);

    desenare_radar_inamic(frame, radar[1], &tabla.radar_inamic);
    desenare_radar_prieten(frame, radar[0], &tabla.radar_prieten);

    let info = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(outer_layer[1]);
    Paragraph::new(format!("{}s", timp.as_secs(),))
        .block(Block::default().borders(Borders::ALL).title("Timp ramas"))
        .render(info[1], frame.buffer_mut());

    Paragraph::new(state)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .fg(Color::Yellow)
                .title("State".to_span().into_centered_line()),
        )
        .render(info[0], frame.buffer_mut());
}
