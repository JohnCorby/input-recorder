use device_query::{Keycode, MouseState};
use enigo::{Key, MouseButton};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// holds all kb and mouse buttons
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Button {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Escape,
    Space,
    LControl,
    RControl,
    LShift,
    RShift,
    LAlt,
    RAlt,
    Meta,
    Enter,
    Up,
    Down,
    Left,
    Right,
    Backspace,
    CapsLock,
    Tab,
    Home,
    End,
    PageUp,
    PageDown,
    Insert,
    Delete,

    // The following keys have not been tested on MacOS!
    // Numpad keys which have not been implemented: NumpadSeparator NumLock
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadSubtract,
    NumpadAdd,
    NumpadDivide,
    NumpadMultiply,

    // The following keys names represent the position of the key in a US keyboard,
    // not the sign value. In a different keyboards and OS, the position can vary.
    Grave,
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    BackSlash,
    Semicolon,
    Apostrophe,
    Comma,
    Dot,
    Slash,

    Mouse1,
    Mouse2,
    Mouse3,
    Mouse4,
    Mouse5,
}

impl From<Keycode> for Button {
    fn from(key: Keycode) -> Self {
        use Keycode::*;
        match key {
            Key0 => Self::Key0,
            Key1 => Self::Key1,
            Key2 => Self::Key2,
            Key3 => Self::Key3,
            Key4 => Self::Key4,
            Key5 => Self::Key5,
            Key6 => Self::Key6,
            Key7 => Self::Key7,
            Key8 => Self::Key8,
            Key9 => Self::Key9,
            A => Self::A,
            B => Self::B,
            C => Self::C,
            D => Self::D,
            E => Self::E,
            F => Self::F,
            G => Self::G,
            H => Self::H,
            I => Self::I,
            J => Self::J,
            K => Self::K,
            L => Self::L,
            M => Self::M,
            N => Self::N,
            O => Self::O,
            P => Self::P,
            Q => Self::Q,
            R => Self::R,
            S => Self::S,
            T => Self::T,
            U => Self::U,
            V => Self::V,
            W => Self::W,
            X => Self::X,
            Y => Self::Y,
            Z => Self::Z,
            F1 => Self::F1,
            F2 => Self::F2,
            F3 => Self::F3,
            F4 => Self::F4,
            F5 => Self::F5,
            F6 => Self::F6,
            F7 => Self::F7,
            F8 => Self::F8,
            F9 => Self::F9,
            F10 => Self::F10,
            F11 => Self::F11,
            F12 => Self::F12,
            Escape => Self::Escape,
            Space => Self::Space,
            LControl => Self::LControl,
            RControl => Self::RControl,
            LShift => Self::LShift,
            RShift => Self::RShift,
            LAlt => Self::LAlt,
            RAlt => Self::RAlt,
            Meta => Self::Meta,
            Enter => Self::Enter,
            Up => Self::Up,
            Down => Self::Down,
            Left => Self::Left,
            Right => Self::Right,
            Backspace => Self::Backspace,
            CapsLock => Self::CapsLock,
            Tab => Self::Tab,
            Home => Self::Home,
            End => Self::End,
            PageUp => Self::PageUp,
            PageDown => Self::PageDown,
            Insert => Self::Insert,
            Delete => Self::Delete,
            Numpad0 => Self::Numpad0,
            Numpad1 => Self::Numpad1,
            Numpad2 => Self::Numpad2,
            Numpad3 => Self::Numpad3,
            Numpad4 => Self::Numpad4,
            Numpad5 => Self::Numpad5,
            Numpad6 => Self::Numpad6,
            Numpad7 => Self::Numpad7,
            Numpad8 => Self::Numpad8,
            Numpad9 => Self::Numpad9,
            NumpadSubtract => Self::NumpadSubtract,
            NumpadAdd => Self::NumpadAdd,
            NumpadDivide => Self::NumpadDivide,
            NumpadMultiply => Self::NumpadMultiply,
            Grave => Self::Grave,
            Minus => Self::Minus,
            Equal => Self::Equal,
            LeftBracket => Self::LeftBracket,
            RightBracket => Self::RightBracket,
            BackSlash => Self::BackSlash,
            Semicolon => Self::Semicolon,
            Apostrophe => Self::Apostrophe,
            Comma => Self::Comma,
            Dot => Self::Dot,
            Slash => Self::Slash,
        }
    }
}

