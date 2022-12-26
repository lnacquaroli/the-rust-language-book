/*

Final project: Building a Single-Threaded Web Server

- The two main protocols involved in web servers are Hypertext Transfer Protocol
(HTTP) and Transmission Control Protocol (TCP).
- Both protocols are request-response protocols, meaning a client initiates requests and a server listens to the requests and provides a response to the client. The contents of those requests and responses are defined by the protocols.

- TCP is the lower-level protocol that describes the details of how information gets
from one server to another but doesn’t specify what that information is.
- HTTP builds on top of TCP by defining the contents of the requests and responses.
It’s technically possible to use HTTP with other protocols, but in the vast majority
of cases, HTTP sends its data over TCP. We’ll work with the raw bytes of TCP and
HTTP requests and responses.

*/

// Get access to traits and types
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // Listening to the TCP Connection
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

    // Reading the request
}

/// Read data from the TCP stream and print it so we can see the data being sent
/// from the browser.
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
