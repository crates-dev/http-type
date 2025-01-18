use std::sync::Arc;

pub type ArcTcpStream = Arc<std::net::TcpStream>;
pub type OptionArcTcpStream = Option<ArcTcpStream>;

pub type ArcTokioStream = Arc<tokio::net::TcpStream>;
pub type OptionArcTokioTcpStream = Option<ArcTokioStream>;
