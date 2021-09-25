use std::net::TcpListener;
pub struct TCP {
    ip: String,
    port: u16,
}
use crate::server::server::Server;
impl TCP {
    pub fn new(ip: String, port: u16) -> TCP {
        TCP { ip: ip, port: port }
    }
}

impl Server for TCP {
    fn listen(&self) -> TcpListener {
        let str = format!("{}:{}", self.ip, self.port);
        println!("listen: {}", str);
        let listener = TcpListener::bind(str).unwrap();
        listener
    }
}

#[cfg(test)]
mod tests {
    use crate::server::{server::Server, tcp::TCP};
    #[test]
    fn test_tcp() {
        let tcp = TCP::new("0.0.0.0".to_string(), 60080);
        let l = tcp.listen();
        match l.accept() {
            Ok((_socket, addr)) => {
                println!("new client: {:?}", addr);
            }
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}
