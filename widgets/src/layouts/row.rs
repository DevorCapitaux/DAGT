use dagt_core::{Constraints, Draw, EventHandler, StateChanged, Widget, WidgetBuilder};
use dagt_platform::desktop::event::GlobalEvent;
use dagt_primitives::{color::Color, rect::Rect};
use std::sync::Arc;

#[derive(Default)]
pub struct RowBuilder {
    padding: u32,
    gap: u32,
    children: Option<Vec<Arc<dyn WidgetBuilder<GlobalEvent>>>>,
}

impl RowBuilder {
    pub fn new() -> RowBuilder {
        Default::default()
    }

    pub fn children(mut self, widgets: Vec<Arc<dyn WidgetBuilder<GlobalEvent>>>) -> Self {
        self.children = Some(widgets);
        self
    }

    pub fn padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    pub fn gap(mut self, gap: u32) -> Self {
        self.gap = gap;
        self
    }
}

impl WidgetBuilder<GlobalEvent> for RowBuilder {
    fn build(&self) -> Box<dyn Widget<GlobalEvent>> {
        let children = match self.children.as_ref() {
            Some(children) => Some(children.iter().map(|c| c.build()).collect()),
            None => None,
        };

        Box::new(Row {
            gap: self.gap,
            padding: self.padding,
            children,
            constraints: None,
        })
    }
}

impl StateChanged for RowBuilder {
    fn state_changed(&self) -> bool {
        if let Some(children) = &self.children {
            for child in children {
                if child.state_changed() {
                    return true;
                }
            }
        }
        false
    }

    fn frame_requested(&self) -> bool {
        if let Some(children) = &self.children {
            for child in children {
                if child.frame_requested() {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Default)]
struct Row {
    padding: u32,
    gap: u32,
    children: Option<Vec<Box<dyn Widget<GlobalEvent>>>>,
    constraints: Option<Constraints>,
}

impl Row {
    fn on_widget(&mut self, p_x: f64, p_y: f64) -> Option<&mut Box<dyn Widget<GlobalEvent>>> {
        let constr = self.constraints();

        let x = constr.x;
        let y = constr.y;
        let width = constr.width;
        let height = constr.height;
        let pad = self.padding as i32;
        let gap = self.gap as i32;

        if let Some(children) = self.children.as_mut() {
            let len = children.len() as i32;
            for (i, child) in children.iter_mut().enumerate() {
                let i = i as i32;
                let width = (width - 2 * pad - (len - 1) * gap) / len;
                let x = x + pad + (width + gap) * i;
                let height = height - 2 * pad;
                let y = y + pad;
                let constr = Constraints {
                    width,
                    x,
                    height,
                    y,
                    ..Default::default()
                };

                if constr.in_box(p_x, p_y) {
                    return Some(child);
                }
            }
        }
        None
    }
}

impl Widget<GlobalEvent> for Row {
    fn constraints(&self) -> Constraints {
        let mut max_height = 0;
        let mut width = 0;

        let mut widget_num = 0;

        self.children.as_ref().map(|children| {
            widget_num = children.len();
            children.iter().for_each(|child| {
                let constr = child.constraints();
                width += constr.width;
                max_height = max_height.max(constr.height);
            });
        });
        max_height += self.padding as i32 * 2;
        width +=
            self.padding as i32 * 2 + (widget_num as i32 - 1).clamp(0, i32::MAX) * self.gap as i32;

        Constraints {
            width,
            height: max_height,
            ..self.constraints.unwrap_or_default()
        }
    }
}

impl EventHandler<GlobalEvent> for Row {
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

impl Draw for Row {
    fn draw(&mut self, constraints: Constraints) -> bool {
        self.constraints = Some(constraints);

        let x = constraints.x;
        let y = constraints.y;
        let width = constraints.width;
        let height = constraints.height;
        let pad = self.padding as i32;
        let gap = self.gap as i32;

        let mut res = false;
        if let Some(children) = self.children.as_mut() {
            let len = children.len() as i32;
            for (i, child) in children.iter_mut().enumerate() {
                let i = i as i32;
                let width = (width - 2 * pad - (len - 1) * gap) / len;
                let x = x + pad + (width + gap) * i;
                let height = height - 2 * pad;
                let y = y + pad;
                let constr = Constraints {
                    width,
                    x,
                    height,
                    y,
                    ..Default::default()
                };

                // Rect {
                //     bg_color: Color::white(),
                //     ..Default::default()
                // }
                // .draw(constr.check());

                if child.draw(constr.check()) {
                    res = true;
                }
            }
        }

        res
    }
}
