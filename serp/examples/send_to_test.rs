use serp::conn;
use std::io::Write;

fn main() {
    let mut stream = match conn("127.0.0.1:8080", "test") {
        Ok(s) => s,
        Err(e) => { println!("{}", e); return; },
    };

    stream.write(b"hello\n").unwrap();
}
