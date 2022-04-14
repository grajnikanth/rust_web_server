// Building a web server using RUst

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    // Server will listen at address 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // thread::spawn will open a new thread and execute the closure function
        // which in this case is the code block inside the culy brackets
        // with the below code, we now have each request from client sent
        // to a different thread. So the /sleep won't block the other home
        // links
        thread::spawn(|| {
            handle_connection(stream);
        });
        
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
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // With sleep code here we are simulating the limitations of a single threaded
    // web server where every requests waits for the first to finish before proceeding
    // to the next. 

    // We can use multi threads to avoid server waiting for a long process to finish
    // before the next request is processed

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    }  // The below else if simulates a pause in the program execution delaying 
    // execution of rest of the code
    else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
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
