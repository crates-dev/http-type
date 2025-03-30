use crate::*;

pub trait AnySend: Any + Send {}
pub trait AnySync: Any + Sync {}
pub trait AnySendSync: Any + Send + Sync {}
