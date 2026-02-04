use bitflags::bitflags;
use num_derive::{FromPrimitive, ToPrimitive};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ToPrimitive, FromPrimitive)]
pub enum UnmarkedButton {
    Left = 1,
    Middle = 2,
    Right = 3,
    Side1 = 4,
    Side2 = 5,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Button: u32 {
        const Left = 0b00000001;
        const Middle = 0b00000010;
        const Right = 0b00000100;
        const Side1 = 0b00001000;
        const Side2 = 0b00010000;
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ToPrimitive, FromPrimitive)]
pub enum MouseWheelDirection {
    Normal,
    Flipped,
}
