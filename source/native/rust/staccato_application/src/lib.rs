use std::convert::Infallible;
use std::ffi::{CStr, CString, NulError};
use std::path::Path;
use sdl3_sys::init::{SDL_SetAppMetadataProperty, SDL_PROP_APP_METADATA_NAME_STRING, SDL_PROP_APP_METADATA_COPYRIGHT_STRING, SDL_PROP_APP_METADATA_CREATOR_STRING, SDL_PROP_APP_METADATA_IDENTIFIER_STRING, SDL_PROP_APP_METADATA_TYPE_STRING, SDL_PROP_APP_METADATA_URL_STRING, SDL_PROP_APP_METADATA_VERSION_STRING, SDL_Init, SDL_InitFlags, SDL_Quit};
use thiserror::Error;
use staccato_core::fallible::Fallible;
use staccato_core::tickable::Tickable;
use staccato_hal::error::SdlError;
use staccato_hal::wgpu_window::WgpuWindow;

pub use staccato_core;
pub use staccato_shared;
pub use staccato_platform_api;
pub use staccato_render_api;
pub use staccato_hal;
use staccato_telemetry::{initialize, TelemetryGuard};

pub struct ApplicationGuard{
    telemetry_guard: TelemetryGuard,
    app_info: ApplicationInformation
}

impl Drop for ApplicationGuard{
    fn drop(&mut self) {
        unsafe { SDL_Quit() }
    }
}

impl ApplicationGuard{
    pub fn info(&self) -> &ApplicationInformation{
        &self.app_info
    }
}

pub struct ApplicationInformation {
    pub name:String,
    pub version:String,
    pub identifier:String,
    pub creator:String,
    pub copyright:String,
    pub url:String,
    pub app_type:String
}

impl ApplicationInformation {
    pub fn initialize_app(self, log_directory:Option<&Path>) -> eyre::Result<ApplicationGuard>{
        let telemetry_guard = initialize(log_directory).unwrap();

        if !unsafe {
            SDL_Init(
                SDL_InitFlags::AUDIO |
                    SDL_InitFlags::CAMERA |
                    SDL_InitFlags::EVENTS |
                    SDL_InitFlags::HAPTIC |
                    SDL_InitFlags::GAMEPAD |
                    SDL_InitFlags::JOYSTICK |
                    SDL_InitFlags::SENSOR |
                    SDL_InitFlags::VIDEO
            )
        }{
            return Err(SdlError::sdl_err("failed to initialize sdl system").into());
        }
        self.set_sdl_property()?;

        Ok(crate::ApplicationGuard{
            telemetry_guard,
            app_info: self
        })
    }

    fn set_sdl_property(&self) -> eyre::Result<()>{
        unsafe{
            let key = SDL_PROP_APP_METADATA_NAME_STRING;
            let value = CString::new(self.name.clone())?;
            if !SDL_SetAppMetadataProperty(key,value.as_ptr()){
                return Err(SdlError::sdl_err("failed to set app metadata - name").into());
            }

            let key = SDL_PROP_APP_METADATA_VERSION_STRING;
            let value = CString::new(self.version.clone())?;
            if !SDL_SetAppMetadataProperty(key,value.as_ptr()){
                return Err(SdlError::sdl_err("failed to set app metadata - version").into());
            }

            let key = SDL_PROP_APP_METADATA_IDENTIFIER_STRING;
            let value = CString::new(self.identifier.clone())?;
            if !SDL_SetAppMetadataProperty(key,value.as_ptr()){
                return Err(SdlError::sdl_err("failed to set app metadata - identifier").into());
            }

            let key = SDL_PROP_APP_METADATA_CREATOR_STRING;
            let value = CString::new(self.creator.clone())?;
            if !SDL_SetAppMetadataProperty(key,value.as_ptr()){
                return Err(SdlError::sdl_err("failed to set app metadata - creator").into());
            }

            let key = SDL_PROP_APP_METADATA_COPYRIGHT_STRING;
            let value = CString::new(self.copyright.clone())?;
            if !SDL_SetAppMetadataProperty(key,value.as_ptr()){
                return Err(SdlError::sdl_err("failed to set app metadata - copyright").into());
            }

            let key = SDL_PROP_APP_METADATA_URL_STRING;
            let value = CString::new(self.url.clone())?;
            if !SDL_SetAppMetadataProperty(key,value.as_ptr()){
                return Err(SdlError::sdl_err("failed to set app metadata - url").into());
            }

            let key = SDL_PROP_APP_METADATA_TYPE_STRING;
            let value = CString::new(self.app_type.clone())?;
            if !SDL_SetAppMetadataProperty(key,value.as_ptr()){
                return Err(SdlError::sdl_err("failed to set app metadata - app_type").into());
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ApplicationWindow<'window> {
    window: WgpuWindow<'window>,
}

impl Fallible for ApplicationWindow<'_>{
    type Error = Infallible;
}

impl Tickable for ApplicationWindow<'_>{
    fn pre_update(&mut self, elapse_ns: u64) -> Result<(), Self::Error> {
        todo!()
    }

    fn fixed_update(&mut self, elapse_ns: u64) -> Result<(), Self::Error> {
        todo!()
    }

    fn update(&mut self, elapse_ns: u64) -> Result<(), Self::Error> {
        todo!()
    }

    fn post_update(&mut self, elapse_ns: u64) -> Result<(), Self::Error> {
        todo!()
    }
}
