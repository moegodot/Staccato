use std::convert::TryFrom;
use num_derive::{FromPrimitive, ToPrimitive};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash,FromPrimitive, ToPrimitive)]
pub enum Scancode {
    Unknown = 0,

    // --- Usage page 0x07 (USB keyboard page) ---

    A = 4,
    B = 5,
    C = 6,
    D = 7,
    E = 8,
    F = 9,
    G = 10,
    H = 11,
    I = 12,
    J = 13,
    K = 14,
    L = 15,
    M = 16,
    N = 17,
    O = 18,
    P = 19,
    Q = 20,
    R = 21,
    S = 22,
    T = 23,
    U = 24,
    V = 25,
    W = 26,
    X = 27,
    Y = 28,
    Z = 29,

    Num1 = 30,
    Num2 = 31,
    Num3 = 32,
    Num4 = 33,
    Num5 = 34,
    Num6 = 35,
    Num7 = 36,
    Num8 = 37,
    Num9 = 38,
    Num0 = 39,

    Return = 40,
    Escape = 41,
    Backspace = 42,
    Tab = 43,
    Space = 44,

    Minus = 45,
    Equals = 46,
    LeftBracket = 47,
    RightBracket = 48,
    /// Located at the lower left of the return key on ISO keyboards and at the right end
    /// of the QWERTY row on ANSI keyboards.
    Backslash = 49,
    /// ISO USB keyboards actually use this code instead of 49 for the same key, but all
    /// OSes I've seen treat the two codes identically.
    NonUsHash = 50,
    Semicolon = 51,
    Apostrophe = 52,
    /// Located in the top left corner (on both ANSI and ISO keyboards).
    Grave = 53,
    Comma = 54,
    Period = 55,
    Slash = 56,

    CapsLock = 57,

    F1 = 58,
    F2 = 59,
    F3 = 60,
    F4 = 61,
    F5 = 62,
    F6 = 63,
    F7 = 64,
    F8 = 65,
    F9 = 66,
    F10 = 67,
    F11 = 68,
    F12 = 69,

    PrintScreen = 70,
    ScrollLock = 71,
    Pause = 72,
    /// insert on PC, help on some Mac keyboards (but does send code 73, not 117)
    Insert = 73,
    Home = 74,
    PageUp = 75,
    Delete = 76,
    End = 77,
    PageDown = 78,
    Right = 79,
    Left = 80,
    Down = 81,
    Up = 82,

    /// num lock on PC, clear on Mac keyboards
    NumLockClear = 83,
    KpDivide = 84,
    KpMultiply = 85,
    KpMinus = 86,
    KpPlus = 87,
    KpEnter = 88,
    Kp1 = 89,
    Kp2 = 90,
    Kp3 = 91,
    Kp4 = 92,
    Kp5 = 93,
    Kp6 = 94,
    Kp7 = 95,
    Kp8 = 96,
    Kp9 = 97,
    Kp0 = 98,
    KpPeriod = 99,

    /// This is the additional key that ISO keyboards have over ANSI ones,
    /// located between left shift and Z.
    NonUsBackslash = 100,
    /// windows contextual menu, compose
    Application = 101,
    /// The USB document says this is a status flag, not a physical key -
    /// but some Mac keyboards do have a power key.
    Power = 102,
    KpEquals = 103,
    F13 = 104,
    F14 = 105,
    F15 = 106,
    F16 = 107,
    F17 = 108,
    F18 = 109,
    F19 = 110,
    F20 = 111,
    F21 = 112,
    F22 = 113,
    F23 = 114,
    F24 = 115,
    Execute = 116,
    /// AL Integrated Help Center
    Help = 117,
    /// Menu (show menu)
    Menu = 118,
    Select = 119,
    /// AC Stop
    Stop = 120,
    /// AC Redo/Repeat
    Again = 121,
    /// AC Undo
    Undo = 122,
    /// AC Cut
    Cut = 123,
    /// AC Copy
    Copy = 124,
    /// AC Paste
    Paste = 125,
    /// AC Find
    Find = 126,
    Mute = 127,
    VolumeUp = 128,
    VolumeDown = 129,
    KpComma = 133,
    KpEqualsAs400 = 134,

    /// used on Asian keyboards, see footnotes in USB doc
    International1 = 135,
    International2 = 136,
    /// Yen
    International3 = 137,
    International4 = 138,
    International5 = 139,
    International6 = 140,
    International7 = 141,
    International8 = 142,
    International9 = 143,
    /// Hangul/English toggle
    Lang1 = 144,
    /// Hanja conversion
    Lang2 = 145,
    /// Katakana
    Lang3 = 146,
    /// Hiragana
    Lang4 = 147,
    /// Zenkaku/Hankaku
    Lang5 = 148,
    /// reserved
    Lang6 = 149,
    /// reserved
    Lang7 = 150,
    /// reserved
    Lang8 = 151,
    /// reserved
    Lang9 = 152,

