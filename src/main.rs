use std::{
    collections::HashMap,
    fmt, fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    path::Path,
};

#[derive(Debug)]
struct HTTPRequest {
    method: HTTPMethod,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    // content: todo!(),
}

impl HTTPRequest {
    fn new(method: HTTPMethod, path: String, version: String) -> HTTPRequest {
        HTTPRequest {
            method,
            path,
            version,
            headers: HashMap::new(),
            // content: unimplemented!(),
        }
    }

    fn get_file(&self) -> Result<Result<String, std::io::Error>, ()> {
        let path = Path::new(match self.path.as_str() {
            "/" => "index.html",
            path => path,
        });

        if !path.exists() {
            return Err(());
        }

        Ok(fs::read_to_string(path))
    }

    fn check_http_version(input: &str) -> Result<&str, ()> {
        match input {
            "HTTP/1.1" => Ok(input),
            &_ => Err(()),
        }
    }

    fn create_response(code: HTTPStatusCode, contents: &str) -> String {
        let code_value = code.to_value();
        let status_line = format!("HTTP/1.1 {code_value} {code}");

        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        response
    }

    fn parse_buf_reader(buf_reader: BufReader<&TcpStream>) -> Result<HTTPRequest, ()> {
        let lines = buf_reader.lines();

        let mut request: Option<HTTPRequest> = None;

        for iteration in lines.enumerate() {
            let (i, line) = iteration;
            let line_str = line.unwrap();
            if line_str.is_empty() {
                break;
            }

            if i == 0 {
                let split: Vec<&str> = line_str.split_whitespace().collect();

                if split.len() != 3 {
                    return Err(());
                }

                let method = HTTPMethod::parse_http_method(split[0]).unwrap();

                let path = split[1];

                let version = HTTPRequest::check_http_version(split[2]).unwrap();

                request = Some(HTTPRequest::new(
                    method,
                    String::from(path),
                    String::from(version),
                ));
            } else {
                let split = line_str.split_once(":").unwrap();

                let key = split.0;
                let value = split.1.trim();

                match request {
                    Some(ref mut r) => r.headers.insert(String::from(key), String::from(value)),
                    None => return Err(()),
                };
            }
        }

        match request {
            Some(r) => Ok(r),
            None => Err(()),
        }
    }
}

#[derive(Debug)]
enum HTTPMethod {
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl HTTPMethod {
    fn parse_http_method(input: &str) -> Result<HTTPMethod, HTTPMethodParseError> {
        match input {
            "GET" => Ok(HTTPMethod::GET),
            "HEAD" => Ok(HTTPMethod::HEAD),
            "POST" => Ok(HTTPMethod::POST),
            "PUT" => Ok(HTTPMethod::PUT),
            "DELETE" => Ok(HTTPMethod::DELETE),
            "CONNECT" => Ok(HTTPMethod::CONNECT),
            "OPTIONS" => Ok(HTTPMethod::OPTIONS),
            "TRACE" => Ok(HTTPMethod::TRACE),
            "PATCH" => Ok(HTTPMethod::PATCH),
            &_ => Err(HTTPMethodParseError {
                input: String::from(input),
            }),
        }
    }
}

#[derive(Debug)]
struct HTTPMethodParseError {
    input: String,
}

enum HTTPStatusCode {
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status
    Ok,
    NotFound,
    InternalServerError,
}

impl HTTPStatusCode {
    fn to_value(&self) -> u32 {
        match self {
            HTTPStatusCode::Ok => 200,
            HTTPStatusCode::NotFound => 404,
            HTTPStatusCode::InternalServerError => 500,
        }
    }
}

impl fmt::Display for HTTPStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HTTPStatusCode::Ok => write!(f, "Ok"),
            HTTPStatusCode::NotFound => write!(f, "Not Found"),
            HTTPStatusCode::InternalServerError => write!(f, "Internal Server Error"),
        }
    }
}

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(listener) => listener,
        Err(_e) => panic!(),
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
    let request = HTTPRequest::parse_buf_reader(buf_reader);

    let response = match request {
        Ok(r) => {
            let contents = match &r.get_file() {
                Ok(file) => match file {
                    Ok(string) => string,
                    Err(e) => {
                        eprintln!("{}", e);
                        return;
                    }
                },
                Err(_) => return,
            };

            HTTPRequest::create_response(HTTPStatusCode::Ok, contents)
        }

        Err(_) => todo!(),
    };

    stream.write_all(response.as_bytes()).unwrap()
}
