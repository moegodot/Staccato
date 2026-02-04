use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use smol_str::SmolStr;
use staccato_core::fallible::Fallible;
use staccato_core::id::HasId;
use staccato_core::rect::Size;
use staccato_core::spatial::Resizable;
pub use staccato_shared::id::WindowId;
use std::fmt::Debug;

pub trait WindowHandle: Send + Sync + HasWindowHandle + HasDisplayHandle {}

impl<T: Send + Sync + HasWindowHandle + HasDisplayHandle> WindowHandle for T {}

pub trait WindowBackend:
    Debug + HasId<Id = WindowId> + Resizable<SizeType = Size> + Fallible
{
    fn title(&self) -> SmolStr;
    fn set_title(&mut self, title: &str) -> Result<(), Self::Error>;
    fn handler(&self) -> Box<dyn WindowHandle + 'static>;
    fn hide(&mut self) -> Result<(), Self::Error>;
    fn show(&mut self) -> Result<(), Self::Error>;
    fn is_open(&self) -> bool;
}
