use sdl3_sys::everything::SDL_PollEvent;
use staccato_shared::event::{Event, RawEvent};
use staccato_shared::event_loop::EventLoop;
use crate::sdl_event;

pub struct SdlEventLoop {
    buffer: Vec<Event>, // 预分配空间，永不释放
}

impl EventLoop for SdlEventLoop {
    fn poll(&mut self) -> &[Event] {
        self.buffer.clear(); // 清空计数，但不释放内存空间

        unsafe {
            let mut sdl_event = std::mem::zeroed();
            while SDL_PollEvent(&mut sdl_event) {
                self.buffer.push(sdl_event::translate_event(sdl_event));
            }
        }

        &self.buffer // 返回借用
    }
}
