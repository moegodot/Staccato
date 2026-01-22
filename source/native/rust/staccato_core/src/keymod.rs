use bitflags::bitflags;
use num_derive::{FromPrimitive, ToPrimitive};

bitflags! {
    /// 按键修饰符状态的位掩码。
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Keymod: u16 {
        /// 无修饰符。
        const NONE = 0x0000;
        /// 左 Shift 键被按下。
        const LSHIFT = 0x0001;
        /// 右 Shift 键被按下。
        const RSHIFT = 0x0002;
        /// Level 5 Shift 键被按下。
        const LEVEL5 = 0x0004;
        /// 左 Ctrl (Control) 键被按下。
        const LCTRL = 0x0040;
        /// 右 Ctrl (Control) 键被按下。
        const RCTRL = 0x0080;
        /// 左 Alt 键被按下。
        const LALT = 0x0100;
        /// 右 Alt 键被按下。
        const RALT = 0x0200;
        /// 左 GUI 键（通常是 Windows 键）被按下。
        const LGUI = 0x0400;
        /// 右 GUI 键（通常是 Windows 键）被按下。
        const RGUI = 0x0800;
        /// Num Lock 键（通常在扩展键盘上）已激活。
        const NUM = 0x1000;
        /// Caps Lock 键已激活。
        const CAPS = 0x2000;
        /// AltGr 键被按下。
        const MODE = 0x4000;
        /// Scroll Lock 键已激活。
        const SCROLL = 0x8000;

        /// 任意 Ctrl 键被按下。
        const CTRL = Self::LCTRL.bits() | Self::RCTRL.bits();
        /// 任意 Shift 键被按下。
        const SHIFT = Self::LSHIFT.bits() | Self::RSHIFT.bits();
        /// 任意 Alt 键被按下。
        const ALT = Self::LALT.bits() | Self::RALT.bits();
        /// 任意 GUI 键被按下。
        const GUI = Self::LGUI.bits() | Self::RGUI.bits();
    }
}

impl Keymod {
    /// 检查是否按下了任意 Shift 键。
    pub fn is_shift(self) -> bool {
        self.intersects(Self::SHIFT)
    }

    /// 检查是否按下了任意 Ctrl 键。
    pub fn is_ctrl(self) -> bool {
        self.intersects(Self::CTRL)
    }

    /// 检查是否按下了任意 Alt 键。
    pub fn is_alt(self) -> bool {
        self.intersects(Self::ALT)
    }

    /// 检查是否按下了任意 GUI 键。
    pub fn is_gui(self) -> bool {
        self.intersects(Self::GUI)
    }
}
