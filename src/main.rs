mod alegere;
mod connect;
mod desenare;
mod joc;
mod nave;
mod sfarsit;
use std::env;
fn main() {
    match connect::initializare_p2p(env::args().collect()) {
        Ok(true) => {
            ratatui::restore();
            println!("Castigator");
        }
        Ok(false) => {
            ratatui::restore();
            println!("Pierzator");
        }
        Err(eroare) => {
            ratatui::restore();
            println!("{}", eroare);
        }
    }
}
