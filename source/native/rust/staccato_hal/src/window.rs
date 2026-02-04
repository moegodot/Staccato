use crate::error::SdlError;
use ::staccato_core::rect::Size;
use ::std::ptr::NonNull;
use raw_window_handle::{
    AppKitDisplayHandle, AppKitWindowHandle, DisplayHandle, HandleError, RawDisplayHandle,
    RawWindowHandle, Win32WindowHandle, WindowsDisplayHandle,
};
use sdl3_sys::video::{
    SDL_DestroyWindow, SDL_PROP_WINDOW_WIN32_HWND_POINTER, SDL_PROP_WINDOW_WIN32_INSTANCE_POINTER,
    SDL_ShowWindow,
};
use smol_str::{SmolStr, ToSmolStr};
use staccato_core::fallible::Fallible;
use staccato_core::id::HasId;
use staccato_core::spatial::{HasSize, Resizable};
use staccato_platform_api::window::{WindowBackend, WindowId};
#[cfg(target_os = "macos")]
use std::ffi::c_void;

#[derive(Debug, PartialEq, Eq)]
pub struct WindowHandler {
    window: NonNull<sdl3_sys::video::SDL_Window>,
    window_handle: RawWindowHandle,
    display_handle: RawDisplayHandle,
    id: WindowId,
    is_open: bool,
    #[cfg(target_os = "macos")]
    ns_view: NonNull<c_void>,
    #[cfg(target_os = "windows")]
    hwnd: NonZeroIsize,
    #[cfg(target_os = "windows")]
    hinstance: NonZeroIsize,
}

#[derive(Debug, Clone)]
pub struct WindowOption {
    pub title: String,
    pub size: Size,
}

#[derive(Debug)]
pub struct Window {
    window: WindowHandler,
    title: SmolStr,
    size: Size,
}

impl HasId for Window {
    type Id = WindowId;
    fn id(&self) -> Self::Id {
        self.window.id
    }
}

impl HasSize for Window {
    type SizeType = Size;

    fn get_size(&self) -> Self::SizeType {
        self.size
    }
}

impl Resizable for Window {
    fn try_set_size(&mut self, size: Self::SizeType) -> Result<(), Self::Error> {
        self.window.set_size(size)
    }
}

impl Fallible for Window {
    type Error = SdlError;
}

impl Window {
    pub fn update(&mut self) -> Result<(), SdlError> {
        self.title = self.window.title()?;
        self.size = self.window.size()?;
        Ok(())
    }

    pub fn window(&self) -> &WindowHandler {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut WindowHandler {
        &mut self.window
    }

    pub fn new(option: WindowOption) -> Result<Self, SdlError> {
        let window = WindowHandler::new(option)?;
        let title = window.title()?;
        let size = window.size()?;
        Ok(Self {
            window,
            title,
            size,
        })
    }
}

impl WindowBackend for Window {
    fn title(&self) -> SmolStr {
        self.title.clone()
    }

    fn set_title(&mut self, title: &str) -> Result<(), Self::Error> {
        self.window.set_title(title)
    }

    fn handler(&self) -> Box<dyn staccato_platform_api::window::WindowHandle + 'static> {
        self.window.handler()
    }

    fn hide(&mut self) -> Result<(), Self::Error> {
        self.window.hide()
    }

    fn show(&mut self) -> Result<(), Self::Error> {
        self.window.show()
    }

    fn is_open(&self) -> bool {
        self.window.is_open
    }
}

impl WindowHandler {
    pub fn new(option: WindowOption) -> Result<Self, SdlError> {
        let title = option.title;
        let width = option.size.width;
        let height = option.size.height;

        let is_open = true;

        unsafe {
            let flags = if cfg!(target_os = "macos") {
                // we use metal on macos
                // for SDL and wgpu
                sdl3_sys::video::SDL_WindowFlags::RESIZABLE
                    | sdl3_sys::video::SDL_WindowFlags::METAL
            } else if cfg!(target_os = "windows") {
                // windows or linux(todo) use vulkan
                // the `dx12` backend is disabled for wgpu
                sdl3_sys::video::SDL_WindowFlags::RESIZABLE
                    | sdl3_sys::video::SDL_WindowFlags::VULKAN
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

            let mut window_err = window;
            let mut error = |msg: &'static str| -> SdlError {
                let err = SdlError::sdl_err(msg);
                SDL_DestroyWindow(window_err.as_mut());
                err
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

                Ok(Self {
                    window,
                    window_handle,
                    display_handle,
                    ns_view,
                    id,
                    is_open,
                })
            } else if cfg!(target_os = "windows") {
                let id = sdl3_sys::video::SDL_GetWindowProperties(window.as_ptr());

                let hwnd = sdl3_sys::properties::SDL_GetPointerProperty(
                    id,
                    SDL_PROP_WINDOW_WIN32_HWND_POINTER,
                    core::ptr::null_mut(),
                );
                let hinstance = sdl3_sys::properties::SDL_GetPointerProperty(
                    id,
                    SDL_PROP_WINDOW_WIN32_INSTANCE_POINTER,
                    core::ptr::null_mut(),
                );

                let hwnd = match std::num::NonZeroIsize::new(hwnd as isize) {
                    Some(hwnd) => hwnd,
                    None => {
                        return Err(error("missing window handle:hwnd"));
                    }
                };
                let hinstance = match std::num::NonZeroIsize::new(hinstance as isize) {
                    Some(hinstance) => hinstance,
                    None => {
                        return Err(error("missing window handle:hinstance"));
                    }
                };

                let mut handle = Win32WindowHandle::new(hwnd);
                handle.hinstance = hinstance.into();

                let _window_handle = RawWindowHandle::Win32(handle);

                let _display_handle = RawDisplayHandle::Windows(WindowsDisplayHandle::new());

                #[cfg(target_os = "windows")]
                return Ok(Self {
                    window,
                    window_handle: _window_handle,
                    display_handle: _display_handle,
                    hwnd,
                    hinstance,
                    id,
                    is_open,
                });
                #[cfg(not(target_os = "windows"))]
                unreachable!();
            } else {
                todo!("linux or other platform support")
            }
        }
    }

