use crate::*;

/// Handles HTTP/2 connections for a request handler.
///
/// Stores the handler closure; call `handle` with an I/O stream to start
/// accepting HTTP/2 requests.
#[derive(New)]
pub struct Http2Connection<F> {
    /// Handler invoked for each decoded HTTP/2 request. Receives the request
    /// and a default response, and returns the response to send.
    pub(super) handler: F,
}

/// Wraps an HTTP/2 response together with its h2 sender.
///
/// Call `send` to transmit the response over the provided `SendResponse` handle.
#[derive(New)]
pub struct Http2Response<'a> {
    /// The response to send.
    pub(super) response: &'a Response,
    /// The h2 response sender.
    pub(super) respond: Http2SendResponse,
}
