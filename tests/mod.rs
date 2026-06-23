#![allow(unused_imports)]

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
mod http2;
mod http3;
mod hash_map_xx_hash3_64;
mod hash_set_xx_hash3_64;
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

use {http2::http2_test_handler, http3::http3_test_handler, http_type::*, lifetime::*};

use std::{
    collections::VecDeque,
    error::Error as StdError,
    future::poll_fn,
    io::{Cursor, Error as IoError, ErrorKind},
    mem::{self, forget},
    net::SocketAddr,
    num::ParseIntError,
    ptr::{self, addr_of_mut},
    rc::Rc,
    sync::{
        Arc,
        atomic::{self, AtomicBool, AtomicUsize},
    },
};

use {
    bytes::{Buf, Bytes},
    http_type::*,
    serde::Deserialize,
    tokio::{
        io::AsyncWriteExt,
        net::{TcpListener, TcpStream},
        runtime::Handle,
        spawn,
        sync::{RwLockReadGuard, RwLockWriteGuard},
        task::{JoinError, JoinHandle},
        time::{Duration, error::Elapsed},
    },
    url::{ParseError, Url},
};