    pub fn handler(&self) -> Box<RawWindowHandler> {
        Box::from(RawWindowHandler {
            window: self.window_handle,
            display: self.display_handle,
        })
    }

    pub fn as_ptr(&self) -> *mut sdl3_sys::video::SDL_Window {
        self.window.as_ptr()
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn size(&self) -> Result<Size, SdlError> {
        self.checked_open()?;
        let mut w = 0;
        let mut h = 0;
        unsafe {
            if !sdl3_sys::video::SDL_GetWindowSize(self.as_ptr(), &mut w, &mut h) {
                return Err(SdlError::sdl_err("failed to get window size"));
            }
        }
        Ok(Size::new(w, h))
    }

    pub fn set_size(&mut self, size: Size) -> Result<(), SdlError> {
        self.checked_open()?;
        unsafe {
            if !sdl3_sys::video::SDL_SetWindowSize(self.as_ptr(), size.width, size.height) {
                return Err(SdlError::sdl_err("failed to set window size"));
            }
        }
        Ok(())
    }

    pub fn title(&self) -> Result<SmolStr, SdlError> {
        self.checked_open()?;
        unsafe {
            let title_ptr = sdl3_sys::video::SDL_GetWindowTitle(self.as_ptr());
            if title_ptr.is_null() {
                Ok("".to_smolstr())
            } else {
                Ok(std::ffi::CStr::from_ptr(title_ptr)
                    .to_string_lossy()
                    .to_smolstr())
            }
        }
    }

    pub fn set_title(&mut self, title: &str) -> Result<(), SdlError> {
        self.checked_open()?;
        let c_title =
            std::ffi::CString::new(title).map_err(|_| SdlError::sdl_err("invalid title string"))?;
        unsafe {
            if !sdl3_sys::video::SDL_SetWindowTitle(self.as_ptr(), c_title.as_ptr()) {
                return Err(SdlError::sdl_err("failed to set window title"));
            }
        }
        Ok(())
    }

    pub fn show(&mut self) -> Result<(), SdlError> {
        self.checked_open()?;
        unsafe {
            if !sdl3_sys::video::SDL_ShowWindow(self.as_ptr()) {
                return Err(SdlError::sdl_err("failed to show window"));
            }
        }
        Ok(())
    }

    pub fn hide(&mut self) -> Result<(), SdlError> {
        self.checked_open()?;
        unsafe {
            if !sdl3_sys::video::SDL_HideWindow(self.as_ptr()) {
                return Err(SdlError::sdl_err("failed to hide window"));
            }
        }
        Ok(())
    }

    fn checked_open(&self) -> Result<(), SdlError> {
        if !self.is_open {
            Err(SdlError::sdl_err("try to operate a closed window"))
        } else {
            Ok(())
        }
    }

    pub fn close(&mut self) {
        self.is_open = false;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

impl Drop for WindowHandler {
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
pub struct RawWindowHandler {
    window: RawWindowHandle,
    display: RawDisplayHandle,
}

unsafe impl Send for RawWindowHandler {}
unsafe impl Sync for RawWindowHandler {}

impl raw_window_handle::HasWindowHandle for RawWindowHandler {
    fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, HandleError> {
        unsafe { Ok(raw_window_handle::WindowHandle::borrow_raw(self.window)) }
    }
}

impl raw_window_handle::HasDisplayHandle for RawWindowHandler {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        unsafe { Ok(DisplayHandle::borrow_raw(self.display)) }
    }
}
