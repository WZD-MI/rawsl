pub mod a_tcp;
use std::io::{Read, Result, Write};
use std::net::SocketAddr;
use std::time::Duration;
pub trait Connection: Read + Write {
    fn peer_addr(&self) -> Result<SocketAddr>;
    fn local_addr(&self) -> Result<SocketAddr>;
    fn start_addr(&self) -> Result<SocketAddr>;
    fn end_addr(&self) -> Result<SocketAddr>;
    fn set_read_timeout(&self, dur: Option<Duration>) -> Result<()>;
    fn set_write_timeout(&self, dur: Option<Duration>) -> Result<()>;
}
