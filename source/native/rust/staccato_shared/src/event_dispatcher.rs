use crate::event::Event;
use staccato_core::fallible::Fallible;
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait EventSource<E = Event>: Debug {
    fn poll(&mut self) -> &[E];
}

pub trait EventHandler<E = Event>: Fallible + Debug {
    /// return false to request next event handler
    fn handle(&mut self, event: &E) -> Result<bool, Self::Error>;
}

pub trait EventDispatcher<E = Event>: Fallible {
    /// return is_handled
    fn fire(
        &self,
        handlers: &mut [&mut dyn EventHandler<E, Error = Self::Error>],
        event: &E,
    ) -> Result<bool, Self::Error>;
}

#[derive(Debug)]
pub struct StdEventDispatcher<Event, Err: std::error::Error + Sync + Send + 'static> {
    event_type: PhantomData<Event>,
    error_type: PhantomData<Err>,
}

impl<Event, Err: std::error::Error + Sync + Send + 'static> Default
    for StdEventDispatcher<Event, Err>
{
    fn default() -> Self {
        Self {
            event_type: Default::default(),
            error_type: Default::default(),
        }
    }
}

impl<Event, Err: std::error::Error + Sync + Send + 'static> Fallible
    for StdEventDispatcher<Event, Err>
{
    type Error = Err;
}

impl<Event, Err: std::error::Error + Sync + Send + 'static> EventDispatcher<Event>
    for StdEventDispatcher<Event, Err>
{
    fn fire(
        &self,
        handlers: &mut [&mut dyn EventHandler<Event, Error = Self::Error>],
        event: &Event,
    ) -> Result<bool, Self::Error> {
        for handler in handlers {
            if handler.handle(event)? {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
