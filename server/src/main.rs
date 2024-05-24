use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use anyhow::Result;

use secure_common::prelude::connection::{Connection, Type};
use secure_common::sign;

fn handle_client(
    stream: &mut TcpStream,
    address: &SocketAddr,
    own_sk: &sign::SecretKey,
    other_pk: &sign::PublicKey,
) -> Result<()> {
    println!("Establishing connection to {address}");
    let mut conn = Connection::establish(stream, own_sk, other_pk, Type::Server)?;
    println!("Connection to {address} established");
    conn.send(b"....----....----")?;
    loop {
        let bytes = conn.receive()?;
        if !bytes.is_empty() {
            println!("Message from client: {:?}", String::from_utf8_lossy(&bytes));
            thread::sleep(Duration::from_secs(2));
            conn.send(b"fedcba9876543210")?;
        }
    }
}

fn main() -> Result<()> {
    let owm_sk_name = std::env::args().nth(1).unwrap_or_else(|| "server".into());
    let other_pk = std::env::args().nth(2).unwrap_or_else(|| "client".into());

    let own_sk = sign::read_sk(format!("{}.sk", owm_sk_name))?;
    let other_pk = sign::read_pk(format!("{}.pk", other_pk))?;
    println!("Successfully read keys");
    let listener = TcpListener::bind("localhost:12345")?;
    println!("Listening...");
    thread::scope(move |scope| loop {
        let (mut stream, address) = match listener.accept() {
            Ok(ok) => ok,
            Err(err) => {
                eprintln!("Error: {err}");
                break;
            }
        };
        scope.spawn(move || {
            if let Err(err) = handle_client(&mut stream, &address, &own_sk, &other_pk) {
                eprintln!("Error: {err}");
            }
        });
    });
    Ok(())
}
