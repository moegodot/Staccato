use num_derive::{FromPrimitive, ToPrimitive};

pub const EXTENDED_MASK: u32 = 1 << 29;

pub const SCANCODE_MASK: u32 = 1 << 30;

/// 将扫描码转换为键码的辅助函数
pub const fn scancode_to_keycode(scancode: u32) -> u32 {
    scancode | SCANCODE_MASK
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash,FromPrimitive, ToPrimitive)]
pub enum KeyCode {
    /// 0
    Unknown = 0x00000000,
    /// '\r'
    Return = 0x0000000d,
    /// '\x1B'
    Escape = 0x0000001b,
    /// '\b'
    Backspace = 0x00000008,
    /// '\t'
    Tab = 0x00000009,
    /// ' '
    Space = 0x00000020,
    /// '!'
    Exclaim = 0x00000021,
    /// '"'
    DoubleApostrophe = 0x00000022,
    /// '#'
    Hash = 0x00000023,
    /// '$'
    Dollar = 0x00000024,
    /// '%'
    Percent = 0x00000025,
    /// '&'
    Ampersand = 0x00000026,
    /// '\''
    Apostrophe = 0x00000027,
    /// '('
    LeftParen = 0x00000028,
    /// ')'
    RightParen = 0x00000029,
    /// '*'
    Asterisk = 0x0000002a,
    /// '+'
    Plus = 0x0000002b,
    /// ','
    Comma = 0x0000002c,
    /// '-'
    Minus = 0x0000002d,
    /// '.'
    Period = 0x0000002e,
    /// '/'
    Slash = 0x0000002f,
    /// '0'
    Num0 = 0x00000030,
    /// '1'
    Num1 = 0x00000031,
    /// '2'
    Num2 = 0x00000032,
    /// '3'
    Num3 = 0x00000033,
    /// '4'
    Num4 = 0x00000034,
    /// '5'
    Num5 = 0x00000035,
    /// '6'
    Num6 = 0x00000036,
    /// '7'
    Num7 = 0x00000037,
    /// '8'
    Num8 = 0x00000038,
    /// '9'
    Num9 = 0x00000039,
    /// ':'
    Colon = 0x0000003a,
    /// ';'
    Semicolon = 0x0000003b,
    /// '<'
    Less = 0x0000003c,
    /// '='
    Equals = 0x0000003d,
    /// '>'
    Greater = 0x0000003e,
    /// '?'
    Question = 0x0000003f,
    /// '@'
    At = 0x00000040,
    /// '['
    LeftBracket = 0x0000005b,
    /// '\\'
    Backslash = 0x0000005c,
    /// ']'
    RightBracket = 0x0000005d,
    /// '^'
    Caret = 0x0000005e,
    /// '_'
    Underscore = 0x0000005f,
    /// '`'
    Grave = 0x00000060,
    /// 'a'
    A = 0x00000061,
    /// 'b'
    B = 0x00000062,
    /// 'c'
    C = 0x00000063,
    /// 'd'
    D = 0x00000064,
    /// 'e'
    E = 0x00000065,
    /// 'f'
    F = 0x00000066,
    /// 'g'
    G = 0x00000067,
    /// 'h'
    H = 0x00000068,
    /// 'i'
    I = 0x00000069,
    /// 'j'
    J = 0x0000006a,
    /// 'k'
    K = 0x0000006b,
    /// 'l'
    L = 0x0000006c,
    /// 'm'
    M = 0x0000006d,
    /// 'n'
    N = 0x0000006e,
    /// 'o'
    O = 0x0000006f,
    /// 'p'
    P = 0x00000070,
    /// 'q'
    Q = 0x00000071,
    /// 'r'
    R = 0x00000072,
    /// 's'
    S = 0x00000073,
    /// 't'
    T = 0x00000074,
    /// 'u'
    U = 0x00000075,
    /// 'v'
    V = 0x00000076,
    /// 'w'
    W = 0x00000077,
    /// 'x'
    X = 0x00000078,
    /// 'y'
    Y = 0x00000079,
    /// 'z'
    Z = 0x0000007a,
    /// '{'
    LeftBrace = 0x0000007b,
    /// '|'
    Pipe = 0x0000007c,
    /// '}'
    RightBrace = 0x0000007d,
    /// '~'
    Tilde = 0x0000007e,
    /// '\x7F'
    Delete = 0x0000007f,
    /// '\xB1'
    PlusMinus = 0x000000b1,

