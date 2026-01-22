use crate::event::{Event};

pub trait EventLoop{
    fn poll(&mut self) -> &[Event];
}
