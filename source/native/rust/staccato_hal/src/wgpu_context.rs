use crate::wgpu_window::{WgpuWindow, WgpuWindowError};
use crate::window::Window;
use pollster::FutureExt;
use std::sync::Arc;
use thiserror::Error;
use wgpu::{
    CreateSurfaceError, DeviceDescriptor, InstanceDescriptor, RequestAdapterError,
    RequestAdapterOptions, RequestDeviceError, SurfaceTarget,
};

#[derive(Debug, Error)]
pub enum ContextError {
    #[error("failed to request adfapter:{0}")]
    RequestAdapterError(#[from] RequestAdapterError),
    #[error("failed to request device:{0}")]
    RequestDeviceError(#[from] RequestDeviceError),
    #[error("failed to create surface:{0}")]
    CreateSurfaceError(#[from] CreateSurfaceError),
    #[error("failed to create wgpu window:{0}")]
    WgpuWindowError(#[from] WgpuWindowError),
}

#[derive(Debug, Clone)]
pub struct WgpuRenderContext {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
}

impl WgpuRenderContext {
    pub fn new(
        instance_descriptor: &InstanceDescriptor,
        adapter_options: &RequestAdapterOptions<'_, '_>,
        device_descriptor: &DeviceDescriptor<'_>,
    ) -> Result<Self, ContextError> {
        let instance = wgpu::Instance::new(instance_descriptor);
        let adapter = instance.request_adapter(adapter_options).block_on()?;

        let (device, queue) = adapter.request_device(device_descriptor).block_on()?;

        Ok(Self {
            instance,
            adapter,
            device: Arc::new(device),
            queue: Arc::new(queue),
        })
    }

    pub fn new_with_window<'window>(
        window: Window,
        instance_descriptor: &InstanceDescriptor,
        adapter_options: &RequestAdapterOptions<'_, '_>,
        device_descriptor: &DeviceDescriptor<'_>,
    ) -> Result<(Self, WgpuWindow<'window>), ContextError> {
        let instance = wgpu::Instance::new(instance_descriptor);

        let surface = instance.create_surface(SurfaceTarget::Window(window.window().handler()))?;

        let mut adapter_options = adapter_options.clone();

        let surface_ref = &surface;

        adapter_options.compatible_surface = Some(surface_ref);

        let adapter = instance.request_adapter(&adapter_options).block_on()?;

        let (device, queue) = adapter.request_device(device_descriptor).block_on()?;

        let context = Self {
            instance,
            adapter,
            device: Arc::new(device),
            queue: Arc::new(queue),
        };

        let window = WgpuWindow::from_window_and_surface(context.clone(), window, surface)?;

        Ok((context, window))
    }

    pub fn instance(&self) -> &wgpu::Instance {
        &self.instance
    }

    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    pub fn device(&self) -> &Arc<wgpu::Device> {
        &self.device
    }

    pub fn queue(&self) -> &Arc<wgpu::Queue> {
        &self.queue
    }
}