    /* Scancode-based keycodes */
    CapsLock = 0x40000039,
    F1 = 0x4000003a,
    F2 = 0x4000003b,
    F3 = 0x4000003c,
    F4 = 0x4000003d,
    F5 = 0x4000003e,
    F6 = 0x4000003f,
    F7 = 0x40000040,
    F8 = 0x40000041,
    F9 = 0x40000042,
    F10 = 0x40000043,
    F11 = 0x40000044,
    F12 = 0x40000045,
    PrintScreen = 0x40000046,
    ScrollLock = 0x40000047,
    Pause = 0x40000048,
    Insert = 0x40000049,
    Home = 0x4000004a,
    PageUp = 0x4000004b,
    End = 0x4000004d,
    PageDown = 0x4000004e,
    Right = 0x4000004f,
    Left = 0x40000050,
    Down = 0x40000051,
    Up = 0x40000052,
    NumLockClear = 0x40000053,
    KpDivide = 0x40000054,
    KpMultiply = 0x40000055,
    KpMinus = 0x40000056,
    KpPlus = 0x40000057,
    KpEnter = 0x40000058,
    Kp1 = 0x40000059,
    Kp2 = 0x4000005a,
    Kp3 = 0x4000005b,
    Kp4 = 0x4000005c,
    Kp5 = 0x4000005d,
    Kp6 = 0x4000005e,
    Kp7 = 0x4000005f,
    Kp8 = 0x40000060,
    Kp9 = 0x40000061,
    Kp0 = 0x40000062,
    KpPeriod = 0x40000063,
    Application = 0x40000065,
    Power = 0x40000066,
    KpEquals = 0x40000067,
    F13 = 0x40000068,
    F14 = 0x40000069,
    F15 = 0x4000006a,
    F16 = 0x4000006b,
    F17 = 0x4000006c,
    F18 = 0x4000006d,
    F19 = 0x4000006e,
    F20 = 0x4000006f,
    F21 = 0x40000070,
    F22 = 0x40000071,
    F23 = 0x40000072,
    F24 = 0x40000073,
    Execute = 0x40000074,
    Help = 0x40000075,
    Menu = 0x40000076,
    Select = 0x40000077,
    Stop = 0x40000078,
    Again = 0x40000079,
    Undo = 0x4000007a,
    Cut = 0x4000007b,
    Copy = 0x4000007c,
    Paste = 0x4000007d,
    Find = 0x4000007e,
    Mute = 0x4000007f,
    VolumeUp = 0x40000080,
    VolumeDown = 0x40000081,
    KpComma = 0x40000085,
    KpEqualsAs400 = 0x40000086,
    AltErase = 0x40000099,
    SysReq = 0x4000009a,
    Cancel = 0x4000009b,
    Clear = 0x4000009c,
    Prior = 0x4000009d,
    Return2 = 0x4000009e,
    Separator = 0x4000009f,
    Out = 0x400000a0,
    Oper = 0x400000a1,
    ClearAgain = 0x400000a2,
    CrSel = 0x400000a3,
    ExSel = 0x400000a4,
    Kp00 = 0x400000b0,
    Kp000 = 0x400000b1,
    ThousandsSeparator = 0x400000b2,
    DecimalSeparator = 0x400000b3,
    CurrencyUnit = 0x400000b4,
    CurrencySubUnit = 0x400000b5,
    KpLeftParen = 0x400000b6,
    KpRightParen = 0x400000b7,
    KpLeftBrace = 0x400000b8,
    KpRightBrace = 0x400000b9,
    KpTab = 0x400000ba,
    KpBackspace = 0x400000bb,
    KpA = 0x400000bc,
    KpB = 0x400000bd,
    KpC = 0x400000be,
    KpD = 0x400000bf,
    KpE = 0x400000c0,
    KpF = 0x400000c1,
    KpXor = 0x400000c2,
    KpPower = 0x400000c3,
    KpPercent = 0x400000c4,
    KpLess = 0x400000c5,
    KpGreater = 0x400000c6,
    KpAmpersand = 0x400000c7,
    KpDoubleAmpersand = 0x400000c8,
    KpVerticalBar = 0x400000c9,
    KpDoubleVerticalBar = 0x400000ca,
    KpColon = 0x400000cb,
    KpHash = 0x400000cc,
    KpSpace = 0x400000cd,
    KpAt = 0x400000ce,
    KpExclam = 0x400000cf,
    KpMemStore = 0x400000d0,
    KpMemRecall = 0x400000d1,
    KpMemClear = 0x400000d2,
    KpMemAdd = 0x400000d3,
    KpMemSubtract = 0x400000d4,
    KpMemMultiply = 0x400000d5,
    KpMemDivide = 0x400000d6,
    KpPlusMinus = 0x400000d7,
    KpClear = 0x400000d8,
    KpClearEntry = 0x400000d9,
    KpBinary = 0x400000da,
    KpOctal = 0x400000db,
    KpDecimal = 0x400000dc,
    KpHexadecimal = 0x400000dd,
    LCtrl = 0x400000e0,
    LShift = 0x400000e1,
    LAlt = 0x400000e2,
    LGui = 0x400000e3,
    RCtrl = 0x400000e4,
    RShift = 0x400000e5,
    RAlt = 0x400000e6,
    RGui = 0x400000e7,
    Mode = 0x40000101,
    Sleep = 0x40000102,
    Wake = 0x40000103,
    ChannelIncrement = 0x40000104,
    ChannelDecrement = 0x40000105,
    MediaPlay = 0x40000106,
    MediaPause = 0x40000107,
    MediaRecord = 0x40000108,
    MediaFastForward = 0x40000109,
    MediaRewind = 0x4000010a,
    MediaNextTrack = 0x4000010b,
    MediaPreviousTrack = 0x4000010c,
    MediaStop = 0x4000010d,
    MediaEject = 0x4000010e,
    MediaPlayPause = 0x4000010f,
    MediaSelect = 0x40000110,
    AcNew = 0x40000111,
    AcOpen = 0x40000112,
    AcClose = 0x40000113,
    AcExit = 0x40000114,
    AcSave = 0x40000115,
    AcPrint = 0x40000116,
    AcProperties = 0x40000117,
    AcSearch = 0x40000118,
    AcHome = 0x40000119,
    AcBack = 0x4000011a,
    AcForward = 0x4000011b,
    AcStop = 0x4000011c,
    AcRefresh = 0x4000011d,
    AcBookmarks = 0x4000011e,
    SoftLeft = 0x4000011f,
    SoftRight = 0x40000120,
    Call = 0x40000121,
    EndCall = 0x40000122,

    /* Extended keys */
    /// Extended key Left Tab
    LeftTab = 0x20000001,
    /// Extended key Level 5 Shift
    Level5Shift = 0x20000002,
    /// Extended key Multi-key Compose
    MultiKeyCompose = 0x20000003,
    /// Extended key Left Meta
    LMeta = 0x20000004,
    /// Extended key Right Meta
    RMeta = 0x20000005,
    /// Extended key Left Hyper
    LHyper = 0x20000006,
    /// Extended key Right Hyper
    RHyper = 0x20000007,
}

impl KeyCode {
    /// Check if this keycode is scancode-based.
    pub fn is_scancode(&self) -> bool {
        (*self as u32 & SCANCODE_MASK) != 0
    }

    /// Check if this keycode is extent keycode.
    pub fn is_extended(&self) -> bool {
        (*self as u32 & EXTENDED_MASK) != 0
    }
}

impl From<KeyCode> for u32 {
    fn from(code: KeyCode) -> Self {
        code as u32
    }
}
