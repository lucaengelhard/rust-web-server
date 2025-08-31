use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

use rust_web_server::{HTTPRequest, HTTPResponse, HTTPStatusCode};

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) => listener,
        Err(e) => {
            print!("{}", e);
            return;
        }
    };

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Unable to parse Stream");
                continue;
            }
        };

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let request = match HTTPRequest::from_buf_reader(buf_reader) {
        Ok(r) => r,
        Err(_) => todo!(),
    };

    let response = match request.version.as_str() {
        "1.1" => match request.get_file() {
            Ok(file_str) => HTTPResponse {
                status: HTTPStatusCode::Success(rust_web_server::SuccessCode::OK),
                version: String::from("1.1"),
                contents: Some(file_str),
            },
            Err(code) => HTTPResponse {
                status: code,
                version: String::from("1.1"),
                contents: None,
            },
        },

        &_ => HTTPResponse {
            status: HTTPStatusCode::ServerError(
                rust_web_server::ServerErrorCode::HTTPVersionNotSupported,
            ),
            version: String::from("1.1"),
            contents: None,
        },
    };

    stream.write_all(response.to_string().as_bytes()).unwrap()
}
