use crate::Text;
use dagt_core::{Constraints, Draw, EventHandler, StateChanged, Widget, WidgetBuilder};
use dagt_platform::desktop::event::{GlobalEvent, PointerEvent};
use dagt_primitives::{color::Color, rect::Rect};
use std::sync::Arc;

pub struct ButtonBuilder {
    text: String,
    on_click: Option<Arc<dyn Fn()>>,
    constraints: Option<Constraints>,
    color_def: Color,
    color_hov: Color,
}

impl ButtonBuilder {
    pub fn new() -> ButtonBuilder {
        ButtonBuilder {
            text: String::new(),
            on_click: None,
            constraints: None,
            color_def: Color::rgb(68, 68, 68),
            color_hov: Color::rgb(0, 100, 0),
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_owned();
        self
    }

    pub fn on_click(mut self, on_click: impl Fn() + 'static) -> Self {
        self.on_click = Some(Arc::new(on_click));
        self
    }
}

impl WidgetBuilder<GlobalEvent> for ButtonBuilder {
    fn build(&self) -> Box<dyn dagt_core::Widget<GlobalEvent>> {
        let text_widget = Text::new().text(&self.text).build();

        Box::new(Button {
            text: self.text.clone(),
            text_widget,
            on_click: self.on_click.clone(),
            constraints: self.constraints,
            color_def: self.color_def,
            color_hov: self.color_hov,
            padding: 20,
        })
    }
}

impl StateChanged for ButtonBuilder {}

struct Button {
    text: String,
    text_widget: Box<dyn Widget<GlobalEvent>>,
    on_click: Option<Arc<dyn Fn()>>,
    constraints: Option<Constraints>,
    color_def: Color,
    color_hov: Color,
    padding: i32,
}

impl Widget<GlobalEvent> for Button {
    fn constraints(&self) -> dagt_core::Constraints {
        let constr = self.text_widget.constraints();

        Constraints {
            width: constr.width + self.padding * 2,
            height: constr.height + self.padding * 2,
            ..self.constraints.unwrap_or_default()
        }
    }
}

impl EventHandler<GlobalEvent> for Button {
    fn handle_event(&mut self, event: &GlobalEvent) -> bool {
        use GlobalEvent::*;
        match event {
            Pointer {
                e: PointerEvent::ButtonClicked(_),
                x: _,
                y: _,
            } => {
                self.on_click.as_ref().map(|callback| callback());
                true
            }
            _ => false,
        }
    }
}

impl Draw for Button {
    fn draw(&mut self, constraints: Constraints) -> bool {
        Rect {
            bg_color: self.color_def,
            bd_radius: 30,
            ..Default::default()
        }
        .draw(Constraints {
            x: constraints.x,
            y: constraints.y,
            ..self.constraints()
        });

        // let constr = self.text_widget.constraints();
        // let x_offset = (constraints.width - constr.width) / 2;

        self.text_widget.draw(Constraints {
            // x: constraints.x + x_offset,
            x: constraints.x + self.padding,
            y: constraints.y + self.padding,
            ..constraints
        });

        true
    }
}
