//! http-type
//!
//! A comprehensive Rust library providing essential types for HTTP operations.
//! Includes core HTTP abstractions (request/response, methods, status codes, versions),
//! content types, cookies, WebSocket support, and thread-safe
//! concurrent types (ArcMutex, ArcRwLock). Also provides convenient
//! Option-wrapped primitive types for flexible HTTP handling.

mod any;
mod arc_mutex;
mod arc_rwlock;
mod box_rwlock;
mod content_type;
mod cookie;
mod file_extension;
mod hash_map_xx_hash3_64;
mod hash_set_xx_hash3_64;
mod http_status;
mod http_url;
mod http_version;
mod methods;
mod protocol;
mod rc_rwlock;
mod request;
mod response;
mod stream;
mod upgrade_type;
mod websocket_frame;

pub use {
    any::*, arc_mutex::*, arc_rwlock::*, box_rwlock::*, content_type::*, cookie::*,
    file_extension::*, hash_map_xx_hash3_64::*, hash_set_xx_hash3_64::*, http_status::*,
    http_url::*, http_version::*, methods::*, protocol::*, rc_rwlock::*, request::*, response::*,
    stream::*, upgrade_type::*, websocket_frame::*,
};

pub use {http_compress::*, http_constant::*, serde_json, tokio};

use std::{
    any::Any,
    collections::{HashMap, HashSet, VecDeque},
    fmt::{self, Debug, Display},
    hash::Hash,
    io::ErrorKind,
    net::IpAddr,
    num::ParseIntError,
    rc::Rc,
    result::Result,
    str::{FromStr, SplitWhitespace},
    sync::Arc,
    time::Duration,
};

use {
    core::hash::BuildHasherDefault,
    lombok_macros::*,
    serde::{Deserialize, Serialize, de::DeserializeOwned},
    tokio::{
        io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
        net::TcpStream,
        sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard},
        time::{error::Elapsed, timeout},
    },
    url::{ParseError, Url},
};
