use crate::*;

/// A thread-safe reference-counted `TcpStream`.
pub type ArcStream = Arc<TcpStream>;
/// An optional thread-safe reference-counted `TcpStream`.
pub type OptionArcTcpStream = Option<ArcStream>;
/// An optional thread-safe read-write locked `TcpStream` wrapper.
pub type OptionArcRwLockStream = Option<ArcRwLockStream>;
/// A read guard for a `RwLock<TcpStream>`.
pub type RwLockReadGuardTcpStream<'a> = RwLockReadGuard<'a, TcpStream>;
/// A write guard for a `RwLock<TcpStream>`.
pub type RwLockWriteGuardTcpStream<'a> = RwLockWriteGuard<'a, TcpStream>;
/// A thread-safe reference to a `RwLock` write guard for `TcpStream`.
pub type ArcRwLockWriteGuardTcpStream<'a> = Arc<RwLockWriteGuard<'a, TcpStream>>;
/// An optional thread-safe reference to a `RwLock` write guard for `TcpStream`.
pub type OptionArcRwLockWriteGuardTcpStream<'a> = Option<ArcRwLockWriteGuardTcpStream<'a>>;
/// A thread-safe reference to a `Mutex` guard for `TcpStream`.
pub type ArcMutexGuardTcpStream<'a> = Arc<MutexGuard<'a, TcpStream>>;
/// An optional thread-safe reference to a `Mutex` guard for `TcpStream`.
pub type OptionArcMutexGuardTcpStream<'a> = Option<ArcMutexGuardTcpStream<'a>>;
/// A socket host represented by an IP address.
pub type SocketHost = IpAddr;
/// A socket port number.
pub type SocketPort = u16;
/// An optional socket host.
pub type OptionSocketHost = Option<SocketHost>;
/// An optional socket port.
pub type OptionSocketPort = Option<SocketPort>;
/// An optional full socket address.
pub type OptionSocketAddr = Option<SocketAddr>;