impl Button {
    pub fn from_mouse(mouse: &MouseState) -> Vec<Self> {
        let mut ret = Vec::with_capacity(5);
        // fixme awful lol
        if mouse.button_pressed[1] {
            ret.push(Self::Mouse1)
        }
        if mouse.button_pressed[2] {
            ret.push(Self::Mouse2)
        }
        if mouse.button_pressed[3] {
            ret.push(Self::Mouse3)
        }
        if mouse.button_pressed[4] {
            ret.push(Self::Mouse4)
        }
        if mouse.button_pressed[5] {
            ret.push(Self::Mouse5)
        }
        ret
    }
}

impl TryFrom<Button> for Key {
    type Error = ();

    fn try_from(b: Button) -> Result<Self, Self::Error> {
        use Button::*;
        // fixme oh boy
        Ok(match b {
            Key0 => Self::Layout('0'),
            Key1 => Self::Layout('1'),
            Key2 => Self::Layout('2'),
            Key3 => Self::Layout('3'),
            Key4 => Self::Layout('4'),
            Key5 => Self::Layout('5'),
            Key6 => Self::Layout('6'),
            Key7 => Self::Layout('7'),
            Key8 => Self::Layout('8'),
            Key9 => Self::Layout('9'),
            A => Self::Layout('a'),
            B => Self::Layout('b'),
            C => Self::Layout('c'),
            D => Self::Layout('d'),
            E => Self::Layout('e'),
            F => Self::Layout('f'),
            G => Self::Layout('g'),
            H => Self::Layout('h'),
            I => Self::Layout('i'),
            J => Self::Layout('j'),
            K => Self::Layout('k'),
            L => Self::Layout('l'),
            M => Self::Layout('m'),
            N => Self::Layout('n'),
            O => Self::Layout('o'),
            P => Self::Layout('p'),
            Q => Self::Layout('q'),
            R => Self::Layout('r'),
            S => Self::Layout('s'),
            T => Self::Layout('t'),
            U => Self::Layout('u'),
            V => Self::Layout('v'),
            W => Self::Layout('w'),
            X => Self::Layout('x'),
            Y => Self::Layout('y'),
            Z => Self::Layout('z'),
            F1 => Self::F1,
            F2 => Self::F2,
            F3 => Self::F3,
            F4 => Self::F4,
            F5 => Self::F5,
            F6 => Self::F6,
            F7 => Self::F7,
            F8 => Self::F8,
            F9 => Self::F9,
            F10 => Self::F10,
            F11 => Self::F11,
            F12 => Self::F12,
            Escape => Self::Escape,
            Space => Self::Space,
            LControl => Self::Control,
            // RControl => Self::,
            LShift => Self::Shift,
            // RShift => Self::,
            LAlt => Self::Alt,
            // RAlt => Self::,
            Meta => Self::Meta,
            Enter => Self::Return,
            Up => Self::UpArrow,
            Down => Self::DownArrow,
            Left => Self::LeftArrow,
            Right => Self::RightArrow,
            Backspace => Self::Backspace,
            CapsLock => Self::CapsLock,
            Tab => Self::Tab,
            Home => Self::Home,
            End => Self::End,
            PageUp => Self::PageUp,
            PageDown => Self::PageDown,
            // Insert => Self::,
            Delete => Self::Delete,
            // Numpad0 => Self::,
            // Numpad1 => Self::,
            // Numpad2 => Self::,
            // Numpad3 => Self::,
            // Numpad4 => Self::,
            // Numpad5 => Self::,
            // Numpad6 => Self::,
            // Numpad7 => Self::,
            // Numpad8 => Self::,
            // Numpad9 => Self::,
            // NumpadSubtract => Self::,
            // NumpadAdd => Self::,
            // NumpadDivide => Self::,
            // NumpadMultiply => Self::,
            Grave => Self::Layout('`'),
            Minus => Self::Layout('-'),
            Equal => Self::Layout('='),
            LeftBracket => Self::Layout('['),
            RightBracket => Self::Layout(']'),
            BackSlash => Self::Layout('\\'),
            Semicolon => Self::Layout(';'),
            Apostrophe => Self::Layout('\''),
            Comma => Self::Layout(','),
            Dot => Self::Layout('.'),
            Slash => Self::Layout('/'),
            _ => return Err(()),
        })
    }
}

impl TryFrom<Button> for MouseButton {
    type Error = ();
    fn try_from(b: Button) -> Result<Self, Self::Error> {
        use Button::*;
        Ok(match b {
            Mouse1 => Self::Left,
            Mouse2 => Self::Right,
            Mouse3 => Self::Middle,
            // Button::Mouse4 => {}
            // Button::Mouse5 => {}
            _ => return Err(()),
        })
    }
}
