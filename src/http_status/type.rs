/// Enumeration of HTTP status codes representing various HTTP response statuses
///
/// This enum includes common HTTP status codes that cover successful requests, client errors,
/// and server errors. Each variant represents a specific HTTP status code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpStatus {
    /// 100 Continue
    Continue,
    /// 101 Switching Protocols
    SwitchingProtocols,
    /// 102 Processing (WebDAV)
    Processing,
    /// 103 Early Hints
    EarlyHints,
    /// 200 OK
    Ok,
    /// 201 Created
    Created,
    /// 202 Accepted
    Accepted,
    /// 203 Non-Authoritative Information
    NonAuthoritativeInformation,
    /// 204 No Content
    NoContent,
    /// 205 Reset Content
    ResetContent,
    /// 206 Partial Content
    PartialContent,
    /// 207 Multi-Status (WebDAV)
    MultiStatus,
    /// 208 Already Reported (WebDAV)
    AlreadyReported,
    /// 226 IM Used
    IMUsed,
    /// 300 Multiple Choices
    MultipleChoices,
    /// 301 Moved Permanently
    MovedPermanently,
    /// 302 Found
    Found,
    /// 303 See Other
    SeeOther,
    /// 304 Not Modified
    NotModified,
    /// 305 Use Proxy
    UseProxy,
    /// 307 Temporary Redirect
    TemporaryRedirect,
    /// 308 Permanent Redirect
    PermanentRedirect,
    /// 400 Bad Request
    BadRequest,
    /// 401 Unauthorized
    Unauthorized,
    /// 402 Payment Required
    PaymentRequired,
    /// 403 Forbidden
    Forbidden,
    /// 404 Not Found
    NotFound,
    /// 405 Method Not Allowed
    MethodNotAllowed,
    /// 406 Not Acceptable
    NotAcceptable,
    /// 407 Proxy Authentication Required
    ProxyAuthenticationRequired,
    /// 408 Request Timeout
    RequestTimeout,
    /// 409 Conflict
    Conflict,
    /// 410 Gone
    Gone,
    /// 411 Length Required
    LengthRequired,
    /// 412 Precondition Failed
    PreconditionFailed,
    /// 413 Payload Too Large
    PayloadTooLarge,
    /// 414 URI Too Long
    URITooLong,
    /// 415 Unsupported Media Type
    UnsupportedMediaType,
    /// 416 Range Not Satisfiable
    RangeNotSatisfiable,
    /// 417 Expectation Failed
    ExpectationFailed,
    /// 418 I'm a teapot
    ImATeapot,
    /// 421 Misdirected Request
    MisdirectedRequest,
    /// 422 Unprocessable Entity (WebDAV)
    UnprocessableEntity,
    /// 423 Locked (WebDAV)
    Locked,
    /// 424 Failed Dependency (WebDAV)
    FailedDependency,
    /// 425 Too Early
    TooEarly,
    /// 426 Upgrade Required
    UpgradeRequired,
    /// 428 Precondition Required
    PreconditionRequired,
    /// 429 Too Many Requests
    TooManyRequests,
    /// 431 Request Header Fields Too Large
    RequestHeaderFieldsTooLarge,
    /// 451 Unavailable For Legal Reasons
    UnavailableForLegalReasons,
    /// 500 Internal Server Error
    InternalServerError,
    /// 501 Not Implemented
    NotImplemented,
    /// 502 Bad Gateway
    BadGateway,
    /// 503 Service Unavailable
    ServiceUnavailable,
    /// 504 Gateway Timeout
    GatewayTimeout,
    /// 505 HTTP Version Not Supported
    HTTPVersionNotSupported,
    /// 506 Variant Also Negotiates
    VariantAlsoNegotiates,
    /// 507 Insufficient Storage (WebDAV)
    InsufficientStorage,
    /// 508 Loop Detected (WebDAV)
    LoopDetected,
    /// 510 Not Extended
    NotExtended,
    /// 511 Network Authentication Required
    NetworkAuthenticationRequired,
    /// Unknown status code
    Unknown,
}
