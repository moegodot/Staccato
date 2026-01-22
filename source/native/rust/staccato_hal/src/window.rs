use ::std::{ops::Deref, ptr::NonNull};
#[cfg(target_os = "macos")]
use std::ffi::c_void;
#[cfg(target_os = "windows")]
use std::num::NonZeroIsize;
use std::{mem::ManuallyDrop, ptr};
use std::num::NonZeroIsize;
use ::staccato_core::rect::Size;
use crate::error::SdlError;
use raw_window_handle::{
    AppKitDisplayHandle, AppKitWindowHandle, DisplayHandle, HandleError, RawDisplayHandle,
    RawWindowHandle, Win32WindowHandle, WindowsDisplayHandle,
};
use sdl3_sys::{
    render::SDL_DestroyRenderer,
    video::{
        SDL_DestroyWindow, SDL_PROP_WINDOW_WIN32_HWND_POINTER,
        SDL_PROP_WINDOW_WIN32_INSTANCE_POINTER, SDL_ShowWindow,
    },
};
use staccato_core::id::HasId;
use staccato_core::spatial::{HasSize, Resizable};
use staccato_platform_api::window::{WindowBackend, WindowId};

#[derive(Debug, PartialEq, Eq)]
pub struct Window {
    window: NonNull<sdl3_sys::video::SDL_Window>,
    window_handle: RawWindowHandle,
    display_handle: RawDisplayHandle,
    id: WindowId,
    #[cfg(target_os = "macos")]
    ns_view: NonNull<c_void>,
    #[cfg(target_os = "windows")]
    hwnd: NonZeroIsize,
    #[cfg(target_os = "windows")]
    hinstance: NonZeroIsize,
}

#[derive(Debug,Clone)]
pub struct WindowOption{
    pub title: String,
    pub width: i32,
    pub height: i32,
}

impl HasId for Window {
    type Id=WindowId;
    fn id(&self) -> Self::Id {
        self.id
    }
}

impl HasSize for Window {
    type SizeType=Size;

    fn get_size(&self) -> Self::SizeType {
        let mut w = 0;
        let mut h = 0;
        unsafe {
            if !sdl3_sys::video::SDL_GetWindowSize(self.as_ptr(), &mut w, &mut h) {
                return Err(SdlError::sdl_err("failed to get window size"));
            }
        }
        Ok(Size::new(w, h))
    }
}

impl Resizable for Window {
    type Error = ();

    fn try_set_size(&mut self, size: Self::SizeType) -> Result<(), Self::Error> {
        todo!()
    }
}

impl WindowBackend for Window {
    type Error = SdlError;

    fn title(&self) -> String {
        self.title()
    }

    fn set_title(&mut self, title: &str) -> Result<(), Self::Error> {
        self.set_title(title)
    }

    fn handler(&self) -> Box<dyn WindowHandle> {
        self.handles()
    }

    fn hide(&mut self) -> Result<(), Self::Error> {
        self.hide()
    }

    fn show(&mut self) -> Result<(), Self::Error> {
        self.show()
    }

    fn is_open(&self) -> bool {}
}

