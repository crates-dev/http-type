use crate::*;

pub trait AnySend: Any + Send {}
pub trait AnySendClone: Any + Send + Clone {}
pub trait AnySync: Any + Sync {}
pub trait AnySyncClone: Any + Sync + Clone {}
pub trait AnySendSync: Any + Send + Sync {}
pub trait AnySendSyncClone: Any + Send + Sync + Clone {}
