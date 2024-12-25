use std::sync::{Arc, RwLock};

pub type ArcRwLock<T> = Arc<RwLock<T>>;
