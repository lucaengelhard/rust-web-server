use dotenv::dotenv;

use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

use rust_web_server::{
    HTTPRequest, HTTPResponse, RequestURL,
    status::{HTTPStatusCode, ServerErrorCode, SuccessCode},
};

fn main() {
    // Setup
    dotenv().ok();

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

    let request_res = HTTPRequest::from_buf_reader(buf_reader);

    if request_res.is_err() {
        let response = HTTPResponse {
            status: request_res.unwrap_err(),
            version: String::from("1.1"),
            contents: None,
        };

        stream.write_all(response.to_string().as_bytes()).unwrap();

        return;
    }

    let request = request_res.unwrap();

    let response = match request.version.as_str() {
        "1.1" => match HTTPRequest::get_file(RequestURL::normalize(request.path.to_str().unwrap()))
        {
            Ok(file_str) => HTTPResponse {
                status: HTTPStatusCode::Success(SuccessCode::OK),
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
            status: HTTPStatusCode::ServerError(ServerErrorCode::HTTPVersionNotSupported),
            version: String::from("1.1"),
            contents: None,
        },
    };

    stream.write_all(response.to_string().as_bytes()).unwrap()
}
