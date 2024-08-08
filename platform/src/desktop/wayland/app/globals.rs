use wayland_client::{
    globals::{registry_queue_init, GlobalListContents},
    protocol::{wl_compositor::WlCompositor, wl_registry::WlRegistry, wl_seat::WlSeat},
    Connection, Dispatch, EventQueue,
};
use wayland_protocols::xdg::shell::client::xdg_wm_base::XdgWmBase;

use self::seat::Seat;

pub mod seat;

pub struct Globals {
    connection: Connection,
    compositor: WlCompositor,
    seat: Seat,
    xdg: XdgWmBase,
}

impl Globals {
    pub fn new<State, U>(data: U) -> (Globals, EventQueue<State>)
    where
        State: Dispatch<WlRegistry, GlobalListContents>
            + Dispatch<WlCompositor, U>
            + Dispatch<WlSeat, U>
            + Dispatch<XdgWmBase, U>
            + 'static,
        U: Send + Sync + Copy + 'static,
    {
        let connection =
            Connection::connect_to_env().expect("failed to connect to the Wayland server");
        let (globals, eq) = registry_queue_init(&connection)
            .expect("failed to init an event queue and retrieve the initial list of globals");
        let qh = eq.handle();
        let compositor = globals
            .bind(&qh, 6..=6, data)
            .expect("failed to bind wl_compositor");
        let wl_seat = globals
            .bind(&qh, 8..=8, data)
            .expect("failed to bind wl_seat");
        let xdg = globals
            .bind(&qh, 6..=6, data)
            .expect("failed to bind xdg_wm_base");

        let seat = Seat::new(wl_seat);

        (
            Globals {
                connection,
                compositor,
                seat,
                xdg,
            },
            eq,
        )
    }

    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    pub fn compositor(&self) -> &WlCompositor {
        &self.compositor
    }

    pub fn seat(&mut self) -> &mut Seat {
        &mut self.seat
    }

    pub fn xdg(&self) -> &XdgWmBase {
        &self.xdg
    }
}
