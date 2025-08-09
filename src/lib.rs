use std::collections::HashMap;
use url::Url;

pub struct HTTPRequest {
    method: HTTPMethod,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    // body: Option<String>,
}

impl HTTPRequest {
    fn new(target: &str, method: Option<HTTPMethod>, version: Option<String>) -> HTTPRequest {
        let version = match version {
            Some(v) => v,
            None => String::from("HTTP/1.1"),
        };

        let method = match method {
            Some(m) => m,
            None => HTTPMethod::GET,
        };

        let parsed_url = Url::parse(target).expect("Failed to parse URL");

        let host = parsed_url.host_str().unwrap();
        let path = parsed_url.path();
        // let query = parsed_url.query().unwrap();

        let mut req = HTTPRequest {
            version,
            method,
            path: String::from(path),
            headers: HashMap::new(),
            // body: Some(String::from(query)),
        };

        req.headers.insert(String::from("Host"), String::from(host));

        return req;
    }
}

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

pub enum HTTPStatusCode {
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status
    Informal(InformalCode),
    Success(SuccessCode),
    Redirection(RedirectionCode),
    ClientError(ClientErrorCode),
    ServerError(ServerErrorCode),
}

enum InformalCode {
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,
}

enum SuccessCode {}

enum RedirectionCode {}

enum ClientErrorCode {}

enum ServerErrorCode {}
