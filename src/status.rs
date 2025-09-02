use std::fmt;

#[derive(Debug)]
pub enum HTTPStatusCode {
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status
    Informal(InformalCode),
    Success(SuccessCode),
    Redirection(RedirectionCode),
    ClientError(ClientErrorCode),
    ServerError(ServerErrorCode),
}

#[derive(Debug)]
pub enum InformalCode {
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum RedirectionCode {
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum ServerErrorCode {
    InternalServerError,
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
    pub fn to_value(&self) -> u16 {
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
                ClientErrorCode::NotFound => 404,
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
                ServerErrorCode::InternalServerError => 500,
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
                ClientErrorCode::NotFound => write!(f, "Not Found"),
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
                ServerErrorCode::InternalServerError => write!(f, "Internal Server Error"),
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
