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

    // Convert the buffer bytes to string. from_utf..() function takes &[u8]
    // lossy part is going to replaces an invalid sequence with "replacement character"
    // The below will print the request sent by a browser
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", 
        contents.len(), contents);

    //as_bytes() converts string to bytes
    // wrtie() function sends the data via the connection
    stream.write(response.as_bytes()).unwrap();
    // flush makes the program wait before continuing until all the bytes are written
    // to the connection
    stream.flush().unwrap();

}
