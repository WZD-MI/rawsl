use crate::connection::Connection;
use std::io::{Read, Result, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
pub struct ATCP {
    end_addr: SocketAddr,
    tcp: TcpStream,
}

impl ATCP {
    pub fn new(tcp: TcpStream, end_addr: SocketAddr) -> ATCP {
        ATCP {
            tcp: tcp,
            end_addr: end_addr,
        }
    }
}

impl Connection for ATCP {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.tcp.read(buf)
    }
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.tcp.write(buf)
    }
    fn peer_addr(&self) -> Result<SocketAddr> {
        self.tcp.peer_addr()
    }
    fn local_addr(&self) -> Result<SocketAddr> {
        self.tcp.local_addr()
    }
    fn end_addr(&self) -> Result<SocketAddr> {
        Result::Ok(self.end_addr)
    }
    fn set_read_timeout(&self, dur: Option<Duration>) -> Result<()> {
        self.tcp.set_read_timeout(dur)
    }
    fn set_write_timeout(&self, dur: Option<Duration>) -> Result<()> {
        self.tcp.set_write_timeout(dur)
    }
}
