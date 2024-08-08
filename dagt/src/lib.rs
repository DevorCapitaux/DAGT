pub mod prelude {
    pub use dagt_core::{AppBuilder, Widget, Window as WindowTrait, WindowBuilder};
    pub use dagt_platform::desktop::{app::App, event::*, window::Window};
    pub use dagt_widgets::{
        buttons::Button,
        children,
        layouts::{Center, Row},
        Text,
    };
    pub use xkeysym::Keysym;
}

pub mod core {
    pub use dagt_core::interface::{app, event, widget, window};
}
