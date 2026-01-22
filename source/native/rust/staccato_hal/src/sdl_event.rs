use eyre::OptionExt;
use sdl3_sys::events::{SDL_Event, SDL_EventType, SDL_QuitEvent};
use staccato_core::keycode::KeyCode;
use staccato_core::keymod::Keymod;
use staccato_core::scancode::Scancode;
use staccato_shared::event::{DeviceOperation, Event, RawEvent, UserOperation};
use staccato_shared::event::RawEvent::{Keyboard, KeyboardDevice, MouseDevice, Quit};
use num_traits::cast::FromPrimitive;

pub fn translate_event(sdl:SDL_Event) -> Event {
    let sdl_type = SDL_EventType(unsafe { sdl.r#type });

    unsafe {
        match sdl_type {
            SDL_EventType::QUIT => {
                let sdl = &sdl.quit;

                Event{
                    ns_timestamp: sdl.timestamp,
                    event: Quit
                }
            },
            SDL_EventType::MOUSE_ADDED => {
                let sdl = &sdl.mdevice;

                Event{
                    ns_timestamp: sdl.timestamp,
                    event: MouseDevice {
                        mouse_id: sdl.which.0.into(),
                        operation: DeviceOperation::Added
                    }
                }
            },
            SDL_EventType::MOUSE_REMOVED => {
                let sdl = &sdl.mdevice;

                Event{
                    ns_timestamp: sdl.timestamp,
                    event: MouseDevice {
                        mouse_id: sdl.which.0.into(),
                        operation: DeviceOperation::Removed
                    }
                }
            },
            SDL_EventType::KEYBOARD_ADDED => {
                let sdl = &sdl.kdevice;

                Event{
                    ns_timestamp: sdl.timestamp,
                    event: KeyboardDevice {
                        keyboard_id: sdl.which.0.into(),
                        operation: DeviceOperation::Added
                    }
                }
            },
            SDL_EventType::KEYBOARD_REMOVED => {
                let sdl = &sdl.kdevice;

                Event{
                    ns_timestamp: sdl.timestamp,
                    event: KeyboardDevice {
                        keyboard_id: sdl.which.0.into(),
                        operation: DeviceOperation::Removed
                    }
                }
            },
            SDL_EventType::KEY_UP => {
                let sdl = &sdl.key;

                Event{
                    ns_timestamp: sdl.timestamp,
                    event: Keyboard {
                        window_id: sdl.windowID.0.into(),
                        keyboard_id: sdl.which.0.into(),
                        scan_code: Scancode::from_i32(sdl.scancode.0).unwrap(),
                        key_code: KeyCode::from_u32(sdl.key.0).unwrap(),
                        keymod: Keymod::from_bits(sdl.r#mod.0).unwrap(),
                        is_down: sdl.down,
                        is_repeat: sdl.repeat,
                        user_operation: UserOperation::Up,
                    }
                }
            }
            _ => todo!(),
        }
    }
}
