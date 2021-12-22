use num_derive::{FromPrimitive, ToPrimitive};

pub struct MouseClick {
    button: MouseButton,
    pos: (i16, i16),
}

impl MouseClick {
    pub fn new(button: MouseButton, pos: (i16, i16)) -> Self {
        Self { button, pos }
    }

    pub fn button(&self) -> MouseButton {
        self.button.clone()
    }

    pub fn pos(&self) -> (i16, i16) {
        self.pos
    }
}

#[derive(Debug, Clone, PartialEq, std::cmp::Eq, Hash)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScrollDirection {
    Up,
    Down,
}

pub struct Key {
    code: KeyCode,
    character: char,
}

impl Key {
    pub fn new(code: KeyCode, character: char) -> Self {
        Self { code, character }
    }

    pub fn code(&self) -> KeyCode {
        self.code.clone()
    }

    pub fn character(&self) -> char {
        self.character
    }
}

#[derive(Debug, Clone, PartialEq, std::cmp::Eq, Hash, FromPrimitive, ToPrimitive)]
pub enum KeyCode {
    Empty,
    Esc,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    K0,
    Hyphen,
    Equals,
    Backspace,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    LeftBracket,
    RightBracket,
    Enter,
    LeftCtrl,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Semicolon,
    SingleQuote,
    Grave,
    LeftShift,
    Backslash,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Dot,
    Slash,
    RightShift,
    PrintScreen,
    LeftAlt,
    Space,
    CapsLock,
}
