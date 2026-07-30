use std::{eprintln, net::TcpStream};
use std::io::{Read, Write};
use std::fs;
use httparse::{Request, EMPTY_HEADER}; 
use std::path::Path;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 4096];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected.");
                break;
            }

            Ok(bytes_read) => {
                let mut header_buffer = [EMPTY_HEADER; 16];
                let mut request = Request::new(&mut header_buffer);

                match request.parse(&buffer[..bytes_read]) {
                    Ok(httparse::Status::Complete(_body_offset)) => {
                        if request.method != Some("GET") {
                            let response = "HTTP/1.1 405 Method Not Allowed\r\n\r\n405 Method Not Allowed";
                            let _ = stream.write_all(response.as_bytes());
                            break;
                        }

                        let path = match request.path.unwrap_or("/") {
                            "/" => "index.html",
                            path => &path[1..]
                        };


                        let file_extension = Path::new(path).extension().and_then(|s| s.to_str()).unwrap_or("");

                        let _ = match fs::read_to_string(path) {
                            Ok(body) => {
                                let mut response = String::new();
                                if file_extension == "html" {
                                    response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}", body.len(), body);
                                }
                                if file_extension == "css" {
                                    response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/css; charset=UTF-8\r\n\r\n{}", body.len(), body);
                                }
                                if file_extension == "js" {
                                    response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/javascript; charset=UTF-8\r\n\r\n{}", body.len(), body);
                                }
                                if file_extension == "txt" {
                                    response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain; charset=UTF-8\r\n\r\n{}", body.len(), body);
                                }

                                let _ = stream.write_all(response.as_bytes());
                                break;
                            },
                            Err(_e) => {
                                let response = "HTTP/1.1 404 Not Found\r\n\r\n404 Not Found";
                                let _ = stream.write_all(response.as_bytes());
                                break;
                            }
                        };
                    }
                    Ok(httparse::Status::Partial) => {
                        continue;
                    }
                    Err(err) => {
                        eprintln!("Error in parsing request: {}", err);
                        break;
                    }
                }
            }

            Err(e) => {
                eprintln!("Error in reading stream data: {}", e);
                break;
            }
        }
    }
}
