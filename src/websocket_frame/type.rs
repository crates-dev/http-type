use crate::*;

pub type WebsocketFrameWithLengthOption = Option<(WebSocketFrame, usize)>;

/// Represents a decoded WebSocket frame
#[derive(Debug, Clone, Lombok, Default, DisplayDebug)]
pub struct WebSocketFrame {
    /// FIN flag indicating if this is the final frame
    #[set(skip)]
    pub(crate) fin: bool,
    /// Opcode indicating the frame type (text, binary, etc.)
    #[set(skip)]
    pub(crate) opcode: u8,
    /// Mask flag indicating if the payload is masked
    #[set(skip)]
    pub(crate) mask: bool,
    /// The payload data of the frame
    #[set(skip)]
    pub(crate) payload_data: Vec<u8>,
}
