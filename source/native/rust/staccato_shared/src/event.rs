use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;
use staccato_core::frect::FPoint;
use staccato_core::keycode::KeyCode;
use staccato_core::keymod::Keymod;
use staccato_core::mouse::{Button, MouseWheelDirection};
use staccato_core::rect::Point;
use staccato_core::scancode::Scancode;
use crate::id::{KeyboardId, MouseId, WindowId};

pub const INLINE_TEXT_MAX: usize = 64;

#[derive(Clone, Debug)]
pub struct InlineText {
    pub len: u8,
    pub data: [u8; INLINE_TEXT_MAX],
}

impl InlineText {
    pub fn empty() -> Self {
        Self {
            len: 0,
            data: [0; INLINE_TEXT_MAX],
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut text = Self::empty();
        let len = bytes.len().min(INLINE_TEXT_MAX);
        text.data[..len].copy_from_slice(&bytes[..len]);
        text.len = len as u8;
        text
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.len as usize]
    }
}

impl Default for InlineText {
    fn default() -> Self {
        Self::empty()
    }
}

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
pub enum AppEvent {
    Terminating,
    LowMemory,
    WillEnterBackground,
    DidEnterBackground,
    WillEnterForeground,
    DidEnterForeground,
    LocaleChanged,
    SystemThemeChanged,
}

#[derive(Clone, Debug)]
pub enum WindowEvent {
    Shown,
    Hidden,
    Exposed { is_live_resize: bool },
    Moved { position: Point },
    Resized { size: Point },
    PixelSizeChanged { size: Point },
    MetalViewResized,
    Minimized,
    Maximized,
    Restored,
    MouseEnter,
    MouseLeave,
    FocusGained,
    FocusLost,
    HitTest,
    IccProfileChanged,
    DisplayChanged { display_id: u32 },
    DisplayScaleChanged,
    SafeAreaChanged,
    Occluded,
    EnterFullscreen,
    LeaveFullscreen,
    Destroyed,
    HdrStateChanged,
}

#[derive(Clone, Debug)]
pub enum RawEvent {
    Quit,
    App { event: AppEvent },
    Window { window_id: WindowId, event: WindowEvent },
    WindowClose { id: WindowId },
    KeymapChanged,
    Keyboard {
        window_id: WindowId,
        keyboard_id: KeyboardId,
        scan_code: Scancode,
        key_code: KeyCode,
        keymod: Keymod,
        raw_scancode: u16,
        is_down: bool,
        is_repeat: bool,
        user_operation: UserOperation,
    },
    TextEditing {
        window_id: WindowId,
        text: InlineText,
        start: i32,
        length: i32,
    },
    TextInput {
        window_id: WindowId,
        text: InlineText,
    },
    KeyboardDevice {
        keyboard_id: KeyboardId,
        operation: DeviceOperation,
    },
    MouseDevice {
        mouse_id: MouseId,
        operation: DeviceOperation,
    },
    MouseMotion {
        window_id: WindowId,
        mouse_id: MouseId,
        state: Button,
        position: FPoint,
        relative: FPoint,
    },
    MouseButton {
        window_id: WindowId,
        mouse_id: MouseId,
        button: Button,
        down: bool,
        clicks: u8,
        position: FPoint,
        user_operation: UserOperation,
    },
    MouseWheel {
        window_id: WindowId,
        mouse_id: MouseId,
        scroll: FPoint,
        direction: MouseWheelDirection,
        position: FPoint,
        accumulated_scroll: Point,
    },
    Unknown {
        type_id: u64,
    },
    //Custom(Arc<dyn CustomEvent>)
}

/*
pub trait CustomEvent : std::any::Any + Debug{
    fn event_name(&self) -> smol_str::SmolStr;
}
*/

#[derive(Clone, Debug)]
pub struct Event{
    pub ns_timestamp: u64,
    pub raw: RawEvent,
}
