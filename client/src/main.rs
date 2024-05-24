use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use secure_common::sign;
use secure_common::prelude::{connection::{Connection, Type}};

fn main() -> Result<()> {
    let owm_sk_name = std::env::args().nth(1).unwrap_or_else(|| "client".into());
    let other_pk = std::env::args().nth(2).unwrap_or_else(|| "server".into());

    let own_sk = sign::read_sk(format!("{}.sk", owm_sk_name))?;
    let other_pk = sign::read_pk(format!("{}.pk", other_pk))?;
    
    let mut stream = TcpStream::connect("localhost:12345")?;
    let mut conn = Connection::establish(&mut stream, &own_sk, &other_pk, Type::Client)?;
    loop {
        let bytes = conn.receive()?;
        if !bytes.is_empty() {
            println!("Message from server: {:?}", String::from_utf8_lossy(&bytes));
            thread::sleep(Duration::from_millis(300));
            conn.send(b"01234567890abcdef")?;
        }
    }
}
