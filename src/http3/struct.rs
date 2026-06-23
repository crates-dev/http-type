use crate::*;

/// Handles HTTP/3 connections for a request handler.
///
/// Stores the handler closure; call `handle` with a QUIC connection to start
/// accepting HTTP/3 requests.
#[derive(New)]
pub struct Http3Connection<F> {
    /// Handler invoked for each decoded HTTP/3 request. Receives the request
    /// and a default response, and returns the response to send.
    pub(super) handler: F,
}

/// Wraps an HTTP/3 response together with its h3 request stream.
///
/// Call `send` to transmit the response over the provided stream.
#[derive(New)]
pub struct Http3Response<'a> {
    /// The response to send.
    pub(super) response: &'a Response,
    /// The h3 request stream.
    pub(super) stream: &'a mut Http3RequestStream,
}
