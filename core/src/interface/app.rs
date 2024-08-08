use super::{event::Event, window::WindowBuilder};
use crate::Window;

pub trait App<B: AppBuilder> {
    fn build(builder: B) -> Self;
    fn exec(self);
}

pub trait AppBuilder {
    type EventSource: Event;
    type PlatformWindow: Window<Self::EventSource>;
    type PlatformWindowBuilder: WindowBuilder<Self::EventSource, Self::PlatformWindow>;

    fn id(self, id: &str) -> Self;
    fn window(self, window: Self::PlatformWindowBuilder) -> Self;
    fn exec(self);
}
