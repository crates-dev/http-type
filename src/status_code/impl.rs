use crate::*;

/// The `StatusCode` enum represents the HTTP status codes.
///
/// It maps common HTTP status codes to their respective meanings. It provides methods to retrieve
/// the corresponding numeric code as well as the associated status text. Additionally, it implements
/// conversion from a string representation of the status code.
impl StatusCode {
    /// Returns the numeric HTTP status code associated with this status code variant.
    ///
    /// This method returns the corresponding HTTP numeric status code based on the `StatusCode` variant.
    /// For example:
    /// - `Self::Ok` returns 200.
    /// - `Self::BadRequest` returns 400.
    /// - `Self::Unknown` returns 0 (the default for unrecognized status codes).
    ///
    /// # Parameters
    /// - `&self`: A reference to the `StatusCode` enum instance. This represents the specific variant of the `StatusCode` enum that the method is called on.
    ///
    /// # Return Value
    /// - `StatusCodeUsize`: The numeric HTTP status code associated with the `StatusCode` variant. For example:
    #[inline]
    pub fn code(&self) -> StatusCodeUsize {
        match self {
            Self::Continue => 100,
            Self::SwitchingProtocols => 101,
            Self::Processing => 102,
            Self::EarlyHints => 103,
            Self::Ok => 200,
            Self::Created => 201,
            Self::Accepted => 202,
            Self::NonAuthoritativeInformation => 203,
            Self::NoContent => 204,
            Self::ResetContent => 205,
            Self::PartialContent => 206,
            Self::MultiStatus => 207,
            Self::AlreadyReported => 208,
            Self::IMUsed => 226,
            Self::MultipleChoices => 300,
            Self::MovedPermanently => 301,
            Self::Found => 302,
            Self::SeeOther => 303,
            Self::NotModified => 304,
            Self::UseProxy => 305,
            Self::TemporaryRedirect => 307,
            Self::PermanentRedirect => 308,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::PaymentRequired => 402,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            Self::NotAcceptable => 406,
            Self::ProxyAuthenticationRequired => 407,
            Self::RequestTimeout => 408,
            Self::Conflict => 409,
            Self::Gone => 410,
            Self::LengthRequired => 411,
            Self::PreconditionFailed => 412,
            Self::PayloadTooLarge => 413,
            Self::URITooLong => 414,
            Self::UnsupportedMediaType => 415,
            Self::RangeNotSatisfiable => 416,
            Self::ExpectationFailed => 417,
            Self::ImATeapot => 418,
            Self::MisdirectedRequest => 421,
            Self::UnprocessableEntity => 422,
            Self::Locked => 423,
            Self::FailedDependency => 424,
            Self::TooEarly => 425,
            Self::UpgradeRequired => 426,
            Self::PreconditionRequired => 428,
            Self::TooManyRequests => 429,
            Self::RequestHeaderFieldsTooLarge => 431,
            Self::UnavailableForLegalReasons => 451,
            Self::InternalServerError => 500,
            Self::NotImplemented => 501,
            Self::BadGateway => 502,
            Self::ServiceUnavailable => 503,
            Self::GatewayTimeout => 504,
            Self::HTTPVersionNotSupported => 505,
            Self::VariantAlsoNegotiates => 506,
            Self::InsufficientStorage => 507,
            Self::LoopDetected => 508,
            Self::NotExtended => 510,
            Self::NetworkAuthenticationRequired => 511,
            Self::Unknown => 0,
        }
    }

