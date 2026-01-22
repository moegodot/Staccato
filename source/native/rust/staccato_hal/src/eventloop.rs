use sdl3_sys::events::{SDL_Event, SDL_EventType, SDL_PollEvent};
use crate::event::Event;

#[derive(Debug)]
pub struct EventLoop{

}

impl EventLoop{
    pub fn new() -> Self{
        Self{}
    }

    pub fn poll_events<F>(&self, mut handler: F)
    where F: FnMut(Event)
    {
        unsafe {
            let mut event = std::mem::zeroed();

            while SDL_PollEvent(&mut event) {

                let et = event.r#type;

                if et == SDL_EventType::QUIT.0{
                    handler(Event::Quit);
                    break;
                }
                
            }
        }
    }
}

pub fn poll_event() -> Option<SDL_Event>{
    let mut event = SDL_Event::default();

    if !unsafe { SDL_PollEvent(&mut event) }{
        return None;
    }

    Some(event)
}
