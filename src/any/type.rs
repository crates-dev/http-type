use crate::*;

/// A type alias for a boxed `Any` trait object.
///
/// This represents a dynamically dispatched trait object that owns its data.
pub type BoxAny = Box<dyn Any>;
/// An optional `BoxAny`.
pub type OptionBoxAny = Option<BoxAny>;
/// A type alias for an `Rc` wrapped `Any` trait object.
///
/// This represents a reference-counted trait object that can be shared across threads.
pub type RcAny = Rc<dyn Any>;
/// An optional `RcAny`.
pub type OptionRcAny = Option<RcAny>;
/// A type alias for an `Arc` wrapped `Any` trait object.
///
/// This represents an atomically reference-counted trait object that can be shared across threads.
pub type ArcAny = Arc<dyn Any>;
/// An optional `ArcAny`.
pub type OptionArcAny = Option<ArcAny>;
/// A type alias for a boxed `Any + Send` trait object.
///
/// This represents a dynamically dispatched trait object that owns its data and is safe to send across threads.
pub type BoxAnySend = Box<dyn Any + Send>;
/// An optional `BoxAnySend`.
pub type OptionBoxAnySend = Option<BoxAnySend>;
/// A type alias for an `Rc` wrapped `Any + Send` trait object.
///
/// This represents a reference-counted trait object that can be shared across threads and is safe to send.
pub type RcAnySend = Rc<dyn Any + Send>;
/// An optional `RcAnySend`.
pub type OptionRcAnySend = Option<RcAnySend>;
/// A type alias for an `Arc` wrapped `Any + Send` trait object.
///
/// This represents an atomically reference-counted trait object that can be shared across threads and is safe to send.
pub type ArcAnySend = Arc<dyn Any + Send>;
/// An optional `ArcAnySend`.
pub type OptionArcAnySend = Option<ArcAnySend>;
/// A type alias for a boxed `Any + Sync` trait object.
///
/// This represents a dynamically dispatched trait object that owns its data and is safe to share across threads.
pub type BoxAnySync = Box<dyn Any + Sync>;
/// An optional `BoxAnySync`.
pub type OptionBoxAnySync = Option<BoxAnySync>;
/// A type alias for an `Rc` wrapped `Any + Sync` trait object.
///
/// This represents a reference-counted trait object that can be shared across threads and is safe to share.
pub type RcAnySync = Rc<dyn Any + Sync>;
/// An optional `RcAnySync`.
pub type OptionRcAnySync = Option<RcAnySync>;
/// A type alias for an `Arc` wrapped `Any + Sync` trait object.
///
/// This represents an atomically reference-counted trait object that can be shared across threads and is safe to share.
pub type ArcAnySync = Arc<dyn Any + Sync>;
/// An optional `ArcAnySync`.
pub type OptionArcAnySync = Option<ArcAnySync>;
/// A type alias for a boxed `Any + Send + Sync` trait object.
///
/// This represents a dynamically dispatched trait object that owns its data and is safe to send and share across threads.
pub type BoxAnySendSync = Box<dyn Any + Send + Sync>;
/// An optional `BoxAnySendSync`.
pub type OptionBoxAnySendSync = Option<BoxAnySendSync>;
/// A type alias for an `Rc` wrapped `Any + Send + Sync` trait object.
///
/// This represents a reference-counted trait object that can be shared across threads and is safe to send and share.
pub type RcAnySendSync = Rc<dyn Any + Send + Sync>;
/// An optional `RcAnySendSync`.
pub type OptionRcAnySendSync = Option<RcAnySendSync>;
/// A type alias for an `Arc` wrapped `Any + Send + Sync` trait object.
///
/// This represents an atomically reference-counted trait object that can be shared across threads and is safe to send and share.
pub type ArcAnySendSync = Arc<dyn Any + Send + Sync>;
/// An optional `ArcAnySendSync`.
pub type OptionArcAnySendSync = Option<ArcAnySendSync>;