    /// Converts an HTTP status code to its corresponding textual description.
    ///
    /// This method matches a given numeric HTTP status code and returns the corresponding
    /// textual representation defined in the `StatusCode` enum.
    ///
    /// # Parameters
    /// - `code`: A `StatusCodeUsize` representing the HTTP status code to convert.
    ///
    /// # Return Value
    /// - `String`: A string representing the textual description of the HTTP status code.
    #[inline]
    pub fn phrase(code: StatusCodeUsize) -> String {
        match code {
            100 => Self::Continue.to_string(),
            101 => Self::SwitchingProtocols.to_string(),
            102 => Self::Processing.to_string(),
            103 => Self::EarlyHints.to_string(),
            200 => Self::Ok.to_string(),
            201 => Self::Created.to_string(),
            202 => Self::Accepted.to_string(),
            203 => Self::NonAuthoritativeInformation.to_string(),
            204 => Self::NoContent.to_string(),
            205 => Self::ResetContent.to_string(),
            206 => Self::PartialContent.to_string(),
            207 => Self::MultiStatus.to_string(),
            208 => Self::AlreadyReported.to_string(),
            226 => Self::IMUsed.to_string(),
            300 => Self::MultipleChoices.to_string(),
            301 => Self::MovedPermanently.to_string(),
            302 => Self::Found.to_string(),
            303 => Self::SeeOther.to_string(),
            304 => Self::NotModified.to_string(),
            305 => Self::UseProxy.to_string(),
            307 => Self::TemporaryRedirect.to_string(),
            308 => Self::PermanentRedirect.to_string(),
            400 => Self::BadRequest.to_string(),
            401 => Self::Unauthorized.to_string(),
            402 => Self::PaymentRequired.to_string(),
            403 => Self::Forbidden.to_string(),
            404 => Self::NotFound.to_string(),
            405 => Self::MethodNotAllowed.to_string(),
            406 => Self::NotAcceptable.to_string(),
            407 => Self::ProxyAuthenticationRequired.to_string(),
            408 => Self::RequestTimeout.to_string(),
            409 => Self::Conflict.to_string(),
            410 => Self::Gone.to_string(),
            411 => Self::LengthRequired.to_string(),
            412 => Self::PreconditionFailed.to_string(),
            413 => Self::PayloadTooLarge.to_string(),
            414 => Self::URITooLong.to_string(),
            415 => Self::UnsupportedMediaType.to_string(),
            416 => Self::RangeNotSatisfiable.to_string(),
            417 => Self::ExpectationFailed.to_string(),
            418 => Self::ImATeapot.to_string(),
            421 => Self::MisdirectedRequest.to_string(),
            422 => Self::UnprocessableEntity.to_string(),
            423 => Self::Locked.to_string(),
            424 => Self::FailedDependency.to_string(),
            425 => Self::TooEarly.to_string(),
            426 => Self::UpgradeRequired.to_string(),
            428 => Self::PreconditionRequired.to_string(),
            429 => Self::TooManyRequests.to_string(),
            431 => Self::RequestHeaderFieldsTooLarge.to_string(),
            451 => Self::UnavailableForLegalReasons.to_string(),
            500 => Self::InternalServerError.to_string(),
            501 => Self::NotImplemented.to_string(),
            502 => Self::BadGateway.to_string(),
            503 => Self::ServiceUnavailable.to_string(),
            504 => Self::GatewayTimeout.to_string(),
            505 => Self::HTTPVersionNotSupported.to_string(),
            506 => Self::VariantAlsoNegotiates.to_string(),
            507 => Self::InsufficientStorage.to_string(),
            508 => Self::LoopDetected.to_string(),
            510 => Self::NotExtended.to_string(),
            511 => Self::NetworkAuthenticationRequired.to_string(),
            _ => Self::Unknown.to_string(),
        }
    }

    #[inline]
    pub fn same(&self, code_str: &str) -> bool {
        self.code().to_string().eq_ignore_ascii_case(code_str)
            || self.to_string().eq_ignore_ascii_case(code_str)
    }
}

