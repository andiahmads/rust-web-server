use std::{collections::HashMap, io, usize};

use tokio::net::TcpListener;
mod http;
use http::*;

async fn handler(socket: tokio::net::TcpStream) -> Result<(), Error> {
    let connection = Connection::new(socket).await?;
    println!(
        "method: {:?}\nuri: {:?}\nversion: {:?}\nheaders:{:?}\n",
        connection.request.method,
        connection.request.uri,
        connection.request.version,
        connection.request.headers,
    );
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8383").await?;
    println!("server running on port 8383");

    loop {
        let (socket, _) = listener.accept().await?;
        handler(socket).await;
    }
}
