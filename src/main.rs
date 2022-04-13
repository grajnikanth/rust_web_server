// Building a web server using RUst

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

fn main() {
    // Server will listen at address 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }


}

fn handle_connection(mut stream: TcpStream) {
    // createa buffer to hold 1024 bytes of data on stack
    // mut because the stream from TCP connection might keep changing
    let mut buffer = [0; 1024];

    // read the TCP stream data -(http request) and store it in buffer 
    stream.read(&mut buffer).unwrap();

    // Create bytes type out of the string given
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", 
        status_line,
        contents.len(), 
        contents);

    //as_bytes() converts string to bytes
    // wrtie() function sends the data via the connection
    stream.write(response.as_bytes()).unwrap();
    // flush makes the program wait before continuing until all the bytes are written
    // to the connection
    stream.flush().unwrap();

}
