use staccato_core::keycode::KeyCode;
use staccato_core::keymod::Keymod;
use staccato_core::scancode::Scancode;
use crate::id::{KeyboardId, WindowId};

pub enum Event{
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
    }
}
