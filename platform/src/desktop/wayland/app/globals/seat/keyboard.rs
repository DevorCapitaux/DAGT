use super::Seat;
use crate::desktop::wayland::app::AppData;
use core::panic;
use wayland_client::{
    protocol::{
        wl_keyboard::{self, KeyState, KeymapFormat, WlKeyboard},
        wl_surface::WlSurface,
    },
    Dispatch, WEnum,
};
use xkbcommon::xkb;
use xkeysym::{KeyCode, Keysym};

pub struct Keyboard {
    wl_keyboard: WlKeyboard,
    xkb_context: xkb::Context,
    xkb_state: Option<xkb::State>,
}

impl Keyboard {
    pub fn new(wl_keyboard: WlKeyboard) -> Keyboard {
        Keyboard {
            wl_keyboard,
            xkb_context: xkb::Context::new(xkb::CONTEXT_NO_FLAGS),
            xkb_state: None,
        }
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        self.wl_keyboard.release();
    }
}

pub struct Modifiers {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub logo: bool,
}

impl Modifiers {
    fn new(state: &xkb::State) -> Modifiers {
        Modifiers {
            ctrl: state.mod_name_is_active(xkb::MOD_NAME_CTRL, xkb::STATE_MODS_EFFECTIVE),
            alt: state.mod_name_is_active(xkb::MOD_NAME_ALT, xkb::STATE_MODS_EFFECTIVE),
            shift: state.mod_name_is_active(xkb::MOD_NAME_SHIFT, xkb::STATE_MODS_EFFECTIVE),
            logo: state.mod_name_is_active(xkb::MOD_NAME_LOGO, xkb::STATE_MODS_EFFECTIVE),
        }
    }
}

pub trait KeyboardHandler {
    fn enter(&mut self, serial: u32, surface: WlSurface, keys: Vec<u32>, keysyms: Vec<Keysym>);
    fn leave(&mut self, serial: u32, surface: WlSurface);
    fn key(
        &mut self,
        serial: u32,
        time: u32,
        state: KeyState,
        key: u32,
        keysym: Keysym,
        utf8: String,
    );
    fn modifiers(&mut self, serial: u32, modifiers: Modifiers, group: u32);
}

impl<State> Dispatch<WlKeyboard, AppData, State> for Keyboard
where
    State: Dispatch<WlKeyboard, AppData>,
    State: AsMut<Seat>,
    State: KeyboardHandler,
{
    fn event(
        state: &mut State,
        _proxy: &WlKeyboard,
        event: <WlKeyboard as wayland_client::Proxy>::Event,
        _data: &AppData,
        _conn: &wayland_client::Connection,
        _qhandle: &wayland_client::QueueHandle<State>,
    ) {
        let keyboard = state.as_mut().keyboard.as_mut().unwrap();

        use wl_keyboard::Event;
        match event {
            Event::Keymap { format, fd, size } => match format {
                WEnum::Value(format) => match format {
                    KeymapFormat::NoKeymap => {
                        println!("wl_keyboard: non-xkb ketmap");
                    }
                    KeymapFormat::XkbV1 => {
                        let context = &keyboard.xkb_context;
                        match unsafe {
                            xkb::Keymap::new_from_fd(
                                &context,
                                fd,
                                size as usize,
                                xkb::KEYMAP_FORMAT_TEXT_V1,
                                xkb::CONTEXT_NO_FLAGS,
                            )
                        } {
                            Ok(Some(keymap)) => {
                                keyboard.xkb_state = Some(xkb::State::new(&keymap));
                            }
                            Ok(None) => {
                                panic!("wl_keyboard none keymap");
                            }
                            Err(e) => {
                                panic!("wl_keyboard keymap error: {e}");
                            }
                        }
                    }
                    _ => unreachable!(),
                },
                WEnum::Unknown(value) => {
                    println!("wl_keyboard: unknown format: {value}");
                }
            },
            Event::Enter {
                serial,
                surface,
                keys,
            } => {
                let xkb_state = &keyboard.xkb_state.as_ref().unwrap();

                let keys = keys
                    .chunks_exact(4)
                    .flat_map(TryInto::<[u8; 4]>::try_into)
                    .map(u32::from_le_bytes)
                    .collect::<Vec<_>>();

                let keysyms = keys
                    .iter()
                    .copied()
                    .map(|raw| xkb_state.key_get_one_sym(KeyCode::new(raw + 8)))
                    .collect::<Vec<_>>();

                state.enter(serial, surface, keys, keysyms);
            }
            Event::Leave { serial, surface } => {
                state.leave(serial, surface);
            }
            Event::Key {
                serial,
                time,
                key,
                state: key_state,
            } => match key_state {
                WEnum::Value(key_state) => {
                    let xkb_state = &keyboard.xkb_state.as_ref().unwrap();

                    let keycode = KeyCode::new(key + 8);
                    let keysym = xkb_state.key_get_one_sym(keycode);
                    let utf8 = xkb_state.key_get_utf8(keycode);

                    state.key(serial, time, key_state, key, keysym, utf8);
                }

                WEnum::Unknown(value) => {
                    println!("wl_keyboard: unknown key state: {value}");
                }
            },
            Event::Modifiers {
                serial,
                mods_depressed,
                mods_latched,
                mods_locked,
                group,
            } => {
                let xkb_state = match keyboard.xkb_state.as_mut() {
                    Some(state) => state,
                    None => return,
                };

                xkb_state.update_mask(mods_depressed, mods_latched, mods_locked, 0, 0, group);

                let modifiers = Modifiers::new(xkb_state);
                state.modifiers(serial, modifiers, group);
            }
            Event::RepeatInfo { rate: _, delay: _ } => {}
            _ => unreachable!(),
        }
    }
}
