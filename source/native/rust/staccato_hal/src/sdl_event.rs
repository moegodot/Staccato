use std::ffi::{c_char, CStr};

use num_traits::cast::FromPrimitive;
use sdl3_sys::events::{SDL_Event, SDL_EventType};
use staccato_core::frect::FPoint;
use staccato_core::keycode::KeyCode;
use staccato_core::keymod::Keymod;
use staccato_core::mouse::{Button, MouseWheelDirection};
use staccato_core::rect::Point;
use staccato_core::scancode::Scancode;
use staccato_shared::event::{AppEvent, DeviceOperation, Event, InlineText, RawEvent, UserOperation, WindowEvent};

fn inline_text_from_ptr(ptr: *const c_char) -> InlineText {
    if ptr.is_null() {
        return InlineText::empty();
    }

    let bytes = unsafe { CStr::from_ptr(ptr).to_bytes() };
    InlineText::from_bytes(bytes)
}

fn button_from_index(button: u8) -> Button {
    match button {
        1 => Button::Left,
        2 => Button::Middle,
        3 => Button::Right,
        4 => Button::Side1,
        5 => Button::Side2,
        _ => Button::empty(),
    }
}

pub fn translate_event(sdl:SDL_Event) -> Event {
    let sdl_type = SDL_EventType(unsafe { sdl.r#type });

    unsafe {
        match sdl_type {
            SDL_EventType::QUIT => {
                let sdl = &sdl.quit;

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::Quit
                }
            },
            SDL_EventType::TERMINATING => {
                let sdl = &sdl.common;

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::App { event: AppEvent::Terminating }
                }
            },
            SDL_EventType::LOW_MEMORY => {
                let sdl = &sdl.common;

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::App { event: AppEvent::LowMemory }
                }
            },
            SDL_EventType::WILL_ENTER_BACKGROUND => {
                let sdl = &sdl.common;

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::App { event: AppEvent::WillEnterBackground }
                }
            },
            SDL_EventType::DID_ENTER_BACKGROUND => {
                let sdl = &sdl.common;

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::App { event: AppEvent::DidEnterBackground }
                }
            },
            SDL_EventType::WILL_ENTER_FOREGROUND => {
                let sdl = &sdl.common;

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::App { event: AppEvent::WillEnterForeground }
                }
            },
            SDL_EventType::DID_ENTER_FOREGROUND => {
                let sdl = &sdl.common;

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::App { event: AppEvent::DidEnterForeground }
                }
            },
            SDL_EventType::LOCALE_CHANGED => {
                let sdl = &sdl.common;

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::App { event: AppEvent::LocaleChanged }
                }
            },
            SDL_EventType::SYSTEM_THEME_CHANGED => {
                let sdl = &sdl.common;

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::App { event: AppEvent::SystemThemeChanged }
                }
            },
            SDL_EventType::MOUSE_ADDED => {
                let sdl = &sdl.mdevice;

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::MouseDevice {
                        mouse_id: sdl.which.0.into(),
                        operation: DeviceOperation::Added
                    }
                }
            },
            SDL_EventType::MOUSE_REMOVED => {
                let sdl = &sdl.mdevice;

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::MouseDevice {
                        mouse_id: sdl.which.0.into(),
                        operation: DeviceOperation::Removed
                    }
                }
            },
            SDL_EventType::KEYBOARD_ADDED => {
                let sdl = &sdl.kdevice;

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::KeyboardDevice {
                        keyboard_id: sdl.which.0.into(),
                        operation: DeviceOperation::Added
                    }
                }
            },
            SDL_EventType::KEYBOARD_REMOVED => {
                let sdl = &sdl.kdevice;

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::KeyboardDevice {
                        keyboard_id: sdl.which.0.into(),
                        operation: DeviceOperation::Removed
                    }
                }
            },
            SDL_EventType::KEYMAP_CHANGED => {
                let sdl = &sdl.common;

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::KeymapChanged
                }
            },
            SDL_EventType::KEY_DOWN | SDL_EventType::KEY_UP => {
                let sdl = &sdl.key;
                let is_down = sdl.down;
                let user_operation = if is_down {
                    UserOperation::Down
                } else {
                    UserOperation::Up
                };

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::Keyboard {
                        window_id: sdl.windowID.0.into(),
                        keyboard_id: sdl.which.0.into(),
                        scan_code: Scancode::from_i32(sdl.scancode.0).unwrap_or(Scancode::Unknown),
                        key_code: KeyCode::from_u32(sdl.key.0).unwrap_or(KeyCode::Unknown),
                        keymod: Keymod::from_bits_truncate(sdl.r#mod.0),
                        raw_scancode: sdl.raw,
                        is_down,
                        is_repeat: sdl.repeat,
                        user_operation,
                    }
                }
            },
            SDL_EventType::TEXT_EDITING => {
                let sdl = &sdl.edit;
                let text = inline_text_from_ptr(sdl.text);

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::TextEditing {
                        window_id: sdl.windowID.0.into(),
                        text,
                        start: sdl.start,
                        length: sdl.length,
                    }
                }
            },
            SDL_EventType::TEXT_INPUT => {
                let sdl = &sdl.text;
                let text = inline_text_from_ptr(sdl.text);

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::TextInput {
                        window_id: sdl.windowID.0.into(),
                        text,
                    }
                }
            },
            SDL_EventType::MOUSE_MOTION => {
                let sdl = &sdl.motion;

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::MouseMotion {
                        window_id: sdl.windowID.0.into(),
                        mouse_id: sdl.which.0.into(),
                        state: Button::from_bits_truncate(sdl.state.0),
                        position: FPoint::new(sdl.x, sdl.y),
                        relative: FPoint::new(sdl.xrel, sdl.yrel),
                    }
                }
            },
            SDL_EventType::MOUSE_BUTTON_DOWN | SDL_EventType::MOUSE_BUTTON_UP => {
                let sdl = &sdl.button;
                let down = sdl.down;
                let user_operation = if down {
                    UserOperation::Down
                } else {
                    UserOperation::Up
                };

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::MouseButton {
                        window_id: sdl.windowID.0.into(),
                        mouse_id: sdl.which.0.into(),
                        button: button_from_index(sdl.button),
                        down,
                        clicks: sdl.clicks,
                        position: FPoint::new(sdl.x, sdl.y),
                        user_operation,
                    }
                }
            },
            SDL_EventType::MOUSE_WHEEL => {
                let sdl = &sdl.wheel;
                let direction = MouseWheelDirection::from_i32(sdl.direction.0)
                    .unwrap_or(MouseWheelDirection::Normal);

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::MouseWheel {
                        window_id: sdl.windowID.0.into(),
                        mouse_id: sdl.which.0.into(),
                        scroll: FPoint::new(sdl.x, sdl.y),
                        direction,
                        position: FPoint::new(sdl.mouse_x, sdl.mouse_y),
                        accumulated_scroll: Point::new(sdl.integer_x, sdl.integer_y),
                    }
                }
            },
            SDL_EventType::WINDOW_CLOSE_REQUESTED => {
                let sdl = &sdl.window;

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::WindowClose { id: sdl.windowID.0.into() }
                }
            },
            SDL_EventType::WINDOW_SHOWN
            | SDL_EventType::WINDOW_HIDDEN
            | SDL_EventType::WINDOW_EXPOSED
            | SDL_EventType::WINDOW_MOVED
            | SDL_EventType::WINDOW_RESIZED
            | SDL_EventType::WINDOW_PIXEL_SIZE_CHANGED
            | SDL_EventType::WINDOW_METAL_VIEW_RESIZED
            | SDL_EventType::WINDOW_MINIMIZED
            | SDL_EventType::WINDOW_MAXIMIZED
            | SDL_EventType::WINDOW_RESTORED
            | SDL_EventType::WINDOW_MOUSE_ENTER
            | SDL_EventType::WINDOW_MOUSE_LEAVE
            | SDL_EventType::WINDOW_FOCUS_GAINED
            | SDL_EventType::WINDOW_FOCUS_LOST
            | SDL_EventType::WINDOW_HIT_TEST
            | SDL_EventType::WINDOW_ICCPROF_CHANGED
            | SDL_EventType::WINDOW_DISPLAY_CHANGED
            | SDL_EventType::WINDOW_DISPLAY_SCALE_CHANGED
            | SDL_EventType::WINDOW_SAFE_AREA_CHANGED
            | SDL_EventType::WINDOW_OCCLUDED
            | SDL_EventType::WINDOW_ENTER_FULLSCREEN
            | SDL_EventType::WINDOW_LEAVE_FULLSCREEN
            | SDL_EventType::WINDOW_DESTROYED
            | SDL_EventType::WINDOW_HDR_STATE_CHANGED => {
                let sdl = &sdl.window;
                let event = match sdl_type {
                    SDL_EventType::WINDOW_SHOWN => WindowEvent::Shown,
                    SDL_EventType::WINDOW_HIDDEN => WindowEvent::Hidden,
                    SDL_EventType::WINDOW_EXPOSED => WindowEvent::Exposed { is_live_resize: sdl.data1 != 0 },
                    SDL_EventType::WINDOW_MOVED => WindowEvent::Moved { position: Point::new(sdl.data1, sdl.data2) },
                    SDL_EventType::WINDOW_RESIZED => WindowEvent::Resized { size: Point::new(sdl.data1, sdl.data2) },
                    SDL_EventType::WINDOW_PIXEL_SIZE_CHANGED => WindowEvent::PixelSizeChanged { size: Point::new(sdl.data1, sdl.data2) },
                    SDL_EventType::WINDOW_METAL_VIEW_RESIZED => WindowEvent::MetalViewResized,
                    SDL_EventType::WINDOW_MINIMIZED => WindowEvent::Minimized,
                    SDL_EventType::WINDOW_MAXIMIZED => WindowEvent::Maximized,
                    SDL_EventType::WINDOW_RESTORED => WindowEvent::Restored,
                    SDL_EventType::WINDOW_MOUSE_ENTER => WindowEvent::MouseEnter,
                    SDL_EventType::WINDOW_MOUSE_LEAVE => WindowEvent::MouseLeave,
                    SDL_EventType::WINDOW_FOCUS_GAINED => WindowEvent::FocusGained,
                    SDL_EventType::WINDOW_FOCUS_LOST => WindowEvent::FocusLost,
                    SDL_EventType::WINDOW_HIT_TEST => WindowEvent::HitTest,
                    SDL_EventType::WINDOW_ICCPROF_CHANGED => WindowEvent::IccProfileChanged,
                    SDL_EventType::WINDOW_DISPLAY_CHANGED => WindowEvent::DisplayChanged { display_id: sdl.data1 as u32 },
                    SDL_EventType::WINDOW_DISPLAY_SCALE_CHANGED => WindowEvent::DisplayScaleChanged,
                    SDL_EventType::WINDOW_SAFE_AREA_CHANGED => WindowEvent::SafeAreaChanged,
                    SDL_EventType::WINDOW_OCCLUDED => WindowEvent::Occluded,
                    SDL_EventType::WINDOW_ENTER_FULLSCREEN => WindowEvent::EnterFullscreen,
                    SDL_EventType::WINDOW_LEAVE_FULLSCREEN => WindowEvent::LeaveFullscreen,
                    SDL_EventType::WINDOW_DESTROYED => WindowEvent::Destroyed,
                    SDL_EventType::WINDOW_HDR_STATE_CHANGED => WindowEvent::HdrStateChanged,
                    _ => WindowEvent::Shown,
                };

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::Window {
                        window_id: sdl.windowID.0.into(),
                        event,
                    }
                }
            },
            _ => {
                let sdl = &sdl.common;

                Event{
                    ns_timestamp: sdl.timestamp,
                    raw: RawEvent::Unknown { type_id: sdl.r#type.into() }
                }
            },
        }
    }
}
