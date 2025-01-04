use std::{net::TcpStream, sync::Arc};

pub type ArcTcpStream = Arc<TcpStream>;
pub type OptionArcTcpStream = Option<ArcTcpStream>;
