use std::{
    io::{Read, Write},
    net::TcpListener,
    net::TcpStream,
};

fn handler_client(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    println!("New connection from : {}", peer_addr);

    let mut buffer = [0; 512];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            stream.write_all(&buffer[0..size]).unwrap();
            true
        }
        Err(_) => false,
    } {}
    println!("connection close from:{}", peer_addr);
}
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("Server listening on port 8000");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let peer_addr = stream.peer_addr().unwrap();
                println!("Accepted connection from: {}", peer_addr);
                std::thread::spawn(move || handler_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}
