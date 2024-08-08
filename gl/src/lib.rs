pub mod egl {
    pub mod binds;
    pub mod error;
    pub mod types;

    pub mod config;
    pub mod context;
    pub mod display;
    pub mod surface;
}

pub mod gles {
    pub mod binds;
    pub mod error;
    pub mod types;

    pub mod func;
    pub mod program;
    pub mod shader;
    pub mod vertex;
}

pub mod error;
pub mod macros;
