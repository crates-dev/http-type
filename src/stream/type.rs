use crate::*;

pub type ArcTcpStream = std::sync::Arc<std::net::TcpStream>;
pub type ArcLockStream = ArcRwLock<std::net::TcpStream>;
pub type OptionArcTcpStream = Option<ArcTcpStream>;
pub type OptionArcLockStream = Option<ArcLockStream>;
pub type ArcRwLockWriteGuardTcpStream<'a> =
    std::sync::Arc<std::sync::RwLockWriteGuard<'a, std::net::TcpStream>>;
pub type OptionArcRwLockWriteGuardTcpStream<'a> = Option<ArcRwLockWriteGuardTcpStream<'a>>;
pub type ArcMutexGuardTcpStream<'a> =
    std::sync::Arc<std::sync::MutexGuard<'a, std::net::TcpStream>>;
pub type OptionArcMutexGuardTcpStream<'a> = Option<ArcMutexGuardTcpStream<'a>>;

pub type ArcTokioStream = std::sync::Arc<tokio::net::TcpStream>;
pub type ArcLockTokioStream = ArcRwLock<tokio::net::TcpStream>;
pub type OptionArcTokioTcpStream = Option<ArcTokioStream>;
pub type OptionArcLockTokioStream = Option<ArcLockTokioStream>;
pub type ArcRwLockWriteGuardTokioTcpStream<'a> =
    std::sync::Arc<std::sync::RwLockWriteGuard<'a, tokio::net::TcpStream>>;
pub type OptionArcRwLockWriteGuardTokioTcpStream<'a> =
    Option<ArcRwLockWriteGuardTokioTcpStream<'a>>;
pub type ArcMutexGuardTokioTcpStream<'a> =
    std::sync::Arc<std::sync::MutexGuard<'a, std::net::TcpStream>>;
pub type OptionArcMutexGuardTokioTcpStream<'a> = Option<ArcMutexGuardTokioTcpStream<'a>>;
