use crate::desktop::event::GlobalEvent;
use dagt_core::interface::{
    draw::{Constraints, Draw},
    event::EventHandler,
    widget::{Widget, WidgetBuilder},
    window::Window,
};
use dagt_gl::egl::{context::EglContext, display::EglDisplay, surface::EglSurface};
use dagt_gl::gles::{
    func as gl,
    types::{GlBlendFact, GlCap},
};
use dagt_primitives::{color::Color, rect::Rect};
use std::sync::Mutex;
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_egl::WlEglSurface;
use wayland_protocols::xdg::shell::client::{xdg_surface::XdgSurface, xdg_toplevel::XdgToplevel};

pub struct WaylandWindow {
    pub(super) cur_conf: WindowConfigure,
    pub(super) pend_conf: WindowConfigure,
    pub(super) event_handler: Option<Box<dyn Fn(&mut WaylandWindow, &GlobalEvent) -> bool>>,
    pub(super) state: Mutex<WindowState>,
    pub(super) widget_builder: Option<Box<dyn WidgetBuilder<GlobalEvent>>>,
    pub(super) widget: Option<Box<dyn Widget<GlobalEvent>>>,
    pub(super) frame_requested: bool,
    pub(super) close_requested: bool,
}

impl Window<GlobalEvent> for WaylandWindow {
    fn frame(&mut self) {
        self.frame_requested = true;
    }

    fn close(&mut self) {
        self.close_requested = true;
    }
}

impl EventHandler<GlobalEvent> for WaylandWindow {
    fn handle_event(&mut self, event: &GlobalEvent) -> bool {
        if let Some(handler) = self.event_handler.take() {
            if handler(self, event) {
                return true;
            }
            self.event_handler = Some(handler);
        }

        let mut res = false;

        use GlobalEvent::*;
        match event {
            Configure => {
                self.cur_conf = self.pend_conf;
                self.state
                    .lock()
                    .map(|state| {
                        let constr = &self.cur_conf.constraints;
                        state.window.resize(constr.width, constr.height, 0, 0);
                    })
                    .expect("failed to get window state");
                self.frame();
                return true;
            }
            Close => {
                self.close();
                return true;
            }
            _ => {
                if let Some(widget) = self.widget.as_mut() {
                    res = widget.handle_event(event);
                }
            }
        };

        self.widget_builder.as_ref().map(|builder| {
            if builder.state_changed() {
                self.widget = Some(builder.build());
                self.frame_requested = true;
            }
        });

        res
    }
}

impl Draw for WaylandWindow {
    fn draw(&mut self, constraints: Constraints) -> bool {
        self.state
            .lock()
            .map(|state| {
                gl::enable(GlCap::Blend);
                gl::blend_func_separate(
                    GlBlendFact::SrcAlpha,
                    GlBlendFact::OneMinusSrcAlpha,
                    GlBlendFact::One,
                    GlBlendFact::One,
                );
                gl::view_port(0, 0, constraints.width, constraints.height).unwrap();

                Rect {
                    bg_color: Color::rgb(30, 30, 30),
                    bd_color: Color::rgb(46, 46, 46),
                    bd_width: 2,
                    bd_radius: 10,
                }
                .draw(constraints);

                if let Some(child) = self.widget.as_mut() {
                    child.draw(Constraints {
                        width: constraints.width - 4,
                        height: constraints.height - 4,
                        x: 2,
                        y: 2,
                        ..Default::default()
                    });
                }

                gl::disable(GlCap::Blend);

                state
                    .surface
                    .swap()
                    .expect("failed to swap EGL surface buffers");
            })
            .expect("failed to get window state");
        true
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(super) struct WindowConfigure {
    pub(super) constraints: Constraints,
}

pub(super) struct WindowState {
    pub(super) wl_surface: WlSurface,
    pub(super) xdg_surface: XdgSurface,
    pub(super) xdg_toplevel: XdgToplevel,
    pub(super) display: EglDisplay,
    pub(super) window: WlEglSurface,
    pub(super) surface: EglSurface,
    pub(super) context: EglContext,
    pub(super) title: String,
}
