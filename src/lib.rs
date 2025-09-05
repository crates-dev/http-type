//! http-type
//!
//! A comprehensive Rust library providing essential types for HTTP operations.
//! Includes core HTTP abstractions (request/response, methods, status codes, versions),
//! content types, cookies, WebSocket support, and thread-safe
//! concurrent types (ArcMutex, ArcRwLock). Also provides convenient
//! Option-wrapped primitive types for flexible HTTP handling.

pub(crate) mod any;
pub(crate) mod arc_mutex;
pub(crate) mod arc_rwlock;
pub(crate) mod box_rwlock;
pub(crate) mod content_type;
pub(crate) mod cookie;
pub(crate) mod file_extension;
pub(crate) mod hash_map_xx_hash3_64;
pub(crate) mod hash_set_xx_hash3_64;
pub(crate) mod http_status;
pub(crate) mod http_url;
pub(crate) mod http_version;
pub(crate) mod json;
pub(crate) mod methods;
pub(crate) mod option_bool;
pub(crate) mod option_compress;
pub(crate) mod option_duration;
pub(crate) mod option_i128;
pub(crate) mod option_i32;
pub(crate) mod option_i64;
pub(crate) mod option_str;
pub(crate) mod option_string;
pub(crate) mod option_u128;
pub(crate) mod option_u16;
pub(crate) mod option_u32;
pub(crate) mod option_u64;
pub(crate) mod option_usize;
pub(crate) mod option_vec_u8;
pub(crate) mod protocol;
pub(crate) mod rc_rwlock;
pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod stream;
pub(crate) mod upgrade_type;
pub(crate) mod websocket_frame;

pub use any::*;
pub use arc_mutex::*;
pub use arc_rwlock::*;
pub use box_rwlock::*;
pub use content_type::*;
pub use cookie::*;
pub use file_extension::*;
pub use hash_map_xx_hash3_64::*;
pub use hash_set_xx_hash3_64::*;
pub use http_status::*;
pub use http_url::*;
pub use http_version::*;
pub use json::*;
pub use methods::*;
pub use option_bool::*;
pub use option_compress::*;
pub use option_duration::*;
pub use option_i32::*;
pub use option_i64::*;
pub use option_i128::*;
pub use option_str::*;
pub use option_string::*;
pub use option_u16::*;
pub use option_u32::*;
pub use option_u64::*;
pub use option_u128::*;
pub use option_usize::*;
pub use option_vec_u8::*;
pub use protocol::*;
pub use rc_rwlock::*;
pub use request::*;
pub use response::*;
pub use stream::*;
pub use upgrade_type::*;
pub use websocket_frame::*;

pub use http_compress::*;
pub use http_constant::*;
pub use tokio;

pub(crate) use core::hash::BuildHasherDefault;
pub(crate) use lombok_macros::*;
pub(crate) use serde::{Serialize, de::DeserializeOwned};
pub(crate) use std::{
    any::Any,
    borrow::Cow,
    collections::{HashMap, HashSet, VecDeque},
    error::Error as StdError,
    fmt::{self, Debug, Display},
    hash::Hash,
    io::ErrorKind,
    net::{IpAddr, SocketAddr},
    rc::Rc,
    result::Result,
    str::FromStr,
    sync::Arc,
    time::Duration,
};
pub(crate) use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard},
};
pub(crate) use url::Url as UrlParser;
