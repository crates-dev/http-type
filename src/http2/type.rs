use crate::*;

/// Type alias for an h2 `SendResponse` handle parameterized with the byte type
/// used by this crate.
pub type Http2SendResponse = h2::server::SendResponse<Bytes>;

/// Type alias for the result of accepting a new HTTP/2 request stream.
pub(crate) type Http2AcceptResult =
    Option<Result<(http::Request<h2::RecvStream>, Http2SendResponse), h2::Error>>;
