use super::wayland::app::globals::seat::keyboard::Modifiers;
use dagt_core::interface::event::Event;
use xkeysym::Keysym;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Button {
    Left,
    Right,
    Middle,
}

pub enum PointerEvent {
    Enter,
    Leave,
    Motion,
    ButtonPressed(Button),
    ButtonReleased(Button),
    ButtonClicked(Button),
}

pub enum KeyboardEvent {
    Enter,
    Leave,
    KeyPressed {
        key: u32,
        keysym: Keysym,
        utf8: String,
    },
    KeyReleased {
        key: u32,
        keysym: Keysym,
        utf8: String,
    },
    Modifiers(Modifiers),
}

pub enum GlobalEvent {
    Pointer { e: PointerEvent, x: f64, y: f64 },
    Keyboard(KeyboardEvent),
    Configure,
    Close,
}

impl Event for GlobalEvent {
    fn source(&self) -> &Self {
        self
    }
}
