use staccato_core::frect::FPoint;
use staccato_core::keycode::KeyCode;
use staccato_core::keymod::Keymod;
use staccato_core::mouse::Button;
use staccato_core::rect::Point;
use staccato_core::scancode::Scancode;
use crate::id::{KeyboardId, MouseId, WindowId};

#[derive(Clone, Debug)]
pub enum DeviceOperation {
    /// device(e.g. mouse or keyboard) was added
    Added,
    /// device(e.g. mouse or keyboard) was removed
    Removed
}

#[derive(Clone, Debug)]
pub enum UserOperation {
    /// Key up
    Up,
    /// Key down
    Down
}

#[derive(Clone, Debug)]
pub enum RawEvent {
    Quit,
    WindowClose { id:WindowId },
    Keyboard{
        window_id: WindowId,
        keyboard_id:KeyboardId,
        scan_code: Scancode,
        key_code: KeyCode,
        keymod: Keymod,
        is_down:bool,
        is_repeat:bool,
        user_operation: UserOperation
    },
    KeyboardDevice{
        keyboard_id:KeyboardId,
        operation: DeviceOperation,
    },
    MouseDevice{
        mouse_id:MouseId,
        operation: DeviceOperation,
    },
    MouseButton{
        window_id:WindowId,
        mouse_id:MouseId,
        button:Button,
        down:bool,
        position:FPoint,
        user_operation: UserOperation
    },
    MouseWheel{
        window_id:WindowId,
        mouse_id:MouseId,
        button:Button,
        down:bool,
        position:FPoint,
        scroll:FPoint,
        accumulated_scroll:Point
    }
}

#[derive(Clone, Debug)]
pub struct Event{
    pub ns_timestamp:u64,
    pub event:RawEvent
}
