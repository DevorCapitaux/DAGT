use super::{event::GlobalEvent, PlatformWindow};
use dagt_core::interface::{widget::WidgetBuilder, window::WindowBuilder};

#[derive(Default)]
pub struct Window {
    pub(crate) title: String,
    pub(crate) child: Option<Box<dyn WidgetBuilder<GlobalEvent>>>,
    pub(crate) event_handler: Option<Box<dyn Fn(&mut PlatformWindow, &GlobalEvent) -> bool>>,
}

impl Window {
    pub fn new() -> Window {
        Default::default()
    }
}

impl WindowBuilder<GlobalEvent, PlatformWindow> for Window {
    fn title(mut self, title: &str) -> Self {
        self.title = title.to_owned();
        self
    }

    fn child(mut self, widget: impl WidgetBuilder<GlobalEvent> + 'static) -> Self {
        self.child = Some(Box::new(widget));
        self
    }

    fn handle_event(
        mut self,
        event_handler: impl Fn(&mut PlatformWindow, &GlobalEvent) -> bool + 'static,
    ) -> Self {
        self.event_handler = Some(Box::new(event_handler));
        self
    }
}
