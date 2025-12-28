use crate::*;

/// A thread-safe reference-counted `TcpStream`.
pub type ArcStream = Arc<TcpStream>;

/// A read guard for a `RwLock<TcpStream>`.
pub type RwLockReadGuardTcpStream<'a> = RwLockReadGuard<'a, TcpStream>;

/// A write guard for a `RwLock<TcpStream>`.
pub type RwLockWriteGuardTcpStream<'a> = RwLockWriteGuard<'a, TcpStream>;

/// A thread-safe reference to a `RwLock` write guard for `TcpStream`.
pub type ArcRwLockWriteGuardTcpStream<'a> = Arc<RwLockWriteGuard<'a, TcpStream>>;

/// A thread-safe reference to a `Mutex` guard for `TcpStream`.
pub type ArcMutexGuardTcpStream<'a> = Arc<MutexGuard<'a, TcpStream>>;

/// A socket host represented by an IP address.
pub type SocketHost = IpAddr;

/// A socket port number.
pub type SocketPort = u16;
