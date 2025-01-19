use crate::*;
use std::sync::Arc;

pub type ArcTcpStream = Arc<std::net::TcpStream>;
pub type ArcLockStream = ArcRwLock<ArcTcpStream>;
pub type OptionArcTcpStream = Option<ArcTcpStream>;
pub type OptionArcLockStream = Option<ArcLockStream>;

pub type ArcTokioStream = Arc<tokio::net::TcpStream>;
pub type ArcLockTokioStream = ArcRwLock<tokio::net::TcpStream>;
pub type OptionArcTokioTcpStream = Option<ArcTokioStream>;
pub type OptionArcLockTokioStream = Option<ArcLockTokioStream>;
