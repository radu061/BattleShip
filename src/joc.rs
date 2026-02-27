use crate::{
    alegere, desenare,
    nave::{Celula, Conditie, Coordonate, Tabla, distrugere, integritate},
    sfarsit::sfarsit,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};
pub fn jocloop(mut sd: TcpStream, mut turn: bool) -> Result<bool, String> {
    println!("Am inceput joc");
    print!("\x1B[2J\x1B[1;1H");
    let mut limita_timp = Duration::from_secs(300);
    let mut shipct = 10;
    let mut tabla = Tabla {
        radar_inamic: [[Celula::Liber; 10]; 10],
        radar_prieten: [[Celula::Liber; 10]; 10],
    };
    let mut terminal = ratatui::init();
    alegere::alegerenave(&mut tabla.radar_prieten)?;
    let mut stare = "Waiting for other player".to_string();
    terminal
        .draw(|frame| desenare::desenare_stare(frame, &stare, &tabla, &limita_timp))
        .map_err(|e| e.to_string())?;
    if turn {
        if sd.write_all("ready".as_bytes()).is_err() {
            return Err("Eroare la scriere".to_string());
        }
        if sd.write_all(b"\n").is_err() {
            return Err("Eroare la scriere".to_string());
        }
        let mut line = String::new();
        let mut reader = BufReader::new(&sd);
        let Ok(_) = reader.read_line(&mut line) else {
            return Err("Eroare la comunicare".to_string());
        };
    } else {
        let mut line = String::new();
        let mut reader = BufReader::new(&sd);
        let Ok(_) = reader.read_line(&mut line) else {
            return Err("Eroare la comunicare".to_string());
        };
        if sd.write_all("ready".as_bytes()).is_err() {
            return Err("Eroare la scriere".to_string());
        }
        if sd.write_all(b"\n").is_err() {
            return Err("Eroare la scriere".to_string());
        }
    }
    stare = "Introduceti coordonatele [X,Y] ale tintei dorite".to_string();
    let mut input = String::new();
    loop {
        if turn {
            let mut timp = Instant::now();
            loop {
                let mut coordonata = Coordonate {
                    x: 0,
                    y: 0,
                    cond: Conditie::Ratat,
                };
                if timp.elapsed() > limita_timp {
                    coordonata.cond = Conditie::Castig;
                    coordonata.x = 0;
                    coordonata.y = 0;
                    let Ok(json_coord) = serde_json::to_string(&coordonata) else {
                        return Err("Eroare la serializare JSON".to_string());
                    };
                    if sd.write_all(json_coord.as_bytes()).is_err() {
                        return Err("Eroare la scriere".to_string());
                    }
                    if sd.write_all(b"\n").is_err() {
                        return Err("Eroare la scriere".to_string());
                    }
                    limita_timp = Duration::from_secs(0);
                    sfarsit(terminal, &tabla, &limita_timp, false)?;
                    return Ok(false);
                }
                limita_timp -= timp.elapsed();
                timp = Instant::now();
                terminal
                    .draw(|f| desenare::desenare_input(f, &stare, &input, &tabla, &limita_timp))
                    .map_err(|e| e.to_string())?;
                if let Event::Key(KeyEvent { code, kind, .. }) =
                    event::read().map_err(|e| e.to_string())?
                {
                    if kind != KeyEventKind::Press {
                        continue;
                    }
                    match code {
                        KeyCode::Char(c) => input.push(c),
                        KeyCode::Backspace => {
                            if !input.is_empty() {
                                input.pop();
                            }
                        }
                        KeyCode::Esc => {
                            coordonata = Coordonate {
                                x: 0,
                                y: 0,
                                cond: Conditie::Castig,
                            };
                            let Ok(json_coord) = serde_json::to_string(&coordonata) else {
                                return Err("Eroare la serializare JSON".to_string());
                            };
                            if sd.write_all(json_coord.as_bytes()).is_err() {
                                return Err("Eroare la scriere".to_string());
                            }
                            if sd.write_all(b"\n").is_err() {
                                return Err("Eroare la scriere".to_string());
                            }
                            sfarsit(terminal, &tabla, &limita_timp, false)?;
                            return Ok(false);
                        }
                        KeyCode::Enter => {
                            let a: Vec<&str> = input.split_whitespace().collect();
                            if a.len() != 2 {
                                stare = "Introduceti 2 coordonate valide".to_string();
                                continue;
                            }
                            let Ok(x): Result<i32, _> = a[0].parse() else {
                                stare = "Prima coordonata incorecta".to_string();
                                continue;
                            };
                            if !(1..=10).contains(&x) {
                                stare = "Prima coordonata incorecta".to_string();
                                continue;
                            }
                            let Ok(y): Result<i32, _> = a[1].parse() else {
                                stare = "A doua coordonata incorecta".to_string();
                                continue;
                            };
                            if !(1..=10).contains(&y) {
                                stare = "A doua coordonata incorecta".to_string();
                                continue;
                            }
                            if tabla.radar_inamic[(x - 1) as usize][(y - 1) as usize]
                                != Celula::Liber
                            {
                                stare = "Aceasta pozitie deja a fost lovita".to_string();
                                continue;
                            }
                            coordonata = Coordonate {
                                x,
                                y,
                                cond: Conditie::Ratat,
                            };
                            let Ok(json_coord) = serde_json::to_string(&coordonata) else {
                                return Err("Eroare la serializare JSON".to_string());
                            };
                            if sd.write_all(json_coord.as_bytes()).is_err() {
                                return Err("Eroare la scriere".to_string());
                            }
                            if sd.write_all(b"\n").is_err() {
                                return Err("Eroare la scriere".to_string());
                            }

                            let mut line = String::new();
                            let mut reader = BufReader::new(&sd);
                            let Ok(_) = reader.read_line(&mut line) else {
                                return Err("Eroare la comunicare".to_string());
                            };
                            let Ok(coordonate) = serde_json::from_str::<Coordonate>(&line) else {
                                return Err("Eroare la serializare JSON".to_string());
                            };
                            match coordonate.cond {
                                Conditie::Lovit => {
                                    tabla.radar_inamic[(x - 1) as usize][(y - 1) as usize] =
                                        Celula::Lovit;
                                }
                                Conditie::Ratat => {
                                    tabla.radar_inamic[(x - 1) as usize][(y - 1) as usize] =
                                        Celula::Ratat;
                                }
                                Conditie::Castig => {
                                    sfarsit(terminal, &tabla, &limita_timp, true)?;
                                    return Ok(true);
                                }
                                Conditie::Distrus => distrugere(
                                    &mut tabla.radar_inamic,
                                    (x - 1) as usize,
                                    (y - 1) as usize,
                                ),
                            }
                            input.clear();
                            break;
                        }
                        _ => {}
                    }
                }
            }

            turn = false;
        } else {
            stare = "Tura jucatorului advers".to_string();
            terminal
                .draw(|frame| desenare::desenare_stare(frame, &stare, &tabla, &limita_timp))
                .map_err(|e| e.to_string())?;
            let mut line = String::new();
            let mut reader = BufReader::new(&sd);
            let Ok(_) = reader.read_line(&mut line) else {
                return Err("Eroare la comunicare".to_string());
            };
            let Ok(coordonate) = serde_json::from_str::<Coordonate>(&line) else {
                return Err("Eroare la serializare JSON".to_string());
            };
            if coordonate.cond == Conditie::Castig {
                sfarsit(terminal, &tabla, &limita_timp, true)?;
                return Ok(true);
            }
            stare = format!("Coordonate receptionate: {} {}", coordonate.x,coordonate.y);
            let mut raspuns = coordonate;
            if raspuns.cond == Conditie::Castig {
                return Ok(true);
            }
            match tabla.radar_prieten[(raspuns.x - 1) as usize][(raspuns.y - 1) as usize] {
                Celula::Liber => {
                    tabla.radar_prieten[(raspuns.x - 1) as usize][(raspuns.y - 1) as usize] =
                        Celula::Ratat;
                    raspuns.cond = Conditie::Ratat;
                }
                _ => {
                    tabla.radar_prieten[(raspuns.x - 1) as usize][(raspuns.y - 1) as usize] =
                        Celula::Lovit;
                    if !integritate(
                        &tabla.radar_prieten,
                        (raspuns.x - 1) as usize,
                        (raspuns.y - 1) as usize,
                    ) {
                        raspuns.cond = Conditie::Distrus;
                        distrugere(
                            &mut tabla.radar_prieten,
                            (raspuns.x - 1) as usize,
                            (raspuns.y - 1) as usize,
                        );
                        shipct -= 1;
                        if shipct == 0 {
                            raspuns.cond = Conditie::Castig;
                        }
                    } else {
                        raspuns.cond = Conditie::Lovit;
                    }
                }
            }
            let Ok(json_coord) = serde_json::to_string(&raspuns) else {
                return Err("Eroare la serializare JSON".to_string());
            };
            if sd.write_all(json_coord.as_bytes()).is_err() {
                return Err("Eroare la scriere".to_string());
            }
            if sd.write_all(b"\n").is_err() {
                return Err("Eroare la scriere".to_string());
            }
            if raspuns.cond == Conditie::Castig {
                sfarsit(terminal, &tabla, &limita_timp, false)?;
                return Ok(false);
            }
            terminal
                .draw(|frame| desenare::desenare_stare(frame, &stare, &tabla, &limita_timp))
                .map_err(|e| e.to_string())?;
            turn = true;
        }
    }
}
