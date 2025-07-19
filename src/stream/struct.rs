use crate::*;

/// A wrapper around `Arc<RwLock<TcpStream>>`.
///
/// `ArcRwLockStream` provides shared, thread-safe access to a `TcpStream`
/// using an atomic reference counter (`Arc`) combined with a read-write lock (`RwLock`).
/// It is primarily used to safely share the stream across asynchronous tasks.
///
/// # Fields
/// - `0`: The inner `Arc<RwLock<TcpStream>>` stream.
#[derive(Clone, Debug, Getter)]
pub struct ArcRwLockStream(#[get(pub(super))] pub(super) ArcRwLock<TcpStream>);
