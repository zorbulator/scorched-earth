use anyhow::bail;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::{env, sync::Arc};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufStream, Interest},
    net::{TcpListener, TcpStream},
};

static MAP: Lazy<Arc<DashMap<String, TcpStream>>> = Lazy::new(|| Arc::new(DashMap::new()));

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let listener = TcpListener::bind(&addr).await?;
    println!("Starting server on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = process(stream).await {
                println!("Error: {}", e);
            }
        });
    }
}

async fn process(mut tcp_stream: TcpStream) -> Result<(), anyhow::Error> {
    let mut stream = BufStream::new(&mut tcp_stream);
    let mut buf = String::new();

    let Ok(_) = stream.read_line(&mut buf).await else { stream.write(b"invalid\n").await?; bail!("no header") };

    let mut fields = buf.split(' ');

    let (Some(method @ ("host" | "conn")), Some(id), None) = (fields.next(), fields.next(), fields.next()) else {
        stream.write(b"invalid\n").await?;
        stream.flush().await?;
        bail!("invalid header line")
    };

    println!("received request: {} {}", method, id);
    stream.flush().await?;

    match method {
        "host" => {
            println!("opening {}", id);

            if MAP.contains_key(id) {
                if MAP
                    .get_mut(id)
                    .unwrap()
                    .ready(Interest::WRITABLE)
                    .await?
                    .is_write_closed()
                {
                    MAP.remove(id);
                } else {
                    stream.write(b"fail\n").await?;
                    stream.flush().await?;
                    bail!("tried to create room that already exists");
                }
            }

            stream.write(b"ok\n").await?;
            stream.flush().await?;
            MAP.insert(id.to_string(), tcp_stream);
        }
        "conn" => {
            // Remove the room if the connection is broken
            if let Some(stream) = MAP.get_mut(id) {
                if stream.ready(Interest::WRITABLE).await?.is_write_closed() {
                    drop(stream);
                    MAP.remove(id);
                }
            }

            let Some(mut other_stream) = MAP.get_mut(&id.to_string()) else {
                stream.write(b"fail\n").await?;
                stream.flush().await?;
                bail!("tried to join nonexistent room");
            };

            stream.write(b"ok\n").await?;
            stream.flush().await?;

            tcp_stream.write(b"connected\n").await?;
            other_stream.write(b"connected\n").await?;
            pipe(&mut tcp_stream, &mut other_stream).await?;
            println!("closing {}", id);
            drop(tcp_stream);
            drop(other_stream);
            MAP.remove(id);
        }
        _ => unreachable!(),
    }

    Ok(())
}

async fn pipe(s1: &mut TcpStream, s2: &mut TcpStream) -> Result<(), anyhow::Error> {
    let mut buf1: Vec<u8> = Vec::new();
    let mut buf2: Vec<u8> = Vec::new();
    loop {
        tokio::select! {
            res = s1.read_buf(&mut buf1) => {
                if let Ok(0) = res { s2.shutdown().await?; return Ok(()) };
                if let Err(e) = res { s2.shutdown().await?; bail!("s1 failed: {}", e)};
                println!("s1 says {:?}", buf1);
                s2.write(&buf1).await?;
                buf1.clear();
            }
            res = s2.read_buf(&mut buf2) => {
                if let Ok(0) = res { s1.shutdown().await?; return Ok(()) };
                if let Err(e) = res { s1.shutdown().await?; bail!("s2 failed: {}", e)};
                println!("s2 says {:?}", buf2);
                s1.write(&buf2).await?;
                buf2.clear();
            }
        }
    }
}
