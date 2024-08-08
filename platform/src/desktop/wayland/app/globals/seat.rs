use self::keyboard::Keyboard;
use crate::desktop::wayland::app::AppData;
use pointer::Pointer;
use wayland_client::{
    protocol::{
        wl_keyboard::WlKeyboard,
        wl_pointer::WlPointer,
        wl_seat::{self, Capability, WlSeat},
    },
    Dispatch,
};

pub mod keyboard;
pub mod pointer;

pub struct Seat {
    wl_seat: WlSeat,
    name: String,
    keyboard: Option<Keyboard>,
    pointer: Option<Pointer>,
}

impl Seat {
    pub(super) fn new(wl_seat: WlSeat) -> Seat {
        Seat {
            wl_seat,
            name: String::new(),
            keyboard: None,
            pointer: None,
        }
    }

    pub fn pointer(&self) -> &Option<Pointer> {
        &self.pointer
    }

    pub fn keyboard(&self) -> &Option<Keyboard> {
        &self.keyboard
    }
}

impl<State> Dispatch<WlSeat, AppData, State> for Seat
where
    State: Dispatch<WlSeat, AppData>,
    State: Dispatch<WlKeyboard, AppData>,
    State: Dispatch<WlPointer, AppData>,
    State: AsMut<Seat>,
    State: 'static,
{
    fn event(
        state: &mut State,
        _proxy: &WlSeat,
        event: <WlSeat as wayland_client::Proxy>::Event,
        _data: &AppData,
        _conn: &wayland_client::Connection,
        qhandle: &wayland_client::QueueHandle<State>,
    ) {
        use wl_seat::Event::*;
        match event {
            Capabilities { capabilities } => {
                let capabilities = Capability::from_bits_truncate(capabilities.into());

                let keyboard = capabilities.contains(Capability::Keyboard);
                let pointer = capabilities.contains(Capability::Pointer);

                let seat = state.as_mut();

                if keyboard && seat.keyboard.is_none() {
                    seat.keyboard =
                        Some(Keyboard::new(seat.wl_seat.get_keyboard(qhandle, AppData)));
                } else if !keyboard && seat.keyboard.is_some() {
                    seat.keyboard = None;
                }

                if pointer && seat.pointer.is_none() {
                    seat.pointer = Some(Pointer::new(seat.wl_seat.get_pointer(qhandle, AppData)));
                } else if !pointer && seat.pointer.is_some() {
                    seat.pointer = None;
                }
            }
            Name { name } => {
                state.as_mut().name = name;
            }
            _ => unreachable!(),
        }
    }
}