impl Window {
    pub fn new(option:WindowOption) -> Result<Self, SdlError> {
        let title = option.title;
        let width = option.width;
        let height = option.height;

        unsafe {
            let flags = if cfg!(target_os = "macos") {
                // we use metal on macos
                // for SDL and wgpu
                sdl3_sys::video::SDL_WindowFlags::RESIZABLE | sdl3_sys::video::SDL_WindowFlags::METAL
            } else if cfg!(target_os = "windows") {
                // windows or linux(todo) use vulkan
                // the `dx12` backend is disabled for wgpu
                sdl3_sys::video::SDL_WindowFlags::RESIZABLE | sdl3_sys::video::SDL_WindowFlags::VULKAN
            } else {
                todo!("linux or other platform support")
            };

            let c_title = std::ffi::CString::new(title)
                .map_err(|_| SdlError::sdl_err("invalid title string"))?;

            let window = NonNull::new(sdl3_sys::video::SDL_CreateWindow(
                c_title.as_ptr(),
                width,
                height,
                flags,
            ));

            let mut window = match window {
                Some(window) => window,
                None => return Err(SdlError::sdl_err("failed to create window")),
            };

            let mut window_err = window.clone();
            let mut error = |msg: &'static str| -> SdlError {
                let err = SdlError::sdl_err(msg);
                SDL_DestroyWindow(window_err.as_mut());
                return err;
            };

            if !SDL_ShowWindow(window.as_mut()) {
                return Err(error("failed to show window"));
            }

            let id = sdl3_sys::video::SDL_GetWindowID(window.as_ptr()).0 as WindowId;

            if cfg!(target_os = "macos") {
                let ns_view = sdl3_sys::metal::SDL_Metal_CreateView(window.as_ptr());
                let ns_view = match NonNull::new(ns_view) {
                    Some(ns_view) => ns_view,
                    None => return Err(error("failed to create NSView")),
                };

                let window_handle = RawWindowHandle::AppKit(AppKitWindowHandle::new(ns_view));

                let display_handle = RawDisplayHandle::AppKit(AppKitDisplayHandle::new());

                Ok(Self { window, window_handle, display_handle, ns_view,id })
            } else if cfg!(target_os = "windows") {
                let id = sdl3_sys::video::SDL_GetWindowProperties(window.as_ptr());

                let hwnd = sdl3_sys::properties::SDL_GetPointerProperty(id, SDL_PROP_WINDOW_WIN32_HWND_POINTER, core::ptr::null_mut());
                let hinstance = sdl3_sys::properties::SDL_GetPointerProperty(id, SDL_PROP_WINDOW_WIN32_INSTANCE_POINTER, core::ptr::null_mut());

                let hwnd = match NonZeroIsize::new(hwnd as isize) {
                    Some(hwnd) => hwnd,
                    None => {
                        return Err(error("missing window handle:hwnd"));
                    }
                };
                let hinstance = match NonZeroIsize::new(hinstance as isize) {
                    Some(hinstance) => hinstance,
                    None => {
                        return Err(error("missing window handle:hinstance"));
                    }
                };

                let mut handle = Win32WindowHandle::new(hwnd);
                handle.hinstance = hinstance.into();

                let window_handle = RawWindowHandle::Win32(handle);

                let display_handle = RawDisplayHandle::Windows(WindowsDisplayHandle::new());

                #[cfg(target_os = "windows")]
                return Ok(Self { window, window_handle, display_handle, hwnd, hinstance,id });
                #[cfg(not(target_os = "windows"))]
                unreachable!();
            } else {
                todo!("linux or other platform support")
            }
        }
    }

    pub fn handles(self:&Box<Self>) -> Box<WindowHandle> {
        Box::from( WindowHandle {
            window: self.window_handle,
            display: self.display_handle
        })
    }

    pub fn as_ptr(&self) -> *mut sdl3_sys::video::SDL_Window {
        self.window.as_ptr()
    }

    pub fn id(&self) -> u64 {
        return self.id
    }

    pub fn size(&self) -> Result<Size, SdlError> {
        let mut w = 0;
        let mut h = 0;
        unsafe {
            if !sdl3_sys::video::SDL_GetWindowSize(self.as_ptr(), &mut w, &mut h) {
                return Err(SdlError::sdl_err("failed to get window size"));
            }
        }
        Ok(Size::new(w, h))
    }

    pub fn set_size(&self, size: Size) -> Result<(), SdlError> {
        unsafe {
            if !sdl3_sys::video::SDL_SetWindowSize(self.as_ptr(), size.width, size.height) {
                return Err(SdlError::sdl_err("failed to set window size"));
            }
        }
        Ok(())
    }

    pub fn title(&self) -> String {
        unsafe {
            let title_ptr = sdl3_sys::video::SDL_GetWindowTitle(self.as_ptr());
            if title_ptr.is_null() {
                return "".to_string();
            }
            std::ffi::CStr::from_ptr(title_ptr)
                .to_string_lossy()
                .to_string()
        }
    }

    pub fn set_title(&self, title: &str) -> Result<(), SdlError> {
        let c_title =
            std::ffi::CString::new(title).map_err(|_| SdlError::sdl_err("invalid title string"))?;
        unsafe {
            if !sdl3_sys::video::SDL_SetWindowTitle(self.as_ptr(), c_title.as_ptr()) {
                return Err(SdlError::sdl_err("failed to set window title"));
            }
        }
        Ok(())
    }

    pub fn show(&self) -> Result<(), SdlError> {
        unsafe {
            if !sdl3_sys::video::SDL_ShowWindow(self.as_ptr()) {
                return Err(SdlError::sdl_err("failed to show window"));
            }
        }
        Ok(())
    }

    pub fn hide(&self) -> Result<(), SdlError> {
        unsafe {
            if !sdl3_sys::video::SDL_HideWindow(self.as_ptr()) {
                return Err(SdlError::sdl_err("failed to hide window"));
            }
        }
        Ok(())
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            if cfg!(target_os = "macos") {
                sdl3_sys::metal::SDL_Metal_DestroyView(self.ns_view.as_ptr());
            }
            SDL_DestroyWindow(self.window.as_ptr());
        }
    }
}

// impl !Send for Window {}
// impl !Sync for Window {}

#[derive(Debug, Clone, Copy)]
pub struct WindowHandle {
    window: RawWindowHandle,
    display: RawDisplayHandle,
}

unsafe impl Send for WindowHandle {}
unsafe impl Sync for WindowHandle {}

impl raw_window_handle::HasWindowHandle for WindowHandle {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, HandleError> {
        unsafe { Ok(raw_window_handle::WindowHandle::borrow_raw(self.window)) }
    }
}
impl raw_window_handle::HasDisplayHandle for WindowHandle {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        unsafe { Ok(DisplayHandle::borrow_raw(self.display)) }
    }
}
