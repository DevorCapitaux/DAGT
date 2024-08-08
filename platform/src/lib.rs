pub mod desktop {
    pub mod wayland {
        pub mod app;
        pub mod window;
    }

    #[cfg(target_os = "linux")]
    type PlatformApp = wayland::app::WaylandApp;

    #[cfg(target_os = "linux")]
    type PlatformWindow = wayland::window::WaylandWindow;

    pub mod app;
    pub mod event;
    pub mod window;
}
