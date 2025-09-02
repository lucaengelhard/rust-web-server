pub mod status;

use std::{
    collections::HashMap,
    env, fmt, fs,
    io::{BufRead, BufReader},
    net::TcpStream,
    path::PathBuf,
};

use crate::status::{ClientErrorCode, HTTPStatusCode, ServerErrorCode};

#[derive(Debug)]
pub enum HTTPMethod {
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
    fn from_str(input: &str) -> Result<HTTPMethod, &str> {
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
            s => Err(s),
        }
    }
}

impl fmt::Display for HTTPMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HTTPMethod::GET => write!(f, "GET"),
            HTTPMethod::HEAD => write!(f, "HEAD"),
            HTTPMethod::POST => write!(f, "POST"),
            HTTPMethod::PUT => write!(f, "PUT"),
            HTTPMethod::DELETE => write!(f, "DELETE"),
            HTTPMethod::CONNECT => write!(f, "CONNECT"),
            HTTPMethod::OPTIONS => write!(f, "OPTIONS"),
            HTTPMethod::TRACE => write!(f, "TRACE"),
            HTTPMethod::PATCH => write!(f, "PATCH"),
        }
    }
}

#[derive(Debug)]
pub struct HTTPRequest {
    pub method: HTTPMethod,
    pub path: PathBuf,
    pub version: String,
    pub headers: HashMap<String, String>,
    // body: Option<String>,
}

const DEFAULT_ROOT_FOLDER: &str = "public";

impl HTTPRequest {
    pub fn get_file(&self) -> Result<String, HTTPStatusCode> {
        let mut path = PathBuf::from(match self.path.to_str() {
            Some(str) => match str {
                "/" => match HTTPRequest::get_index_file_name() {
                    Ok(p) => p,
                    Err(_) => String::from(""),
                },
                path_str => String::from(path_str),
            },
            None => todo!(),
        });

        // todo! more prefix stripping
        if path.starts_with("/") {
            path = path.strip_prefix("/").unwrap().to_path_buf();
        }

        let mut root_path = HTTPRequest::get_root_dir();

        root_path.push(path);

        println!("{}", root_path.display());

        if !root_path.exists() || !root_path.is_file() {
            return Err(HTTPStatusCode::ClientError(ClientErrorCode::NotFound));
        }

        match fs::read_to_string(root_path) {
            Ok(str) => Ok(str),
            Err(_) => Err(HTTPStatusCode::ServerError(
                ServerErrorCode::InternalServerError,
            )),
        }

        // todo!("path checking and sanitization");
        // todo! root folder
    }

    fn get_index_file_name() -> Result<String, HTTPStatusCode> {
        let extensions = [".html"];
        let root_path = HTTPRequest::get_root_dir();

        let mut res: Option<String> = None;

        for ext in extensions {
            let test_path = PathBuf::from(format!("{}{}", "index", ext));
            let mut test_root = root_path.clone();
            test_root.push(test_path.clone());

            if test_root.exists() {
                res = Some(String::from(test_path.to_str().unwrap()));
                break;
            }
        }

        match res {
            Some(r) => Ok(r),
            None => Err(HTTPStatusCode::ClientError(ClientErrorCode::NotFound)),
        }
    }

    fn get_root_dir() -> PathBuf {
        PathBuf::from(match env::var("ROOT") {
            Ok(value) => value,
            Err(_) => String::from(DEFAULT_ROOT_FOLDER),
        })
    }

    pub fn from_buf_reader(buf_reader: BufReader<&TcpStream>) -> Result<HTTPRequest, ()> {
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

                let method = match HTTPMethod::from_str(split[0]) {
                    Ok(m) => m,
                    Err(_) => todo!(),
                };

                let path = PathBuf::from(split[1]);

                let version = match get_http_version_from_string(split[2]) {
                    Ok(v) => v,
                    Err(_) => todo!(),
                };

                request = Some(HTTPRequest {
                    method,
                    path,
                    version,
                    headers: HashMap::new(),
                })
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
pub struct HTTPResponse {
    pub status: HTTPStatusCode,
    pub version: String,
    pub contents: Option<String>,
}

impl HTTPResponse {
    pub fn to_string(&self) -> String {
        let version = &self.version;
        let status_code = self.status.to_value();
        let status_message = &self.status;
        let status_line = format!("HTTP/{version} {status_code} {status_message}");

        let contents = match &self.contents {
            Some(c) => String::from(c),
            None => String::from(""),
        };
        let length = contents.len();

        let res_string = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        res_string
    }
}

fn get_http_version_from_string(input: &str) -> Result<String, ()> {
    let prefix = "HTTP/";

    if !input.starts_with(prefix) {
        return Err(());
    }

    Ok(input.replace(prefix, ""))
}
