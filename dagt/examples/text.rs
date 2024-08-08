use dagt::prelude::*;

fn main() {
    App::new()
        .id("example.app.text")
        .window(
            Window::new()
                .title("TextWindow")
                .child(
                    Center::new().child(
                        Text::new().text("TouchIng grass Is not enough...\nI need some sleep!"),
                    ),
                )
                .handle_event(|window, event| match event {
                    GlobalEvent::Keyboard(KeyboardEvent::KeyPressed {
                        key: _,
                        keysym,
                        utf8: _,
                    }) => match *keysym {
                        Keysym::q => {
                            window.close();
                            true
                        }
                        _ => false,
                    },
                    _ => false,
                }),
        )
        .exec()
}
