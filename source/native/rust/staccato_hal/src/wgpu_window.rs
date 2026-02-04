use crate::error::SdlError;
use crate::wgpu_context::WgpuRenderContext;
use crate::window::Window;
use eyre::{Context, Report};
use staccato_core::spatial::HasSize;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use thiserror::Error;
use wgpu::{CreateSurfaceError, Device, Queue, Surface, SurfaceConfiguration, SurfaceTarget};

#[derive(Debug, Error)]
pub enum WgpuWindowError {
    #[error("Get a sdl error:{0}")]
    SdlError(#[from] SdlError),
    #[error("Get a create surface error:{0}")]
    CreateSurfaceError(#[from] CreateSurfaceError),
    #[error("Get an error when initialize wgpu and window:{0}")]
    Other(#[from] Report),
}

#[derive(Debug)]
pub struct WgpuWindow<'window> {
    window: ManuallyDrop<Box<Window>>,
    surface: ManuallyDrop<Surface<'window>>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    config: ManuallyDrop<SurfaceConfiguration>,
}

impl Drop for WgpuWindow<'_> {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.config);
            ManuallyDrop::drop(&mut self.surface);
            ManuallyDrop::drop(&mut self.window);
        }
    }
}

impl<'window> WgpuWindow<'window> {
    pub fn from_window(
        context: WgpuRenderContext,
        window: Window,
    ) -> Result<Self, WgpuWindowError> {
        let window = Box::from(window);

        let surface = context
            .instance()
            .create_surface(SurfaceTarget::Window(window.window().handler()))?;

        let caps = surface.get_capabilities(context.adapter());
        let mut size = window.get_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: size
                .width
                .try_into()
                .wrap_err("failed to convert windows width to u32")?,
            height: size
                .height
                .try_into()
                .wrap_err("failed to convert windows height to u32")?,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(context.device(), &config);

        Ok(Self {
            window: ManuallyDrop::new(window),
            surface: ManuallyDrop::new(surface),
            config: ManuallyDrop::new(config),
            device: context.device().clone(),
            queue: context.queue().clone(),
        })
    }

    pub fn from_window_and_surface(
        context: WgpuRenderContext,
        window: Window,
        surface: Surface<'window>,
    ) -> Result<Self, WgpuWindowError> {
        let window = Box::from(window);

        let caps = surface.get_capabilities(context.adapter());
        let mut size = window.get_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: size
                .width
                .try_into()
                .wrap_err("failed to convert windows width to u32")?,
            height: size
                .height
                .try_into()
                .wrap_err("failed to convert windows height to u32")?,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(context.device(), &config);

        Ok(Self {
            window: ManuallyDrop::new(window),
            surface: ManuallyDrop::new(surface),
            config: ManuallyDrop::new(config),
            device: context.device().clone(),
            queue: context.queue().clone(),
        })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    pub fn surface(&self) -> &Surface<'_> {
        &self.surface
    }
}
