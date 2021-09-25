pub mod tcp;
use crate::listener::Listener;
pub trait Server {
    fn listen(&self) -> Box<dyn Listener>;
}
