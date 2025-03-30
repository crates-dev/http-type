use crate::*;

pub type BoxAny = Box<dyn Any>;
pub type ArcAny = Arc<dyn Any>;
pub type BoxAnySend = Box<dyn Any + Send>;
pub type ArcAnySend = Arc<dyn Any + Send>;
pub type BoxAnySync = Box<dyn Any + Sync>;
pub type ArcAnySync = Arc<dyn Any + Sync>;
pub type BoxAnySendSync = Box<dyn Any + Send + Sync>;
pub type ArcAnySendSync = Arc<dyn Any + Send + Sync>;
