use super::{
    draw::Draw,
    event::{Event, EventHandler},
    widget::WidgetBuilder,
};

pub trait Window<E: Event>: EventHandler<E> + Draw {
    fn frame(&mut self);
    fn close(&mut self);
}

pub trait WindowBuilder<E: Event, W: Window<E>> {
    fn title(self, title: &str) -> Self;
    fn child(self, widget: impl WidgetBuilder<E> + 'static) -> Self;
    fn handle_event(self, event_handler: impl Fn(&mut W, &E) -> bool + 'static) -> Self;
}
