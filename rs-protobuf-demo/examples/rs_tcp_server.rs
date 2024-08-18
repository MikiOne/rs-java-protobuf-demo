use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let addr = stream.peer_addr().unwrap();
    println!("Client connected: {}", addr);

    // 创建一个缓冲读取器
    let reader = BufReader::new(stream.try_clone().unwrap());

    for line in reader.lines() {
        match line {
            Ok(message) => {
                println!("Received from {}: {}", addr, message);
                // 处理消息并发送响应
                let response = format!("Echo: {}", message);
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                eprintln!("Failed to read from client {}: {}", addr, e);
                break;
            }
        }
    }

    println!("Client disconnected: {}", addr);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9527").expect("Could not bind to address");
    println!("Server is listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // 为每个连接创建一个新线程
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}