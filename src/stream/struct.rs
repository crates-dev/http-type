use crate::*;

/// Thread-safe TCP stream wrapper.
///
/// Provides shared access to a TcpStream using Arc and RwLock.
///
/// # Fields
///
/// - `Arc<RwLock<TcpStream>>` - The protected TCP stream.
#[derive(Clone, Debug, Getter)]
pub struct ArcRwLockStream(#[get(pub(super))] pub(super) ArcRwLock<TcpStream>);
