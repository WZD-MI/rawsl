pub mod a_tcp_listener;
use crate::connection::Connection;
use std::io::Result;
use std::net::SocketAddr;

pub trait Listener {
    fn accept(&self) -> Result<(Box<dyn Connection>, SocketAddr)>;
}
