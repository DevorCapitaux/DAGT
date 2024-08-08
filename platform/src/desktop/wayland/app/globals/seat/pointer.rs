use super::Seat;
use crate::desktop::{event::Button, wayland::app::AppData};
use wayland_client::{
    protocol::{
        wl_pointer::{self, WlPointer},
        wl_surface::WlSurface,
    },
    Dispatch, WEnum,
};

pub struct Pointer {
    wl_pointer: WlPointer,
    focus: Option<WlSurface>,
    coords: (f64, f64),
    accumulated_events: Vec<wl_pointer::Event>,
    last_button: Option<Button>,
}

impl Pointer {
    pub fn new(wl_pointer: WlPointer) -> Pointer {
        Pointer {
            wl_pointer,
            focus: None,
            coords: (0.0, 0.0),
            accumulated_events: Vec::new(),
            last_button: None,
        }
    }

    pub fn focus(&self) -> &Option<WlSurface> {
        &self.focus
    }

    pub fn coords(&self) -> (f64, f64) {
        self.coords
    }
}

impl Drop for Pointer {
    fn drop(&mut self) {
        self.wl_pointer.release();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonState {
    Pressed,
    Released,
    Clicked,
}

pub trait PointerHandler {
    fn enter(&mut self, serial: u32, surface: WlSurface, x: f64, y: f64);
    fn leave(&mut self, serial: u32, surface: WlSurface);
    fn motion(&mut self, time: u32, x: f64, y: f64);
    fn button(&mut self, serial: u32, time: u32, state: ButtonState, button: Button);
}

impl<State> Dispatch<WlPointer, AppData, State> for Pointer
where
    State: Dispatch<WlPointer, AppData>,
    State: AsMut<Seat>,
    State: PointerHandler,
{
    fn event(
        state: &mut State,
        _proxy: &WlPointer,
        event: <WlPointer as wayland_client::Proxy>::Event,
        _data: &AppData,
        _conn: &wayland_client::Connection,
        _qhandle: &wayland_client::QueueHandle<State>,
    ) {
        let pointer = state.as_mut().pointer.as_mut().unwrap();

        use wl_pointer::Event;
        match event {
            Event::Enter {
                serial,
                surface,
                surface_x,
                surface_y,
            } => {
                pointer.focus = Some(surface.clone());
                pointer.coords = (surface_x, surface_y);
                pointer.accumulated_events.push(wl_pointer::Event::Enter {
                    serial,
                    surface,
                    surface_x,
                    surface_y,
                });
            }
            Event::Leave { serial, surface } => {
                pointer.focus = None;
                pointer
                    .accumulated_events
                    .push(wl_pointer::Event::Leave { serial, surface });
            }
            Event::Motion {
                time,
                surface_x,
                surface_y,
            } => {
                pointer.coords = (surface_x, surface_y);
                pointer.accumulated_events.push(wl_pointer::Event::Motion {
                    time,
                    surface_x,
                    surface_y,
                });
            }
            Event::Button {
                serial,
                time,
                button,
                state,
            } => {
                pointer.accumulated_events.push(wl_pointer::Event::Button {
                    serial,
                    time,
                    button,
                    state,
                });
            }
            Event::Frame => {
                if pointer.accumulated_events.len() != 1 {
                    pointer.accumulated_events.clear();
                    return;
                }

                match pointer.accumulated_events[0] {
                    Event::Button {
                        serial: _,
                        time: _,
                        button: _,
                        state: _,
                    } => (),
                    _ => pointer.last_button = None,
                };

                match pointer.accumulated_events.pop().unwrap() {
                    Event::Enter {
                        serial,
                        surface,
                        surface_x,
                        surface_y,
                    } => state.enter(serial, surface, surface_x, surface_y),
                    Event::Leave { serial, surface } => state.leave(serial, surface),
                    Event::Motion {
                        time,
                        surface_x,
                        surface_y,
                    } => state.motion(time, surface_x, surface_y),
                    Event::Button {
                        serial,
                        time,
                        button,
                        state: button_state,
                    } => match button_state {
                        WEnum::Value(button_state) => {
                            let button = match button {
                                272 => Button::Left,
                                273 => Button::Right,
                                274 => Button::Middle,
                                _ => return,
                            };

                            let button_state = match button_state {
                                wl_pointer::ButtonState::Pressed => ButtonState::Pressed,
                                wl_pointer::ButtonState::Released => ButtonState::Released,
                                _ => return,
                            };

                            let last = match button_state {
                                ButtonState::Pressed => {
                                    pointer.last_button = Some(button);
                                    None
                                }
                                ButtonState::Released => pointer.last_button.take(),
                                _ => None,
                            };

                            state.button(serial, time, button_state, button);
                            if let Some(last) = last {
                                if last == button {
                                    state.button(serial, time, ButtonState::Clicked, button);
                                }
                            }
                        }
                        WEnum::Unknown(value) => {
                            println!("wl_pointer: unknown button state: {value}")
                        }
                    },
                    _ => (),
                }
            }
            Event::Axis {
                time: _,
                axis: _,
                value: _,
            } => {}
            Event::AxisSource { axis_source: _ } => {}
            Event::AxisStop { time: _, axis: _ } => {}
            Event::AxisDiscrete {
                axis: _,
                discrete: _,
            } => {}
            Event::AxisValue120 {
                axis: _,
                value120: _,
            } => {}
            Event::AxisRelativeDirection {
                axis: _,
                direction: _,
            } => {}
            _ => unreachable!(),
        }
    }
}
