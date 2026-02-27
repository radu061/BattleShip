use crate::joc;
use std::net::TcpListener;
pub fn initializare_p2p(a: Vec<String>) -> Result<bool, String> {
    if a.len() < 2 {
        return Err(
            "[Inceput]Argument incorect\nStructura este [Host] sau [Join] [IP:Port]".to_string(),
        );
    }
    if a[1] == "Host" {
        let Ok(serverfd) = TcpListener::bind("127.0.0.1:2908") else {
            return Err("Eroare la bind".to_string());
        };
        println!("Asteptare jucator");
        let Ok((sd, addr)) = serverfd.accept() else {
            return Err("Eroare la accept".to_string());
        };
        println!("Acceptare conexiune de la adresa {}", addr);
        joc::jocloop(sd, true)
    } else if a[1] == "Join" {
        if a.len() != 3 {
            return Err(
                "[Join] Argument incorect\nStructura este [Host] sau [Join] [IP:Port]".to_string(),
            );
        }
        let Ok(sd) = std::net::TcpStream::connect(&a[2]) else {
            return Err("Eroare la conectare".to_string());
        };

        println!("Connectare la jucator cu adresa {}", &a[2]);
        joc::jocloop(sd, false)
    } else {
        Err("[Final] Argument incorect\nStructura este [Host] sau [Join] [IP:Port]".to_string())
    }
}