impl Display for StatusCode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let res: &str = match self {
            Self::Continue => "Continue",
            Self::SwitchingProtocols => "Switching Protocols",
            Self::Processing => "Processing",
            Self::EarlyHints => "Early Hints",
            Self::Ok => "OK",
            Self::Created => "Created",
            Self::Accepted => "Accepted",
            Self::NonAuthoritativeInformation => "Non-Authoritative Information",
            Self::NoContent => "No Content",
            Self::ResetContent => "Reset Content",
            Self::PartialContent => "Partial Content",
            Self::MultiStatus => "Multi-Status",
            Self::AlreadyReported => "Already Reported",
            Self::IMUsed => "IM Used",
            Self::MultipleChoices => "Multiple Choices",
            Self::MovedPermanently => "Moved Permanently",
            Self::Found => "Found",
            Self::SeeOther => "See Other",
            Self::NotModified => "Not Modified",
            Self::UseProxy => "Use Proxy",
            Self::TemporaryRedirect => "Temporary Redirect",
            Self::PermanentRedirect => "Permanent Redirect",
            Self::BadRequest => "Bad Request",
            Self::Unauthorized => "Unauthorized",
            Self::PaymentRequired => "Payment Required",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "Not Found",
            Self::MethodNotAllowed => "Method Not Allowed",
            Self::NotAcceptable => "Not Acceptable",
            Self::ProxyAuthenticationRequired => "Proxy Authentication Required",
            Self::RequestTimeout => "Request Timeout",
            Self::Conflict => "Conflict",
            Self::Gone => "Gone",
            Self::LengthRequired => "Length Required",
            Self::PreconditionFailed => "Precondition Failed",
            Self::PayloadTooLarge => "Payload Too Large",
            Self::URITooLong => "URI Too Long",
            Self::UnsupportedMediaType => "Unsupported Media Type",
            Self::RangeNotSatisfiable => "Range Not Satisfiable",
            Self::ExpectationFailed => "Expectation Failed",
            Self::ImATeapot => "I'm a teapot",
            Self::MisdirectedRequest => "Misdirected Request",
            Self::UnprocessableEntity => "Unprocessable Entity",
            Self::Locked => "Locked",
            Self::FailedDependency => "Failed Dependency",
            Self::TooEarly => "Too Early",
            Self::UpgradeRequired => "Upgrade Required",
            Self::PreconditionRequired => "Precondition Required",
            Self::TooManyRequests => "Too Many Requests",
            Self::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            Self::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            Self::InternalServerError => "Internal Server Error",
            Self::NotImplemented => "Not Implemented",
            Self::BadGateway => "Bad Gateway",
            Self::ServiceUnavailable => "Service Unavailable",
            Self::GatewayTimeout => "Gateway Timeout",
            Self::HTTPVersionNotSupported => "HTTP Version Not Supported",
            Self::VariantAlsoNegotiates => "Variant Also Negotiates",
            Self::InsufficientStorage => "Insufficient Storage",
            Self::LoopDetected => "Loop Detected",
            Self::NotExtended => "Not Extended",
            Self::NetworkAuthenticationRequired => "Network Authentication Required",
            Self::Unknown => "Unknown",
        };
        write!(f, "{}", res)
    }
}

impl FromStr for StatusCode {
    type Err = ();

