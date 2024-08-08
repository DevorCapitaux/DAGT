pub mod interface {
    pub mod app;
    pub mod draw;
    pub mod event;
    pub mod widget;
    pub mod window;
}

pub use interface::{app::*, draw::*, event::*, widget::*, window::*};
