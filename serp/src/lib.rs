use std::{net::{ToSocketAddrs, TcpStream}, io::{Write, BufReader, BufRead}, time::Duration};

#[derive(thiserror::Error, Debug)]
pub enum SerpError {
    #[error("Connection error: {0}")]
    ConnectionError(#[from] std::io::Error),
    #[error("No ok from server")]
    NoOk,
    #[error("Invalid request")]
    InvalidRequest,
    #[error("Room already exists")]
    RoomExists,
    #[error("Room doesn't exist")]
    RoomDoesntExist,
    #[error("Connection broke somehow")]
    ConnectionBroke,
}

pub fn host<A: ToSocketAddrs>(addr: A, id: &str) -> Result<TcpStream, SerpError> {
    let mut stream = TcpStream::connect(addr)?;
    stream.write(b"host ")?;
    stream.write(id.as_bytes())?;
    stream.write(b"\n")?;

    // Server should respond with "ok" soon
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;

    let mut reader = BufReader::new(stream);
    let mut buf = String::new();

    reader.read_line(&mut buf).map_err(|_| SerpError::NoOk)?;
 
    match buf.as_str() {
        "ok\n" => {},
        "invalid\n" => { return Err(SerpError::InvalidRequest); },
        "fail\n" => { return Err(SerpError::RoomExists); },
        _ => { return Err(SerpError::NoOk); },
    }

    // The connection could take a lot longer, so disable timeout
    reader.get_mut().set_read_timeout(None)?;

    buf.clear();
    reader.read_line(&mut buf)?;

    match buf.as_str() {
        "connected\n" => Ok(reader.into_inner()),
        _ => Err(SerpError::ConnectionBroke),
    }
}

pub fn conn<A: ToSocketAddrs>(addr: A, id: &str) -> Result<TcpStream, SerpError> {
    tracing::info!("connecting...");
    let mut stream = TcpStream::connect_timeout(&addr.to_socket_addrs()?.next().unwrap(), Duration::from_secs(15))?;
    tracing::info!("connected");
    stream.write(b"conn ")?;
    stream.write(id.as_bytes())?;
    stream.write(b"\n")?;

    // Server should respond with "ok" soon
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;

    let mut reader = BufReader::new(stream);
    let mut buf = String::new();

    reader.read_line(&mut buf).map_err(|_| SerpError::NoOk)?;

    match buf.as_str() {
        "ok\n" => {},
        "invalid\n" => { return Err(SerpError::InvalidRequest); },
        "fail\n" => { return Err(SerpError::RoomDoesntExist); },
        _ => { return Err(SerpError::NoOk); },
    }

    buf.clear();
    reader.read_line(&mut buf)?;

    // Connect should be fast so don't unset timeout until afterwards
    reader.get_mut().set_read_timeout(None)?;

    match buf.as_str() {
        "connected\n" => Ok(reader.into_inner()),
        _ => Err(SerpError::ConnectionBroke),
    }
}
