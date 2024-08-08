use self::globals::{
    seat::{keyboard::Keyboard, pointer::Pointer, Seat},
    Globals,
};
use super::window::{WaylandWindow, WindowState};
use crate::desktop::{
    app::App as AppBuilder,
    event::{Button, GlobalEvent, KeyboardEvent, PointerEvent},
    wayland::window::WindowConfigure,
};
use dagt_core::interface::{
    app::App,
    draw::{Constraints, Draw},
    event::EventHandler,
};
use dagt_gl::egl::{
    config::attrs::{EglConfAttrs, EglSurfaceType},
    context::attrs::EglCtxAttrs,
    display::EglDisplay,
    types::{NativeDisplayType, NativeWindowType},
};
use globals::seat::{
    keyboard::KeyboardHandler,
    pointer::{ButtonState, PointerHandler},
};
use std::sync::Mutex;
use wayland_client::{
    delegate_dispatch,
    globals::GlobalListContents,
    protocol::{
        wl_compositor::WlCompositor,
        wl_keyboard::{KeyState, WlKeyboard},
        wl_pointer::WlPointer,
        wl_registry::WlRegistry,
        wl_seat::WlSeat,
        wl_surface::{self, WlSurface},
    },
    Dispatch, EventQueue, Proxy,
};
use wayland_egl::WlEglSurface;
use wayland_protocols::xdg::shell::client::{
    xdg_surface::{self, XdgSurface},
    xdg_toplevel::{self, XdgToplevel},
    xdg_wm_base::{self, XdgWmBase},
};

pub mod globals;

pub struct WaylandApp {
    state: Mutex<AppState>,
}

impl App<AppBuilder> for WaylandApp {
    fn build(builder: AppBuilder) -> Self {
        let state = Mutex::new(AppState::new(builder));
        Self { state }
    }

    fn exec(self) {
        self.state.lock().expect("couldn't find app state").exec();
    }
}

struct AppState {
    event_queue: Option<EventQueue<AppState>>,
    globals: Globals,
    _id: String,
    window: WaylandWindow,
    running: bool,
}

#[derive(Clone, Copy)]
struct AppData;

impl AppState {
    fn new(builder: AppBuilder) -> AppState {
        let (globals, eq) = Globals::new(AppData);
        let qh = eq.handle();
        let id = builder.id;

        let win_builder = builder.window.unwrap_or_default();
        let title = win_builder.title;
        let event_handler = win_builder.event_handler;
        let widget_builder = win_builder.child;
        let widget = match &widget_builder {
            Some(builder) => Some(builder.build()),
            None => None,
        };

        let wl_surface = globals.compositor().create_surface(&qh, AppData);
        let xdg_surface = globals.xdg().get_xdg_surface(&wl_surface, &qh, AppData);
        let xdg_toplevel = xdg_surface.get_toplevel(&qh, AppData);
        xdg_toplevel.set_app_id(id.clone());
        xdg_toplevel.set_title(title.clone());

        let constr = Constraints {
            width: 600,
            height: 600,
            ..Default::default()
        };

        let display =
            EglDisplay::get(globals.connection().display().id().as_ptr() as NativeDisplayType)
                .expect("failed to get EGL display");
        let (major, minor) = display.init().unwrap();
        println!("EGL version: {major}.{minor}");

        let config = display
            .get_conf_first(Some(
                &EglConfAttrs::new()
                    .red_size(8)
                    .blue_size(8)
                    .green_size(8)
                    .alpha_size(8)
                    .stencil_size(8)
                    .surface_type(EglSurfaceType::new().window()),
            ))
            .expect("failed to get EGL config");

        let window = WlEglSurface::new(wl_surface.id(), constr.width, constr.height)
            .expect("failed to get EGL window");

        let surface = display
            .create_window_surface(&config, window.ptr() as NativeWindowType)
            .expect("failed to create EGL window surface");

        let context = display
            .create_context(
                &config,
                None,
                Some(&EglCtxAttrs::new().major_version(3).minor_version(2)),
            )
            .expect("failed to get EGL context");
        context
            .make_current(&surface, &surface)
            .expect("failed to make EGL context current");

        let window_state = WindowState {
            wl_surface,
            xdg_surface,
            xdg_toplevel,
            display,
            window,
            surface,
            context,
            title,
        };

        let cur_conf = WindowConfigure {
            constraints: constr,
        };
        let pend_conf = cur_conf.clone();

        let window = WaylandWindow {
            cur_conf,
            pend_conf,
            event_handler,
            state: Mutex::new(window_state),
            widget_builder,
            widget,
            frame_requested: true,
            close_requested: false,
        };

        AppState {
            event_queue: Some(eq),
            globals,
            _id: id,
            window,
            running: true,
        }
    }

