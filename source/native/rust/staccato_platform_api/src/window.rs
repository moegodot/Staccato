use std::fmt::Debug;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use staccato_core::id::HasId;
use staccato_core::rect::Size;
use staccato_core::spatial::{Resizable};
use staccato_shared::id::WindowId;

pub trait WindowHandle : Send + Sync + HasWindowHandle + HasDisplayHandle{}

pub trait WindowBackend : Debug + HasId<Id = WindowId> + Resizable<SizeType = Size>{
    type Error: std::error::Error + Send + Sync + 'static;
    fn title(&self) -> String;
    fn set_title(&mut self,title:&str) -> Result<(),<Self as WindowBackend>::Error>;
    fn handler(&self) -> Box<dyn WindowHandle>;
    fn hide(&mut self) -> Result<(),<Self as WindowBackend>::Error>;
    fn show(&mut self) -> Result<(),<Self as WindowBackend>::Error>;
    fn is_open(&self) -> bool;
}
