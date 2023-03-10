use std::error::Error;

use scorched_earth_core::Board;
use scorched_earth_network::Connection;

fn main() -> Result<(), Box<dyn Error>> {
    let param = std::env::args()
        .nth(1)
        .expect("run with parameter server or client");
    match param.as_str() {
        "server" => {
            let mut conn = Connection::host("127.0.0.1:8080", b"this must be 32 characters long.", &Board::default())?;
            println!("connected");
            let res = String::from_utf8_lossy(conn.recv()?);
            println!("received {:?}", res);
            conn.send(b"yeah it does")?;
            println!("responded");
        },
        "client" => {
            let (mut conn, _board) = Connection::conn("127.0.0.1:8080", b"this must be 32 characters long.")?;
            println!("connected");
            conn.send(b"does the connection work?")?;
            println!("sent");
            let res = String::from_utf8_lossy(conn.recv()?);
            println!("received {:?}", res);
        },
        _ => {
            panic!("Parameter needs to be server or client");
        },
    }
    Ok(())
}