    fn exec(&mut self) {
        while self.running {
            let mut eq = self.event_queue.take().expect("couldn't get event queue");

            eq.blocking_dispatch(self)
                .expect("failed to dispatch event queue");

            self.event_queue = Some(eq);

            if self.window.frame_requested {
                self.window.frame_requested = false;
                self.window.draw(self.window.cur_conf.constraints.clone());
            }
            if self.window.close_requested {
                self.running = false;
            }
        }
    }
}

impl Dispatch<WlRegistry, GlobalListContents> for AppState {
    fn event(
        _state: &mut Self,
        _proxy: &WlRegistry,
        _event: <WlRegistry as wayland_client::Proxy>::Event,
        _data: &GlobalListContents,
        _conn: &wayland_client::Connection,
        _qhandle: &wayland_client::QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlCompositor, AppData> for AppState {
    fn event(
        _state: &mut Self,
        _proxy: &WlCompositor,
        _event: <WlCompositor as wayland_client::Proxy>::Event,
        _data: &AppData,
        _conn: &wayland_client::Connection,
        _qhandle: &wayland_client::QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<XdgWmBase, AppData> for AppState {
    fn event(
        _state: &mut Self,
        proxy: &XdgWmBase,
        event: <XdgWmBase as wayland_client::Proxy>::Event,
        _data: &AppData,
        _conn: &wayland_client::Connection,
        _qhandle: &wayland_client::QueueHandle<Self>,
    ) {
        use xdg_wm_base::Event::*;
        match event {
            Ping { serial } => {
                proxy.pong(serial);
            }
            _ => unreachable!(),
        };
    }
}

impl Dispatch<WlSurface, AppData> for AppState {
    fn event(
        _state: &mut AppState,
        _proxy: &WlSurface,
        event: <WlSurface as wayland_client::Proxy>::Event,
        _data: &AppData,
        _conn: &wayland_client::Connection,
        _qhandle: &wayland_client::QueueHandle<AppState>,
    ) {
        use wl_surface::Event::*;
        match event {
            Enter { output: _ } => {}
            Leave { output: _ } => {}
            PreferredBufferScale { factor: _ } => {}
            PreferredBufferTransform { transform: _ } => {}
            _ => unreachable!(),
        }
    }
}

impl Dispatch<XdgSurface, AppData> for AppState {
    fn event(
        state: &mut AppState,
        proxy: &XdgSurface,
        event: <XdgSurface as wayland_client::Proxy>::Event,
        _data: &AppData,
        _conn: &wayland_client::Connection,
        _qhandle: &wayland_client::QueueHandle<AppState>,
    ) {
        use xdg_surface::Event::*;
        match event {
            Configure { serial } => {
                state.window.handle_event(&GlobalEvent::Configure);
                proxy.ack_configure(serial);
            }
            _ => unreachable!(),
        }
    }
}

impl Dispatch<XdgToplevel, AppData> for AppState {
    fn event(
        state: &mut AppState,
        _proxy: &XdgToplevel,
        event: <XdgToplevel as wayland_client::Proxy>::Event,
        _data: &AppData,
        _conn: &wayland_client::Connection,
        _qhandle: &wayland_client::QueueHandle<AppState>,
    ) {
        use xdg_toplevel::Event::*;
        match event {
            Configure {
                width,
                height,
                states: _,
            } => {
                state.window.pend_conf.constraints.resize(width, height);
            }
            Close => {
                println!("Close event");
                state.window.handle_event(&GlobalEvent::Close);
            }
            ConfigureBounds {
                width: _,
                height: _,
            } => {}
            WmCapabilities { capabilities: _ } => {}
            _ => unreachable!(),
        };
    }
}

impl AsMut<Seat> for AppState {
    fn as_mut(&mut self) -> &mut Seat {
        self.globals.seat()
    }
}

impl PointerHandler for AppState {
    fn enter(&mut self, _serial: u32, surface: WlSurface, x: f64, y: f64) {
        let event = GlobalEvent::Pointer {
            e: PointerEvent::Enter,
            x,
            y,
        };

        if self.window.state.lock().unwrap().wl_surface == surface {
            self.window.handle_event(&event);
        }
    }

    fn leave(&mut self, _serial: u32, surface: WlSurface) {
        let (x, y) = self.globals.seat().pointer().as_ref().unwrap().coords();
        let event = GlobalEvent::Pointer {
            e: PointerEvent::Leave,
            x,
            y,
        };

        if self.window.state.lock().unwrap().wl_surface == surface {
            self.window.handle_event(&event);
        }
    }

    fn motion(&mut self, _time: u32, x: f64, y: f64) {
        let event = GlobalEvent::Pointer {
            e: PointerEvent::Motion,
            x,
            y,
        };

        self.window.handle_event(&event);
    }

    fn button(&mut self, _serial: u32, _time: u32, state: ButtonState, button: Button) {
        let (x, y) = self.globals.seat().pointer().as_ref().unwrap().coords();
        let event = GlobalEvent::Pointer {
            e: match state {
                ButtonState::Pressed => PointerEvent::ButtonPressed(button),
                ButtonState::Released => PointerEvent::ButtonReleased(button),
                ButtonState::Clicked => PointerEvent::ButtonClicked(button),
            },
            x,
            y,
        };

        self.window.handle_event(&event);
    }
}

impl KeyboardHandler for AppState {
    fn enter(
        &mut self,
        _serial: u32,
        surface: WlSurface,
        _keys: Vec<u32>,
        _keysyms: Vec<xkeysym::Keysym>,
    ) {
        let event = GlobalEvent::Keyboard(KeyboardEvent::Enter);

        if self.window.state.lock().unwrap().wl_surface == surface {
            self.window.handle_event(&event);
        }
    }

    fn leave(&mut self, _serial: u32, surface: WlSurface) {
        let event = GlobalEvent::Keyboard(KeyboardEvent::Leave);

        if self.window.state.lock().unwrap().wl_surface == surface {
            self.window.handle_event(&event);
        }
    }

    fn key(
        &mut self,
        _serial: u32,
        _time: u32,
        state: KeyState,
        key: u32,
        keysym: xkeysym::Keysym,
        utf8: String,
    ) {
        let event = GlobalEvent::Keyboard(match state {
            KeyState::Pressed => KeyboardEvent::KeyPressed { key, keysym, utf8 },
            KeyState::Released => KeyboardEvent::KeyReleased { key, keysym, utf8 },
            _ => return,
        });

        self.window.handle_event(&event);
    }

    fn modifiers(
        &mut self,
        _serial: u32,
        modifiers: globals::seat::keyboard::Modifiers,
        _group: u32,
    ) {
        let event = GlobalEvent::Keyboard(KeyboardEvent::Modifiers(modifiers));

        self.window.handle_event(&event);
    }
}

delegate_dispatch!(AppState: [WlSeat: AppData] => Seat);
delegate_dispatch!(AppState: [WlKeyboard: AppData] => Keyboard);
delegate_dispatch!(AppState: [WlPointer: AppData] => Pointer);
