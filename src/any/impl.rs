use crate::*;

impl AnySend for DynAnySend {}
impl AnySync for DynAnySync {}
impl AnySendSync for DynAnySendSync {}
impl AnySend for Arc<dyn Any + Send + Sync> {}
impl AnySync for Arc<dyn Any + Send + Sync> {}
impl AnySendSync for Arc<dyn Any + Send + Sync> {}
