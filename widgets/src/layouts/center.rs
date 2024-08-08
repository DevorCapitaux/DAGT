use dagt_core::{Constraints, Draw, EventHandler, StateChanged, Widget, WidgetBuilder};
use dagt_platform::desktop::event::GlobalEvent;

#[derive(Default)]
pub struct CenterBuilder {
    child: Option<Box<dyn WidgetBuilder<GlobalEvent>>>,
}

impl CenterBuilder {
    pub fn new() -> CenterBuilder {
        Default::default()
    }

    pub fn child(mut self, widget: impl WidgetBuilder<GlobalEvent> + 'static) -> Self {
        self.child = Some(Box::new(widget));
        self
    }
}

impl WidgetBuilder<GlobalEvent> for CenterBuilder {
    fn build(&self) -> Box<dyn Widget<GlobalEvent>> {
        let child = self
            .child
            .as_ref()
            .map_or(None, |child| Some(child.build()));

        Box::new(Center {
            child,
            constraints: None,
        })
    }
}

impl StateChanged for CenterBuilder {
    fn state_changed(&self) -> bool {
        self.child
            .as_ref()
            .map_or(false, |child| child.state_changed())
    }

    fn frame_requested(&self) -> bool {
        self.child
            .as_ref()
            .map_or(false, |child| child.state_changed())
    }
}

struct Center {
    child: Option<Box<dyn Widget<GlobalEvent>>>,
    constraints: Option<Constraints>,
}

impl Center {
    fn on_widget(&mut self, x: f64, y: f64) -> Option<&mut Box<dyn Widget<GlobalEvent>>> {
        self.child.as_mut().map_or(None, |child| {
            if self.constraints.is_none() {
                return None;
            }
            let constr = self.constraints.unwrap();
            let child_constr = child.constraints();

            let width = constr.width.min(child_constr.width);
            let height = constr.height.min(child_constr.height);
            let c_x = if constr.width > child_constr.width {
                constr.x + (constr.width - child_constr.width) / 2
            } else {
                constr.x
            };
            let c_y = if constr.height > child_constr.height {
                constr.y + (constr.height - child_constr.height) / 2
            } else {
                constr.y
            };

            let c = Constraints {
                width,
                height,
                x: c_x,
                y: c_y,
                ..Default::default()
            }
            .check();

            if c.in_box(x, y) {
                Some(child)
            } else {
                None
            }
        })
    }
}

impl Widget<GlobalEvent> for Center {
    fn constraints(&self) -> dagt_core::Constraints {
        self.child
            .as_ref()
            .map_or(Default::default(), |child| child.constraints())
    }
}

impl EventHandler<GlobalEvent> for Center {
    fn handle_event(&mut self, event: &GlobalEvent) -> bool {
        use GlobalEvent::*;
        match event {
            Pointer { e: _, x, y } => self
                .on_widget(*x, *y)
                .map_or(false, |w| w.handle_event(event)),
            _ => false,
        }
    }
}

impl Draw for Center {
    fn draw(&mut self, constraints: Constraints) -> bool {
        self.constraints = Some(constraints);

        self.child.as_mut().map_or(false, |child| {
            let constr = constraints;
            let child_constr = child.constraints();

            let width = constr.width.min(child_constr.width) + 20;
            let height = constr.height.min(child_constr.height);
            let x = if constr.width > child_constr.width {
                constr.x + (constr.width - child_constr.width) / 2
            } else {
                constr.x
            };
            let y = if constr.height > child_constr.height {
                constr.y + (constr.height - child_constr.height) / 2
            } else {
                constr.y
            };

            child.draw(
                Constraints {
                    width,
                    height,
                    x,
                    y,
                    ..Default::default()
                }
                .check(),
            )
        })
    }
}
