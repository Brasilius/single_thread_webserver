use std::net::TcpListener;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:7777").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("conncection established!");
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let https_request: Vec<_> = buf_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();
    println!("{:?}", https_request);




}