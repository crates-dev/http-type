use crate::*;

/// WebSocket frame opcode types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WebSocketOpcode {
    /// Continuation frame (0x0)
    Continuation = 0x0,
    /// Text frame (0x1)
    Text = 0x1,
    /// Binary frame (0x2)
    Binary = 0x2,
    /// Connection close frame (0x8)
    Close = 0x8,
    /// Ping frame (0x9)
    Ping = 0x9,
    /// Pong frame (0xA)
    Pong = 0xA,
    /// Reserved for future use
    Reserved(u8),
}
