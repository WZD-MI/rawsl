use crate::listener::a_tcp_listener::ATCPListener;
use crate::listener::Listener;
use crate::server::Server;
use std::net::TcpListener;
pub struct TCP {
    ip: &'static str,
    port: u16,
}
impl TCP {
    pub fn new(ip: &'static str, port: u16) -> TCP {
        TCP { ip: ip, port: port }
    }
}

impl Server for TCP {
    fn listen(&self) -> Box<dyn Listener> {
        let str = format!("{}:{}", self.ip, self.port);
        println!("listen: {}", str);
        let listener = TcpListener::bind(str).unwrap();
        let al = ATCPListener::new(listener);
        Box::new(al)
    }
}

#[cfg(test)]
mod tests {
    use crate::server::{tcp::TCP, Server};
    #[test]
    fn test_tcp() {
        let s = "0.0.0.0";
        let tcp: Box<dyn Server> = Box::new(TCP::new(s, 60080));
        let l = tcp.listen();
        match l.accept() {
            Ok((_socket, addr)) => {
                println!("new client: {:?}", addr);
            }
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}
