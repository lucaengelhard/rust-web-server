use core::str;
use std::{
    collections::HashMap,
    fmt, fs,
    io::{BufRead, BufReader},
    net::TcpStream,
    path::PathBuf,
};

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
    fn from_str(input: &str) -> Result<HTTPMethod, ()> {
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
            &_ => Err(()),
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

pub enum HTTPStatusCode {
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status
    Informal(InformalCode),
    Success(SuccessCode),
    Redirection(RedirectionCode),
    ClientError(ClientErrorCode),
    ServerError(ServerErrorCode),
}

pub enum InformalCode {
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,
}

pub enum SuccessCode {
    OK,
    Created,
    Accepted,
    NonAuthorativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,
}

pub enum RedirectionCode {
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
}

pub enum ClientErrorCode {
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    ContentTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableContent,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,
}

pub enum ServerErrorCode {
    InternalServerError = 500,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
}

impl HTTPStatusCode {
    pub fn to_value(&self) -> u8 {
        match self {
            HTTPStatusCode::Informal(code) => match code {
                InformalCode::Continue => todo!(),
                InformalCode::SwitchingProtocols => todo!(),
                InformalCode::Processing => todo!(),
                InformalCode::EarlyHints => todo!(),
            },
            HTTPStatusCode::Success(code) => match code {
                SuccessCode::OK => 200,
                SuccessCode::Created => todo!(),
                SuccessCode::Accepted => todo!(),
                SuccessCode::NonAuthorativeInformation => todo!(),
                SuccessCode::NoContent => todo!(),
                SuccessCode::ResetContent => todo!(),
                SuccessCode::PartialContent => todo!(),
                SuccessCode::MultiStatus => todo!(),
                SuccessCode::AlreadyReported => todo!(),
                SuccessCode::IMUsed => todo!(),
            },
            HTTPStatusCode::Redirection(code) => match code {
                RedirectionCode::MultipleChoices => todo!(),
                RedirectionCode::MovedPermanently => todo!(),
                RedirectionCode::Found => todo!(),
                RedirectionCode::SeeOther => todo!(),
                RedirectionCode::NotModified => todo!(),
                RedirectionCode::UseProxy => todo!(),
                RedirectionCode::TemporaryRedirect => todo!(),
            },
            HTTPStatusCode::ClientError(code) => match code {
                ClientErrorCode::BadRequest => todo!(),
                ClientErrorCode::Unauthorized => todo!(),
                ClientErrorCode::PaymentRequired => todo!(),
                ClientErrorCode::Forbidden => todo!(),
                ClientErrorCode::NotFound => todo!(),
                ClientErrorCode::MethodNotAllowed => todo!(),
                ClientErrorCode::NotAcceptable => todo!(),
                ClientErrorCode::ProxyAuthenticationRequired => todo!(),
                ClientErrorCode::RequestTimeout => todo!(),
                ClientErrorCode::Conflict => todo!(),
                ClientErrorCode::Gone => todo!(),
                ClientErrorCode::LengthRequired => todo!(),
                ClientErrorCode::PreconditionFailed => todo!(),
                ClientErrorCode::ContentTooLarge => todo!(),
                ClientErrorCode::URITooLong => todo!(),
                ClientErrorCode::UnsupportedMediaType => todo!(),
                ClientErrorCode::RangeNotSatisfiable => todo!(),
                ClientErrorCode::ExpectationFailed => todo!(),
                ClientErrorCode::ImATeapot => todo!(),
                ClientErrorCode::MisdirectedRequest => todo!(),
                ClientErrorCode::UnprocessableContent => todo!(),
                ClientErrorCode::Locked => todo!(),
                ClientErrorCode::FailedDependency => todo!(),
                ClientErrorCode::TooEarly => todo!(),
                ClientErrorCode::UpgradeRequired => todo!(),
                ClientErrorCode::PreconditionRequired => todo!(),
                ClientErrorCode::TooManyRequests => todo!(),
                ClientErrorCode::RequestHeaderFieldsTooLarge => todo!(),
                ClientErrorCode::UnavailableForLegalReasons => todo!(),
            },
            HTTPStatusCode::ServerError(code) => match code {
                ServerErrorCode::InternalServerError => todo!(),
                ServerErrorCode::NotImplemented => todo!(),
                ServerErrorCode::BadGateway => todo!(),
                ServerErrorCode::ServiceUnavailable => todo!(),
                ServerErrorCode::GatewayTimeout => todo!(),
                ServerErrorCode::HTTPVersionNotSupported => todo!(),
                ServerErrorCode::VariantAlsoNegotiates => todo!(),
                ServerErrorCode::InsufficientStorage => todo!(),
                ServerErrorCode::LoopDetected => todo!(),
                ServerErrorCode::NotExtended => todo!(),
                ServerErrorCode::NetworkAuthenticationRequired => todo!(),
            },
        }
    }
}

impl fmt::Display for HTTPStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HTTPStatusCode::Informal(code) => match code {
                InformalCode::Continue => write!(f, "Continue"),
                InformalCode::SwitchingProtocols => todo!(),
                InformalCode::Processing => todo!(),
                InformalCode::EarlyHints => todo!(),
            },
            HTTPStatusCode::Success(code) => match code {
                SuccessCode::OK => write!(f, "Ok"),
                SuccessCode::Created => todo!(),
                SuccessCode::Accepted => todo!(),
                SuccessCode::NonAuthorativeInformation => todo!(),
                SuccessCode::NoContent => todo!(),
                SuccessCode::ResetContent => todo!(),
                SuccessCode::PartialContent => todo!(),
                SuccessCode::MultiStatus => todo!(),
                SuccessCode::AlreadyReported => todo!(),
                SuccessCode::IMUsed => todo!(),
            },
            HTTPStatusCode::Redirection(code) => match code {
                RedirectionCode::MultipleChoices => todo!(),
                RedirectionCode::MovedPermanently => todo!(),
                RedirectionCode::Found => todo!(),
                RedirectionCode::SeeOther => todo!(),
                RedirectionCode::NotModified => todo!(),
                RedirectionCode::UseProxy => todo!(),
                RedirectionCode::TemporaryRedirect => todo!(),
            },
            HTTPStatusCode::ClientError(code) => match code {
                ClientErrorCode::BadRequest => todo!(),
                ClientErrorCode::Unauthorized => todo!(),
                ClientErrorCode::PaymentRequired => todo!(),
                ClientErrorCode::Forbidden => todo!(),
                ClientErrorCode::NotFound => todo!(),
                ClientErrorCode::MethodNotAllowed => todo!(),
                ClientErrorCode::NotAcceptable => todo!(),
                ClientErrorCode::ProxyAuthenticationRequired => todo!(),
                ClientErrorCode::RequestTimeout => todo!(),
                ClientErrorCode::Conflict => todo!(),
                ClientErrorCode::Gone => todo!(),
                ClientErrorCode::LengthRequired => todo!(),
                ClientErrorCode::PreconditionFailed => todo!(),
                ClientErrorCode::ContentTooLarge => todo!(),
                ClientErrorCode::URITooLong => todo!(),
                ClientErrorCode::UnsupportedMediaType => todo!(),
                ClientErrorCode::RangeNotSatisfiable => todo!(),
                ClientErrorCode::ExpectationFailed => todo!(),
                ClientErrorCode::ImATeapot => todo!(),
                ClientErrorCode::MisdirectedRequest => todo!(),
                ClientErrorCode::UnprocessableContent => todo!(),
                ClientErrorCode::Locked => todo!(),
                ClientErrorCode::FailedDependency => todo!(),
                ClientErrorCode::TooEarly => todo!(),
                ClientErrorCode::UpgradeRequired => todo!(),
                ClientErrorCode::PreconditionRequired => todo!(),
                ClientErrorCode::TooManyRequests => todo!(),
                ClientErrorCode::RequestHeaderFieldsTooLarge => todo!(),
                ClientErrorCode::UnavailableForLegalReasons => todo!(),
            },
            HTTPStatusCode::ServerError(code) => match code {
                ServerErrorCode::InternalServerError => todo!(),
                ServerErrorCode::NotImplemented => todo!(),
                ServerErrorCode::BadGateway => todo!(),
                ServerErrorCode::ServiceUnavailable => todo!(),
                ServerErrorCode::GatewayTimeout => todo!(),
                ServerErrorCode::HTTPVersionNotSupported => todo!(),
                ServerErrorCode::VariantAlsoNegotiates => todo!(),
                ServerErrorCode::InsufficientStorage => todo!(),
                ServerErrorCode::LoopDetected => todo!(),
                ServerErrorCode::NotExtended => todo!(),
                ServerErrorCode::NetworkAuthenticationRequired => todo!(),
            },
        }
    }
}

pub struct HTTPRequest {
    pub method: HTTPMethod,
    pub path: PathBuf,
    pub version: String,
    pub headers: HashMap<String, String>,
    // body: Option<String>,
}

impl HTTPRequest {
    pub fn get_file(&self) -> Result<String, HTTPStatusCode> {
        let path = PathBuf::from(match self.path.to_str() {
            Some(str) => match str {
                "/" => "index.html",
                path_str => path_str,
            },
            None => todo!(),
        });

        if !path.exists() {
            return Err(HTTPStatusCode::ClientError(ClientErrorCode::NotFound));
        }

        match fs::read_to_string(path) {
            Ok(str) => Ok(str),
            Err(_) => Err(HTTPStatusCode::ServerError(
                ServerErrorCode::InternalServerError,
            )),
        }

        // todo!("path checking and sanitization");
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
        let status_line = format!("{version} {status_code} {status_message}");

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
