use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

fn handler_client(stream: TcpStream, client: Arc<Mutex<Vec<TcpStream>>>) {
    let peer_addr = stream.peer_addr().unwrap();
    let mut line = String::new();
    let mut reader = BufReader::new(&stream);
    println!("New connection from : {}", peer_addr);

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).unwrap();

        if bytes_read == 0 {
            println!("Client disconnected: {}", peer_addr);
            break;
        }
        let message = format!("{}: {}", peer_addr, line);
        let mut clients_guard = client.lock().unwrap();
        for client in clients_guard.iter_mut() {
            let _ = client.write_all(message.as_bytes());
        }
    }
    let mut client_guard = client.lock().unwrap();
    client_guard.retain(|c| c.peer_addr().unwrap() != peer_addr);
}
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let clients = Arc::new(Mutex::new(Vec::new()));
    println!("Server listening on port 8000");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let client_clone = Arc::clone(&clients);
                clients.lock().unwrap().push(stream.try_clone().unwrap());
                std::thread::spawn(move || handler_client(stream, client_clone));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}
