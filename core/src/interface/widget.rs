use super::{
    draw::Draw,
    event::{Event, EventHandler, StateChanged},
};
use crate::Constraints;

pub trait Widget<E: Event>: EventHandler<E> + Draw {
    fn constraints(&self) -> Constraints;
}

pub trait WidgetBuilder<E: Event>: StateChanged {
    fn build(&self) -> Box<dyn Widget<E>>;
}
