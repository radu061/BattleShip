use crate::desenare::desenare_stare;
use crate::nave::Tabla;
use crossterm::event::{self, Event, KeyCode};
use ratatui::Terminal;
use ratatui::prelude::CrosstermBackend;
use ratatui::widgets::Clear;
use std::io::Stdout;
use std::thread;
use std::time::Duration;
pub fn sfarsit(
    mut terminal: Terminal<CrosstermBackend<Stdout>>,
    tabla: &Tabla,
    timp: &Duration,
    cond: bool,
) -> Result<bool, String> {
    let mut stare = "Congratulation! \n The laurels of victory are yours!".to_string();
    if !cond {
        stare = "The taste of defeat is bitter!".to_string();
    }
    terminal
        .draw(|f| desenare_stare(f, &stare, tabla, timp))
        .map_err(|e| e.to_string())?;
    thread::sleep(Duration::from_secs(3));
    loop {
        if let Event::Key(key_event) = event::read().map_err(|e| e.to_string())?
            && key_event.code == KeyCode::Esc
        {
            terminal
                .draw(|f| {
                    f.render_widget(Clear, f.area());
                })
                .map_err(|e| e.to_string())?;
            return Ok(cond);
        }
    }
}
