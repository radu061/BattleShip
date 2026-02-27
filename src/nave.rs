use serde::{Deserialize, Serialize};
pub fn inside(x: i32, y: i32) -> bool {
    (0..=9).contains(&x) && (0..=9).contains(&y)
}

pub fn integritate(radar: &[[Celula; 10]; 10], x: usize, y: usize) -> bool {
    let movx = [0, 0, 1, -1];
    let movy = [-1, 1, 0, 0];
    for i in 0..4 {
        let mut pozx = x as i32 + movx[i];
        let mut pozy = y as i32 + movy[i];
        while inside(pozx, pozy)
            && radar[pozx as usize][pozy as usize] != Celula::Liber
            && radar[pozx as usize][pozy as usize] != Celula::Ratat
        {
            if radar[pozx as usize][pozy as usize] == Celula::Nava {
                return true;
            }
            pozx += movx[i];
            pozy += movy[i];
        }
    }
    false
}
pub fn distrugere(radar: &mut [[Celula; 10]; 10], x: usize, y: usize) {
    let movx = [0, 0, 1, -1];
    let movy = [-1, 1, 0, 0];
    let deplx = [0, 0, 1, -1, -1, 1, 1, -1];
    let deply = [-1, 1, 0, 0, -1, -1, 1, 1];
    for j in 0..8 {
        if inside(x as i32 + deplx[j], y as i32 + deply[j])
            && radar[(x as i32 + deplx[j]) as usize][(y as i32 + deply[j]) as usize]
                != Celula::Distrus
            && radar[(x as i32 + deplx[j]) as usize][(y as i32 + deply[j]) as usize]
                != Celula::Lovit
        {
            radar[(x as i32 + deplx[j]) as usize][(y as i32 + deply[j]) as usize] = Celula::Ratat;
        }
    }
    radar[x][y] = Celula::Distrus;
    for i in 0..4 {
        let mut pozx = x as i32 + movx[i];
        let mut pozy = y as i32 + movy[i];
        while inside(pozx, pozy) && radar[pozx as usize][pozy as usize] == Celula::Lovit {
            radar[pozx as usize][pozy as usize] = Celula::Distrus;
            for j in 0..8 {
                if inside(pozx + deplx[j], pozy + deply[j])
                    && radar[(pozx + deplx[j]) as usize][(pozy + deply[j]) as usize]
                        != Celula::Distrus
                    && radar[(pozx + deplx[j]) as usize][(pozy + deply[j]) as usize]
                        != Celula::Lovit
                {
                    radar[(pozx + deplx[j]) as usize][(pozy + deply[j]) as usize] = Celula::Ratat;
                }
            }
            pozx += movx[i];
            pozy += movy[i];
        }
    }
}
pub struct Tabla {
    pub radar_inamic: [[Celula; 10]; 10],
    pub radar_prieten: [[Celula; 10]; 10],
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Coordonate {
    pub x: i32,
    pub y: i32,
    pub cond: Conditie,
}
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Conditie {
    Ratat,
    Lovit,
    Distrus,
    Castig,
}
#[derive(Clone, Copy, PartialEq)]
pub enum Celula {
    Lovit,
    Ratat,
    Liber,
    Nava,
    Distrus,
}
