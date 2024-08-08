pub trait Event<EventSource = Self> {
    fn source(&self) -> &EventSource;
}

pub trait EventHandler<E: Event> {
    fn handle_event(&mut self, event: &E) -> bool {
        false
    }
}

pub trait StateChanged {
    fn state_changed(&self) -> bool {
        false
    }

    fn frame_requested(&self) -> bool {
        false
    }
}