    #[inline]
    fn from_str(code_str: &str) -> Result<Self, Self::Err> {
        match code_str {
            _code_str if Self::Continue.same(_code_str) => Ok(Self::Continue),
            _code_str if Self::SwitchingProtocols.same(_code_str) => Ok(Self::SwitchingProtocols),
            _code_str if Self::Processing.same(_code_str) => Ok(Self::Processing),
            _code_str if Self::EarlyHints.same(_code_str) => Ok(Self::EarlyHints),
            _code_str if Self::Ok.same(_code_str) => Ok(Self::Ok),
            _code_str if Self::Created.same(_code_str) => Ok(Self::Created),
            _code_str if Self::Accepted.same(_code_str) => Ok(Self::Accepted),
            _code_str if Self::NonAuthoritativeInformation.same(_code_str) => {
                Ok(Self::NonAuthoritativeInformation)
            }
            _code_str if Self::NoContent.same(_code_str) => Ok(Self::NoContent),
            _code_str if Self::ResetContent.same(_code_str) => Ok(Self::ResetContent),
            _code_str if Self::PartialContent.same(_code_str) => Ok(Self::PartialContent),
            _code_str if Self::MultiStatus.same(_code_str) => Ok(Self::MultiStatus),
            _code_str if Self::AlreadyReported.same(_code_str) => Ok(Self::AlreadyReported),
            _code_str if Self::IMUsed.same(_code_str) => Ok(Self::IMUsed),
            _code_str if Self::MultipleChoices.same(_code_str) => Ok(Self::MultipleChoices),
            _code_str if Self::MovedPermanently.same(_code_str) => Ok(Self::MovedPermanently),
            _code_str if Self::Found.same(_code_str) => Ok(Self::Found),
            _code_str if Self::SeeOther.same(_code_str) => Ok(Self::SeeOther),
            _code_str if Self::NotModified.same(_code_str) => Ok(Self::NotModified),
            _code_str if Self::UseProxy.same(_code_str) => Ok(Self::UseProxy),
            _code_str if Self::TemporaryRedirect.same(_code_str) => Ok(Self::TemporaryRedirect),
            _code_str if Self::PermanentRedirect.same(_code_str) => Ok(Self::PermanentRedirect),
            _code_str if Self::BadRequest.same(_code_str) => Ok(Self::BadRequest),
            _code_str if Self::Unauthorized.same(_code_str) => Ok(Self::Unauthorized),
            _code_str if Self::PaymentRequired.same(_code_str) => Ok(Self::PaymentRequired),
            _code_str if Self::Forbidden.same(_code_str) => Ok(Self::Forbidden),
            _code_str if Self::NotFound.same(_code_str) => Ok(Self::NotFound),
            _code_str if Self::MethodNotAllowed.same(_code_str) => Ok(Self::MethodNotAllowed),
            _code_str if Self::NotAcceptable.same(_code_str) => Ok(Self::NotAcceptable),
            _code_str if Self::ProxyAuthenticationRequired.same(_code_str) => {
                Ok(Self::ProxyAuthenticationRequired)
            }
            _code_str if Self::RequestTimeout.same(_code_str) => Ok(Self::RequestTimeout),
            _code_str if Self::Conflict.same(_code_str) => Ok(Self::Conflict),
            _code_str if Self::Gone.same(_code_str) => Ok(Self::Gone),
            _code_str if Self::LengthRequired.same(_code_str) => Ok(Self::LengthRequired),
            _code_str if Self::PreconditionFailed.same(_code_str) => Ok(Self::PreconditionFailed),
            _code_str if Self::PayloadTooLarge.same(_code_str) => Ok(Self::PayloadTooLarge),
            _code_str if Self::URITooLong.same(_code_str) => Ok(Self::URITooLong),
            _code_str if Self::UnsupportedMediaType.same(_code_str) => {
                Ok(Self::UnsupportedMediaType)
            }
            _code_str if Self::RangeNotSatisfiable.same(_code_str) => Ok(Self::RangeNotSatisfiable),
            _code_str if Self::ExpectationFailed.same(_code_str) => Ok(Self::ExpectationFailed),
            _code_str if Self::ImATeapot.same(_code_str) => Ok(Self::ImATeapot),
            _code_str if Self::MisdirectedRequest.same(_code_str) => Ok(Self::MisdirectedRequest),
            _code_str if Self::UnprocessableEntity.same(_code_str) => Ok(Self::UnprocessableEntity),
            _code_str if Self::Locked.same(_code_str) => Ok(Self::Locked),
            _code_str if Self::FailedDependency.same(_code_str) => Ok(Self::FailedDependency),
            _code_str if Self::TooEarly.same(_code_str) => Ok(Self::TooEarly),
            _code_str if Self::UpgradeRequired.same(_code_str) => Ok(Self::UpgradeRequired),
            _code_str if Self::PreconditionRequired.same(_code_str) => {
                Ok(Self::PreconditionRequired)
            }
            _code_str if Self::TooManyRequests.same(_code_str) => Ok(Self::TooManyRequests),
            _code_str if Self::RequestHeaderFieldsTooLarge.same(_code_str) => {
                Ok(Self::RequestHeaderFieldsTooLarge)
            }
            _code_str if Self::UnavailableForLegalReasons.same(_code_str) => {
                Ok(Self::UnavailableForLegalReasons)
            }
            _code_str if Self::InternalServerError.same(_code_str) => Ok(Self::InternalServerError),
            _code_str if Self::NotImplemented.same(_code_str) => Ok(Self::NotImplemented),
            _code_str if Self::BadGateway.same(_code_str) => Ok(Self::BadGateway),
            _code_str if Self::ServiceUnavailable.same(_code_str) => Ok(Self::ServiceUnavailable),
            _code_str if Self::GatewayTimeout.same(_code_str) => Ok(Self::GatewayTimeout),
            _code_str if Self::HTTPVersionNotSupported.same(_code_str) => {
                Ok(Self::HTTPVersionNotSupported)
            }
            _code_str if Self::VariantAlsoNegotiates.same(_code_str) => {
                Ok(Self::VariantAlsoNegotiates)
            }
            _code_str if Self::InsufficientStorage.same(_code_str) => Ok(Self::InsufficientStorage),
            _code_str if Self::LoopDetected.same(_code_str) => Ok(Self::LoopDetected),
            _code_str if Self::NotExtended.same(_code_str) => Ok(Self::NotExtended),
            _code_str if Self::NetworkAuthenticationRequired.same(_code_str) => {
                Ok(Self::NetworkAuthenticationRequired)
            }
            _ => Ok(Self::Unknown),
        }
    }
}

impl Default for StatusCode {
    #[inline]
    fn default() -> Self {
        Self::Ok
    }
}
