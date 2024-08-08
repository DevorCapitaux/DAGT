use dagt::{
    core::{event::StateChanged, widget::WidgetBuilder},
    prelude::*,
};
use std::sync::{Arc, Mutex};

pub struct Counter {
    state: Arc<Mutex<CounterState>>,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            state: Arc::new(Mutex::new(CounterState::new())),
        }
    }
}

impl WidgetBuilder<GlobalEvent> for Counter {
    fn build(&self) -> Box<dyn Widget<GlobalEvent>> {
        let state = Arc::clone(&self.state);
        Row::new()
            .gap(40)
            .children(children![
                Button::new().text("Increment").on_click(move || {
                    state.lock().unwrap().increment();
                }),
                Center::new().child(
                    Text::new().text(&format!("Counter: {}", self.state.lock().unwrap().counter))
                ),
            ])
            .build()
    }
}

impl StateChanged for Counter {
    fn state_changed(&self) -> bool {
        self.state.lock().unwrap().state_changed()
    }
}

#[derive(Default)]
pub struct CounterState {
    changed: bool,
    counter: i32,
}

impl CounterState {
    pub fn new() -> CounterState {
        Default::default()
    }

    pub fn state_changed(&mut self) -> bool {
        if self.changed {
            self.changed = false;
            true
        } else {
            false
        }
    }

    pub fn increment(&mut self) {
        self.counter += 1;
        self.changed = true;
    }
}

fn main() {
    App::new()
        .id("example.app.counter")
        .window(
            Window::new()
                .title("CounterApp")
                .handle_event(|window, event| {
                    use GlobalEvent::*;
                    use KeyboardEvent::*;
                    match event {
                        Keyboard(KeyPressed {
                            key: _,
                            keysym,
                            utf8: _,
                        }) => match *keysym {
                            Keysym::q => {
                                println!("Closing window");
                                window.close();
                                return true;
                            }
                            _ => (),
                        },
                        _ => (),
                    };

                    false
                })
                .child(Center::new().child(Counter::new())),
        )
        .exec();
}
