use crate::*;

impl<T: Any + Send> AnySend for T {}
impl<T: Any + Send + Clone> AnySendClone for T {}
impl<T: Any + Sync> AnySync for T {}
impl<T: Any + Sync + Clone> AnySyncClone for T {}
impl<T: Any + Send + Sync> AnySendSync for T {}
impl<T: Any + Send + Sync + Clone> AnySendSyncClone for T {}
