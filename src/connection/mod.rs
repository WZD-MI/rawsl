pub mod a_tcp;
use std::io::Result;
use std::net::SocketAddr;
use std::time::Duration;
pub trait Connection {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn peer_addr(&self) -> Result<SocketAddr>;
    fn local_addr(&self) -> Result<SocketAddr>;
    fn end_addr(&self) -> Result<SocketAddr>;
    fn set_read_timeout(&self, dur: Option<Duration>) -> Result<()>;
    fn set_write_timeout(&self, dur: Option<Duration>) -> Result<()>;
}