    /// Erase-Eaze
    AltErase = 153,
    SysReq = 154,
    /// AC Cancel
    Cancel = 155,
    Clear = 156,
    Prior = 157,
    Return2 = 158,
    Separator = 159,
    Out = 160,
    Oper = 161,
    ClearAgain = 162,
    CrSel = 163,
    ExSel = 164,

    Kp00 = 176,
    Kp000 = 177,
    ThousandsSeparator = 178,
    DecimalSeparator = 179,
    CurrencyUnit = 180,
    CurrencySubunit = 181,
    KpLeftParen = 182,
    KpRightParen = 183,
    KpLeftBrace = 184,
    KpRightBrace = 185,
    KpTab = 186,
    KpBackspace = 187,
    KpA = 188,
    KpB = 189,
    KpC = 190,
    KpD = 191,
    KpE = 192,
    KpF = 193,
    KpXor = 194,
    KpPower = 195,
    KpPercent = 196,
    KpLess = 197,
    KpGreater = 198,
    KpAmpersand = 199,
    KpDblAmpersand = 200,
    KpVerticalBar = 201,
    KpDblVerticalBar = 202,
    KpColon = 203,
    KpHash = 204,
    KpSpace = 205,
    KpAt = 206,
    KpExclam = 207,
    KpMemStore = 208,
    KpMemRecall = 209,
    KpMemClear = 210,
    KpMemAdd = 211,
    KpMemSubtract = 212,
    KpMemMultiply = 213,
    KpMemDivide = 214,
    KpPlusMinus = 215,
    KpClear = 216,
    KpClearEntry = 217,
    KpBinary = 218,
    KpOctal = 219,
    KpDecimal = 220,
    KpHexadecimal = 221,

    LCtrl = 224,
    LShift = 225,
    /// alt, option
    LAlt = 226,
    /// windows, command (apple), meta
    LGui = 227,
    RCtrl = 228,
    RShift = 229,
    /// alt gr, option
    RAlt = 230,
    /// windows, command (apple), meta
    RGui = 231,

    /// I'm not sure if this is really not covered by any of the above
    Mode = 257,

    // --- Usage page 0x0C (USB consumer page) ---

    /// Sleep
    Sleep = 258,
    /// Wake
    Wake = 259,

    /// Channel Increment
    ChannelIncrement = 260,
    /// Channel Decrement
    ChannelDecrement = 261,

    /// Play
    MediaPlay = 262,
    /// Pause
    MediaPause = 263,
    /// Record
    MediaRecord = 264,
    /// Fast Forward
    MediaFastForward = 265,
    /// Rewind
    MediaRewind = 266,
    /// Next Track
    MediaNextTrack = 267,
    /// Previous Track
    MediaPreviousTrack = 268,
    /// Stop
    MediaStop = 269,
    /// Eject
    MediaEject = 270,
    /// Play / Pause
    MediaPlayPause = 271,
    /// Media Select
    MediaSelect = 272,

    /// AC New
    AcNew = 273,
    /// AC Open
    AcOpen = 274,
    /// AC Close
    AcClose = 275,
    /// AC Exit
    AcExit = 276,
    /// AC Save
    AcSave = 277,
    /// AC Print
    AcPrint = 278,
    /// AC Properties
    AcProperties = 279,

    /// AC Search
    AcSearch = 280,
    /// AC Home
    AcHome = 281,
    /// AC Back
    AcBack = 282,
    /// AC Forward
    AcForward = 283,
    /// AC Stop
    AcStop = 284,
    /// AC Refresh
    AcRefresh = 285,
    /// AC Bookmarks
    AcBookmarks = 286,

    // --- Mobile keys ---

    /// Usually situated below the display on phones and used as a multi-function feature key
    SoftLeft = 287,
    /// Usually situated below the display on phones and used as a multi-function feature key
    SoftRight = 288,
    /// Used for accepting phone calls.
    Call = 289,
    /// Used for rejecting phone calls.
    EndCall = 290,

    /// 400-500 reserved for dynamic keycodes
    Reserved = 400,
}

impl Scancode {
    /// 返回 Scancodes 的总数（对应 SDL_SCANCODE_COUNT）
    pub const COUNT: usize = 512;
}
