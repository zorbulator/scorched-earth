use std::{
    io::{self, Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

use rmp_serde::{to_vec, from_slice};
use scorched_earth_core::{Board, Move};
use serde::{Serialize, Deserialize};
use serp::SerpError;
use snow::{Builder, TransportState};
use thiserror::Error;

static PARAMS: &'static str = "Noise_XXpsk3_25519_ChaChaPoly_BLAKE2s";

pub struct Connection {
    noise: TransportState,
    stream: TcpStream,
    buf: Vec<u8>,
    pub player_num: usize,
}

#[derive(Serialize, Deserialize)]
pub struct MoveMessage {
    pub new_board: Board,
    pub new_move: Move,
    pub player: usize,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("serp error: {0}")]
    SerpError(#[from] SerpError),
    #[error("encode error: {0}")]
    EncodeError(#[from] rmp_serde::encode::Error),
    #[error("decode error: {0}")]
    DecodeError(#[from] rmp_serde::decode::Error),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Noise protocol error: {0}")]
    NoiseError(#[from] snow::Error),
    #[error("Error hashing room id: {0}")]
    HashError(#[from] argon2::Error),
    #[error("Opponent disconnected ({0})")]
    DisconnectError(io::Error),
}

impl Connection {
    pub fn send_move(&mut self, msg: MoveMessage) -> Result<(), Error> {
        let msg_buf = to_vec(&msg)?;
        self.send(&msg_buf)?;
        Ok(())
    } 

    pub fn recv_move(&mut self) -> Result<MoveMessage, Error> {
        Ok(from_slice(self.recv()?)?)
    }

    pub fn host<A: ToSocketAddrs>(addr: A, secret: &[u8], board: &Board) -> Result<Self, Error> {
        let mut buf = vec![0u8; 65535];

        let builder = Builder::new(PARAMS.parse()?);
        let static_key = builder.generate_keypair()?.private;
        let mut noise = builder
            .local_private_key(&static_key)
            .psk(3, secret)
            .build_responder()?;

        // Kind of misusing argon2 to send the secret to the server without revealing it
        // Can't really salt it since it needs to match what the server generates
        let salt = b"scorchedearth";
        let config = argon2::Config::default();
        let hash = argon2::hash_encoded(secret, salt, &config)?;

        let mut stream = serp::host(addr, &hash)?;

        // <- e
        noise.read_message(&tcp_recv(&mut stream)?, &mut buf)?;

        // -> e, ee, s, es
        let len = noise.write_message(&[0u8; 0], &mut buf)?;
        tcp_send(&mut stream, &buf[..len])?;

        // <- s, se
        noise.read_message(&tcp_recv(&mut stream)?, &mut buf)?;

        // handshake complete
        let noise = noise.into_transport_mode()?;

        let mut conn = Self { noise, stream, buf, player_num: 0 };

        let board_buf = to_vec(board)?;

        conn.send(&board_buf)?;

        Ok(conn)
    }

    pub fn conn<A: ToSocketAddrs>(addr: A, secret: &[u8]) -> Result<(Self, Board), Error> {
        let mut buf = vec![0u8; 65535];

        let builder = Builder::new(PARAMS.parse()?);
        let static_key = builder.generate_keypair()?.private;
        let mut noise = builder
            .local_private_key(&static_key)
            .psk(3, secret)
            .build_initiator()?;

        let salt = b"scorchedearth";
        let config = argon2::Config::default();
        let hash = argon2::hash_encoded(secret, salt, &config)?;

        let mut stream = serp::conn(addr, &hash)?;

        // -> e
        let len = noise.write_message(&[], &mut buf)?;
        tcp_send(&mut stream, &buf[..len])?;

        // <- e, ee, s, es
        noise.read_message(&tcp_recv(&mut stream)?, &mut buf)?;

        // -> s, se
        let len = noise.write_message(&[], &mut buf)?;
        tcp_send(&mut stream, &buf[..len])?;

        let noise = noise.into_transport_mode()?;

        let mut conn = Self { noise, stream, buf, player_num: 1 };

        let board_buf = conn.recv()?;
        let board: Board = from_slice(board_buf)?;

        Ok((conn, board))
    }

    pub fn recv(&mut self) -> Result<&[u8], Error> {
        let msg = tcp_recv(&mut self.stream).map_err(Error::DisconnectError)?;
        let len = self.noise.read_message(&msg, &mut self.buf)?;
        Ok(&self.buf[..len])
    }

    pub fn send(&mut self, msg: &[u8]) -> Result<(), Error> {
        let len = self.noise.write_message(msg, &mut self.buf)?;
        tcp_send(&mut self.stream, &self.buf[..len]).map_err(Error::DisconnectError)?;
        Ok(())
    }
}

/// Receive some data preceded by 16-bit BE length
fn tcp_recv(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
    let mut msg_len_buf = [0u8; 2];
    stream.read_exact(&mut msg_len_buf)?;
    let msg_len = ((msg_len_buf[0] as usize) << 8) + (msg_len_buf[1] as usize);
    let mut msg = vec![0u8; msg_len];
    stream.read_exact(&mut msg[..])?;
    Ok(msg)
}

/// Send some data preceded by 16-bit BE length
fn tcp_send(stream: &mut TcpStream, buf: &[u8]) -> io::Result<()> {
    let msg_len_buf = [(buf.len() >> 8) as u8, (buf.len() & 0xff) as u8];
    stream.write_all(&msg_len_buf)?;
    stream.write_all(buf)?;
    Ok(())
}
