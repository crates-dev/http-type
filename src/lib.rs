#![allow(unused_imports)]

//! http-type
//!
//! A comprehensive Rust type library for HTTP operations and concurrent programming.
//! Provides core HTTP types (Request/Response with builder patterns, Method, HttpStatus, HttpVersion,
//! ContentType, FileExtension with MIME mapping, Cookie parsing/building, HttpUrl parsing,
//! WebSocket frame/opcode, protocol upgrade types, stream/task management, panic handling),
//! thread-safe concurrent wrappers (ArcMutex, ArcRwLock, BoxRwLock, RcRwLock),
//! dynamic dispatch types (BoxAny, RcAny, ArcAny with Send/Sync variants),
//! high-performance hash collections (HashMapXxHash3_64, HashSetXxHash3_64),
//! and static lifetime utilities (BoxLeak, Lifetime trait).

mod any;
mod arc_mutex;
mod arc_rwlock;
mod attribute;
mod box_leak;
mod box_rwlock;
mod connection;
mod content_type;
mod cookie;
mod file_extension;
mod hash_map_xx_hash3_64;
mod hash_set_xx_hash3_64;
mod http2;
mod http3;
mod http_status;
mod http_url;
mod http_version;
mod lifetime;
mod methods;
mod panic;
mod protocol;
mod rc_rwlock;
mod request;
mod response;
mod status;
mod stream;
mod task;
mod tls;
mod upgrade_type;
mod websocket_frame;

pub use {
    any::*, arc_mutex::*, arc_rwlock::*, attribute::*, box_leak::*, box_rwlock::*, connection::*,
    content_type::*, cookie::*, file_extension::*, hash_map_xx_hash3_64::*,
    hash_set_xx_hash3_64::*, http_status::*, http_url::*, http_version::*, http2::*, http3::*,
    lifetime::*, methods::*, panic::*, protocol::*, rc_rwlock::*, request::*, response::*,
    status::*, stream::*, task::*, tls::*, upgrade_type::*, websocket_frame::*,
};

pub use {
    http_compress::*,
    http_constant::*,
    serde_json::{self},
    tokio::{self},
};

use std::{
    any::Any,
    collections::{HashMap, HashSet, VecDeque},
    error::Error as StdError,
    fmt::{self, Debug, Display, Formatter},
    hash::Hash,
    io::{Cursor, Error as IoError, ErrorKind},
    mem::forget,
    net::{IpAddr, SocketAddr},
    num::ParseIntError,
    path::Path,
    pin::Pin,
    ptr::{self, addr_of_mut},
    rc::Rc,
    result::Result,
    str::{FromStr, SplitWhitespace},
    sync::{
        Arc,
        atomic::{self, AtomicBool, AtomicUsize},
    },
    time::Duration,
};

use {
    bytes::{self, Buf, Bytes},
    core::hash::BuildHasherDefault,
    h2::{self},
    h3::{self},
    h3_quinn::{self},
    http::{self},
    lombok_macros::*,
    quinn::{self},
    rustls::{self},
    rustls_pemfile::{self},
    serde::{Deserialize, Serialize, de::DeserializeOwned},
    tokio::{
        fs::read as tokio_fs_read,
        io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader},
        net::TcpStream,
        runtime::Handle,
        sync::{
            Mutex, Notify, RwLock, RwLockReadGuard, RwLockWriteGuard,
            mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
        },
        task::{JoinError, LocalSet, spawn_blocking, spawn_local},
        time::{error::Elapsed, timeout},
    },
    url::{ParseError, Url},
};
