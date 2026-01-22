use eyre::{Context, Report};
use pollster::FutureExt;
use thiserror::Error;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, SurfaceTarget};
use crate::error::SdlError;
use crate::window::{Window, WindowOption};

#[derive(Debug,Error)]
pub enum WgpuWindowError{
    #[error("Get a sdl error:{0}")]
    SdlError(#[from] SdlError),
    #[error("Get an error when initialize wgpu and window:{0}")]
    Other(#[from] Report)
}

#[derive(Debug,Clone)]
pub struct WgpuWindowOption{
    window: WindowOption,

}

pub struct WgpuWindow<'window>{
    window: Box<Window>,
    surface:Surface<'window>,
    device:Device,
    queue:Queue,
    config: SurfaceConfiguration,
}

impl<'window> WgpuWindow<'window>{
    pub fn new(option:WgpuWindowOption) -> Result<Box<Self>,WgpuWindowError> {
        let window = Box::new(Window::new(option.window)?);

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::METAL | wgpu::Backends::VULKAN,
            ..Default::default()
        });

        let surface = instance.create_surface(SurfaceTarget::Window(window.handles())).unwrap();

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
        let mut size = window.size()?;
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
            Box::new(Self {
                window,
                surface,
                device,
                queue,
                config,
            })
        )
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