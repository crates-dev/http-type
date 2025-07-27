use crate::*;

/// An alias for `Option<(WebSocketFrame, usize)>`, representing a decoded WebSocket frame along with the number of bytes consumed.
pub type WebsocketFrameWithLengthOption = Option<(WebSocketFrame, usize)>;
