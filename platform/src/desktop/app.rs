use super::{event::GlobalEvent, window::Window, PlatformApp, PlatformWindow};
use dagt_core::interface::app::{App as AppTrait, AppBuilder};

#[derive(Default)]
pub struct App {
    pub(crate) id: String,
    pub(crate) window: Option<<App as AppBuilder>::PlatformWindowBuilder>,
}

impl App {
    pub fn new() -> App {
        Default::default()
    }
}

impl AppBuilder for App {
    type EventSource = GlobalEvent;
    type PlatformWindow = PlatformWindow;
    type PlatformWindowBuilder = Window;

    fn id(mut self, id: &str) -> Self {
        self.id = id.to_owned();
        self
    }

    fn window(mut self, window: Self::PlatformWindowBuilder) -> Self {
        self.window = Some(window);
        self
    }

    fn exec(self) {
        PlatformApp::build(self).exec();
    }
}
