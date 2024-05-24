use std::net::TcpStream;
use anyhow::Error;
use crate::encryption::Encrypted;
use super::logger::Logger;
use crate::sign;

pub enum Type {
    Server,
    Client
}

pub struct Connection<'a> {
    stream: Encrypted<&'a mut TcpStream>,
    logger: Logger,
}

impl<'a> Connection<'a> {
    pub fn establish(
        stream: &'a mut TcpStream,
        own_sk: &sign::SecretKey,
        other_pk: &sign::PublicKey,
        type_: Type
    ) -> anyhow::Result<Self> {
        let logger = Logger::create()?;
        match type_ {
            Type::Client => {
                let mut stream = Encrypted::request(stream)?;
                stream.verify(other_pk)?;
                stream.authorize(own_sk)?;
                Ok(Self { stream, logger })
            }
            Type::Server => {
                let mut stream = Encrypted::accept(stream)?;
                stream.authorize(own_sk)?;
                stream.verify(other_pk)?;
                Ok(Self { stream, logger })
            }
        }
    }

    pub fn send(&mut self, data: &[u8]) -> anyhow::Result<()> {
        self.stream.send(&data.len().to_le_bytes())?;
        self.logger.info(&format!("Sent message to client: {:?}", String::from_utf8_lossy(&data)));
        self.stream.send(data)
    }

    pub fn receive(&mut self) -> anyhow::Result<Vec<u8>> {
        let len = self.stream.receive(4)?;
        let [a, b, c, d, ..] = len[..] else {
            return Err(Error::msg("Not a valid length received"))
        };
        let length = a as u32 | (b as u32) << 8 | (c as u32) << 16 | (d as u32) << 24;
        let received = self.stream.receive(length as usize);
        if let Ok(val) = &received {
            self.logger.info(&format!("Received message from client: {:?}", String::from_utf8_lossy(val)));
        }

        received
    }
}