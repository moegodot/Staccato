use crate::sdl_event;
use crate::sdl_event::translate_event;
use sdl3_sys::events::SDL_EVENT_POLL_SENTINEL;
use sdl3_sys::everything::SDL_PollEvent;
use staccato_shared::event::Event;
use staccato_shared::event_dispatcher::{EventDispatcher, EventSource};
use tracing::{error, trace};

#[derive(Debug, Default)]
pub struct SdlEventSource {
    buffer: Vec<Event>, // 预分配空间，永不释放
}

impl EventSource for SdlEventSource {
    fn poll(&mut self) -> &[Event] {
        self.buffer.clear();

        unsafe {
            let mut event = std::mem::zeroed();

            while SDL_PollEvent(&mut event) {
                if event.r#type == SDL_EVENT_POLL_SENTINEL {
                    break;
                }

                let event = translate_event(event);

                self.buffer.push(event);
            }
        }

        for event in &self.buffer {
            error!("collect event {event:?}")
        }

        self.buffer.as_slice()
    }
}
