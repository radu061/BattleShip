use crate::nave::Celula;
use crate::nave::inside;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::ToSpan,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Widget},
};
fn validare_oriz(table: &[[Alegere; 10]; 10], x: usize, y: usize, i: i32, lg: i32) -> bool {
    if inside(x as i32, y as i32 + i) {
        if table[x][(y as i32 + i) as usize] == Alegere::Nava {
            return false;
        }
        if inside(x as i32 - 1, y as i32 + i - 1)
            && table[x - 1][(y as i32 + i - 1) as usize] == Alegere::Nava
        {
            return false;
        }
        if inside(x as i32 - 1, y as i32 + i)
            && table[x - 1][(y as i32 + i) as usize] == Alegere::Nava
        {
            return false;
        }
        if inside(x as i32 - 1, y as i32 + i + 1)
            && table[x - 1][(y as i32 + i + 1) as usize] == Alegere::Nava
        {
            return false;
        }
        if inside(x as i32 + 1, y as i32 + i - 1)
            && table[x + 1][(y as i32 + i - 1) as usize] == Alegere::Nava
        {
            return false;
        }
        if inside(x as i32 + 1, y as i32 + i)
            && table[x + 1][(y as i32 + i) as usize] == Alegere::Nava
        {
            return false;
        }
        if inside(x as i32 + 1, y as i32 + i + 1)
            && table[x + 1][(y as i32 + i + 1) as usize] == Alegere::Nava
        {
            return false;
        }
        if i == 0
            && inside(x as i32, y as i32 + i - 1)
            && table[x][(y as i32 + i - 1) as usize] == Alegere::Nava
        {
            return false;
        }
        if i == lg
            && inside(x as i32, y as i32 + i + 1)
            && table[x][(y as i32 + i + 1) as usize] == Alegere::Nava
        {
            return false;
        }
        return true;
    }
    false
}
fn validare_vert(table: &[[Alegere; 10]; 10], x: usize, y: usize, i: i32, lg: i32) -> bool {
    if inside(x as i32 + i, y as i32) {
        if table[(x as i32 + i) as usize][y] == Alegere::Nava {
            return false;
        }
        if inside(x as i32 + i - 1, y as i32 - 1)
            && table[(x as i32 + i - 1) as usize][y - 1] == Alegere::Nava
        {
            return false;
        }
        if inside(x as i32 + i, y as i32 - 1)
            && table[(x as i32 + i) as usize][y - 1] == Alegere::Nava
        {
            return false;
        }
        if inside(x as i32 + i + 1, y as i32 - 1)
            && table[(x as i32 + i + 1) as usize][y - 1] == Alegere::Nava
        {
            return false;
        }
        if inside(x as i32 + i - 1, y as i32 + 1)
            && table[(x as i32 + i - 1) as usize][y + 1] == Alegere::Nava
        {
            return false;
        }
        if inside(x as i32 + i, y as i32 + 1)
            && table[(x as i32 + i) as usize][y + 1] == Alegere::Nava
        {
            return false;
        }
        if inside(x as i32 + i + 1, y as i32 + 1)
            && table[(x as i32 + i + 1) as usize][y + 1] == Alegere::Nava
        {
            return false;
        }
        if i == 0
            && inside(x as i32 + i - 1, y as i32)
            && table[(x as i32 + i - 1) as usize][y] == Alegere::Nava
        {
            return false;
        }
        if i == lg
            && inside(x as i32 + i + 1, y as i32)
            && table[(x as i32 + i + 1) as usize][y] == Alegere::Nava
        {
            return false;
        }
        return true;
    }
    false
}
fn verificare(table: &[[Alegere; 10]; 10], x: usize, y: usize, dir: Direction, lg: i32) -> bool {
    for i in 0..=lg {
        if dir == Direction::Horizontal {
            if !validare_oriz(table, x, y, i, lg) {
                return false;
            }
        } else if !validare_vert(table, x, y, i, lg) {
            return false;
        }
    }
    true
}
fn asig_oriz(table: &mut [[Alegere; 10]; 10], x: usize, y: usize, i: i32, lg: i32) {
    table[x][(y as i32 + i) as usize] = Alegere::Nava;
    if inside(x as i32 - 1, y as i32 + i - 1) {
        table[x - 1][(y as i32 + i - 1) as usize] = Alegere::Arie;
    }
    if inside(x as i32 - 1, y as i32) {
        table[x - 1][(y as i32 + i) as usize] = Alegere::Arie;
    }
    if inside(x as i32 - 1, y as i32 + i + 1) {
        table[x - 1][(y as i32 + i + 1) as usize] = Alegere::Arie;
    }
    if inside(x as i32 + 1, y as i32 + i - 1) {
        table[x + 1][(y as i32 + i - 1) as usize] = Alegere::Arie;
    }
    if inside(x as i32 + 1, y as i32) {
        table[x + 1][(y as i32 + i) as usize] = Alegere::Arie;
    }
    if inside(x as i32 + 1, y as i32 + i + 1) {
        table[x + 1][(y as i32 + i + 1) as usize] = Alegere::Arie;
    }
    if i == 0 && inside(x as i32, y as i32 + i - 1) {
        table[x][(y as i32 + i - 1) as usize] = Alegere::Arie;
    }
    if i == lg && inside(x as i32, y as i32 + i + 1) {
        table[x][(y as i32 + i + 1) as usize] = Alegere::Arie;
    }
}
fn asig_vert(table: &mut [[Alegere; 10]; 10], x: usize, y: usize, i: i32, lg: i32) {
    table[(x as i32 + i) as usize][y] = Alegere::Nava;
    if inside(x as i32 + i - 1, y as i32 - 1) {
        table[(x as i32 + i - 1) as usize][y - 1] = Alegere::Arie;
    }
    if inside(x as i32 + i, y as i32 - 1) {
        table[(x as i32 + i) as usize][y - 1] = Alegere::Arie;
    }
    if inside(x as i32 + i + 1, y as i32 - 1) {
        table[(x as i32 + i + 1) as usize][y - 1] = Alegere::Arie;
    }
    if inside(x as i32 + i - 1, y as i32 + 1) {
        table[(x as i32 + i - 1) as usize][y + 1] = Alegere::Arie;
    }
    if inside(x as i32 + i, y as i32 + 1) {
        table[(x as i32 + i) as usize][y + 1] = Alegere::Arie;
    }
    if inside(x as i32 + i + 1, y as i32 + 1) {
        table[(x as i32 + i + 1) as usize][y + 1] = Alegere::Arie;
    }
    if i == 0 && inside(x as i32 - 1, y as i32) {
        table[x - 1][y] = Alegere::Arie;
    }
    if i == lg && inside(x as i32 + i + 1, y as i32) {
        table[(x as i32 + i + 1) as usize][y] = Alegere::Arie;
    }
}
fn asignare(table: &mut [[Alegere; 10]; 10], x: usize, y: usize, dir: Direction, lg: i32) {
    for i in 0..=lg {
        if dir == Direction::Horizontal {
            asig_oriz(table, x, y, i, lg);
        } else {
            asig_vert(table, x, y, i, lg);
        }
    }
}
fn desenare_alegere(
    frame: &mut Frame,
    state: &str,
    table: &[[Alegere; 10]; 10],
    x: usize,
    y: usize,
    dir: Direction,
    lg: i32,
) {
    let outer_layer = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.area());

    Paragraph::new(state)
        .block(
            Block::bordered()
                .fg(Color::Cyan)
                .title("Stare".to_span().into_centered_line()),
        )
        .render(outer_layer[0], frame.buffer_mut());

    let zona = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(vec![
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(outer_layer[1]);

    let rows: Vec<Row> = table
        .iter()
        .enumerate()
        .map(|(r, row)| {
            let mut cells: Vec<Cell> = Vec::new();
            cells.push(Cell::from((r + 1).to_string()));

            for (c, celula) in row.iter().enumerate() {
                let mut is_selected = false;
                for i in 0..=lg {
                    let (cx, cy) = if dir == Direction::Horizontal {
                        (x, y + i as usize)
                    } else {
                        (x + i as usize, y)
                    };

                    if cx == r && cy == c {
                        is_selected = true;
                    }
                }

                let symbol = if is_selected {
                    "🟡"
                } else {
                    match celula {
                        Alegere::Liber => "🔵",
                        Alegere::Nava => "🟢",
                        Alegere::Arie => "🔘",
                    }
                };

                let style = if is_selected {
                    Style::default().fg(Color::Yellow)
                } else {
                    match celula {
                        Alegere::Liber => Style::default().fg(Color::Blue),
                        Alegere::Nava => Style::default().fg(Color::Green),
                        Alegere::Arie => Style::default().fg(Color::Gray),
                    }
                };

                cells.push(Cell::from(symbol).style(style));
            }
            Row::new(cells)
        })
        .collect();

    let widths = [Constraint::Length(2); 11];
    let table = Table::new(rows, &widths).block(
        Block::default()
            .fg(Color::Green)
            .title("Battlefield Positions".to_span().into_centered_line())
            .borders(Borders::ALL),
    );
    frame.render_widget(table, zona[1]);
}
pub fn alegerenave(radar: &mut [[Celula; 10]; 10]) -> Result<(), String> {
    let mut tabla = [[Alegere::Liber; 10]; 10];
    let mut terminal = ratatui::init();
    for i in (0..10).rev() {
        let mut x = 0_usize;
        let mut y = 0_usize;
        let mut dir = Direction::Horizontal;
        let mut lg = 0;
        let mut stare = "Choose Corvette Placement".to_string();
        if i >= 4 {
            lg = 1;
            stare = "Choose Frigate Placement".to_string();
        }
        if i >= 7 {
            lg = 2;
            stare = "Choose Cruiser Placement".to_string();
        }
        if i == 9 {
            lg = 3;
            stare = "Choose Flagship Placement".to_string();
        }
        loop {
            terminal
                .draw(|f| desenare_alegere(f, &stare, &tabla, x, y, dir, lg))
                .map_err(|e| e.to_string())?;
            if let Event::Key(KeyEvent { code, kind, .. }) =
                event::read().map_err(|e| e.to_string())?
            {
                if kind != KeyEventKind::Press {
                    continue;
                }
                match code {
                    KeyCode::Up => {
                        if inside(x as i32 - 1, y as i32) {
                            x -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if dir == Direction::Vertical {
                            if inside(x as i32 + lg + 1, y as i32) {
                                x += 1;
                            }
                        } else if inside(x as i32 + 1, y as i32) {
                            x += 1;
                        }
                    }
                    KeyCode::Left => {
                        if inside(x as i32, y as i32 - 1) {
                            y -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if dir == Direction::Horizontal {
                            if inside(x as i32, y as i32 + lg + 1) {
                                y += 1;
                            }
                        } else if inside(x as i32, y as i32 + 1) {
                            y += 1;
                        }
                    }
                    KeyCode::Char('r') => {
                        if dir == Direction::Horizontal {
                            dir = Direction::Vertical;
                        } else {
                            dir = Direction::Horizontal;
                        }
                    }
                    KeyCode::Enter => {
                        if verificare(&tabla, x, y, dir, lg) {
                            asignare(&mut tabla, x, y, dir, lg);
                            break;
                        } else {
                            stare = "Pozitie incorecta".to_string();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    ratatui::restore();
    for i in 0..10 {
        for j in 0..10 {
            radar[i][j] = match tabla[i][j] {
                Alegere::Nava => Celula::Nava,
                _ => Celula::Liber,
            };
        }
    }
    Ok(())
}
#[derive(Clone, Copy, PartialEq)]
enum Alegere {
    Nava,
    Arie,
    Liber,
}
