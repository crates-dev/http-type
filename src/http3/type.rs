use crate::*;

/// Type alias for an h3 server connection over a Quinn connection.
pub(crate) type Http3ServerConnection =
    h3::server::Connection<h3_quinn::Connection, Bytes>;

/// Type alias for an h3 request stream used to receive data and send responses.
pub type Http3RequestStream =
    h3::server::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes>;
