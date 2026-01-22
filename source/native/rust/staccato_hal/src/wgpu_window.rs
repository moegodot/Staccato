use std::mem::ManuallyDrop;
use eyre::{Context, Report};
use pollster::FutureExt;
use thiserror::Error;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, SurfaceTarget};
use staccato_core::spatial::HasSize;
use staccato_platform_api::window::WindowBackend;
use crate::error::SdlError;
use crate::window::{RawWindowHandler, Window, WindowOption};

#[derive(Debug,Error)]
pub enum WgpuWindowError{
    #[error("Get a sdl error:{0}")]
    SdlError(#[from] SdlError),
    #[error("Get an error when initialize wgpu and window:{0}")]
    Other(#[from] Report)
}

#[derive(Debug,Clone)]
pub struct WgpuWindowOption{
}

pub struct WgpuWindow<'window>{
    window: ManuallyDrop<Box<Window>>,
    surface:ManuallyDrop<Surface<'window>>,
    device:ManuallyDrop<Device>,
    queue:ManuallyDrop<Queue>,
    config: ManuallyDrop<SurfaceConfiguration>,
}

impl Drop for WgpuWindow<'_>{
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.surface);
            ManuallyDrop::drop(&mut self.queue);
            ManuallyDrop::drop(&mut self.config);
            ManuallyDrop::drop(&mut self.device);
            ManuallyDrop::drop(&mut self.window);
        }
    }
}

impl<'window> WgpuWindow<'window>{

    pub fn from_window(option:WgpuWindowOption, window: Window) -> Result<Self,WgpuWindowError> {
        let window = Box::from(window);

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::METAL | wgpu::Backends::VULKAN,
            ..Default::default()
        });

        let surface = instance.create_surface(SurfaceTarget::Window(window.window().handler())).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .block_on()
            .unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                label: None,
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            }
        ).block_on().unwrap();

        let caps = surface.get_capabilities(&adapter);
        let mut size = window.get_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: size.width.try_into().wrap_err("failed to convert windows width to u32")?,
            height: size.height.try_into().wrap_err("failed to convert windows height to u32")?,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Ok(
            Self {
                window:ManuallyDrop::new(window),
                surface:ManuallyDrop::new(surface),
                device:ManuallyDrop::new(device),
                queue:ManuallyDrop::new(queue),
                config:ManuallyDrop::new(config),
            }
        )
    }

    pub fn new(option:WgpuWindowOption,window_option: WindowOption) -> Result<Self,WgpuWindowError> {
        let window = Window::new(window_option)?;

        Self::from_window(option,window)
    }

    pub fn window(&self) -> &Box<Window> {
        &self.window
    }
    
    pub fn device(&self) -> &Device {
        &self.device
    }
    
    pub fn queue(&self) -> &Queue {
        &self.queue
    }
    
    pub fn surface(&self) -> &Surface {
        &self.surface
    }
}
