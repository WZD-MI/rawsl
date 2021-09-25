use std::net::TcpListener;
pub trait Server {
    fn listen(&self) -> TcpListener;
}
