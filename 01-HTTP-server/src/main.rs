// Uncomment this block to pass the first stage
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_success_connect(_stream);
                println!("successful connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_success_connect(mut _stream: TcpStream) {
    // read the data here important to preallocate else it reads till infinity
    let mut request_data_vec: Vec<u8> = vec![0; 1000];
    match _stream.read(request_data_vec.as_mut_slice()) {
        Ok(_) => {
            // check the request data vec
            println!(
                "buffer: {}",
                String::from_utf8_lossy(request_data_vec.as_slice())
            );
        }
        Err(_err) => {
            // Nothing to do here
        }
    };

    // send the response here
    let response = format!("HTTP/1.1 200 OK\r\n\r\n");
    _stream.write_all(response.as_bytes()).unwrap();
}
