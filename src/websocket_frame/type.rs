use crate::*;

pub type WebsocketFrameWithLengthOption = Option<(WebSocketFrame, usize)>;

/// Represents a decoded WebSocket frame
#[derive(Debug, Clone, Lombok, Default, DisplayDebug)]
pub struct WebSocketFrame {
    /// FIN flag indicating if this is the final frame
    #[set(skip)]
    pub(super) fin: bool,
    /// Opcode indicating the frame type (text, binary, etc.)
    #[set(skip)]
    pub(super) opcode: u8,
    /// Mask flag indicating if the payload is masked
    #[set(skip)]
    pub(super) mask: bool,
    /// The payload data of the frame
    #[set(skip)]
    pub(super) payload_data: Vec<u8>,
}
