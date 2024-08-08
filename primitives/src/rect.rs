use crate::{color::Color, vertex::Vertex};
use dagt_core::{Constraints, Draw};
use dagt_gl::gles::{
    func as gl,
    program::{GlProgram, Uniform},
    shader::GlShader,
    types::{GlDrawMode, GlShaderType},
    vertex::GlVArray,
};
use std::{fs, ops::Deref, sync::OnceLock};

pub struct Rect {
    pub bg_color: Color,
    pub bd_color: Color,
    pub bd_width: i32,
    pub bd_radius: i32,
}

impl Default for Rect {
    fn default() -> Self {
        Rect {
            bg_color: Color::white(),
            bd_color: Color::black(),
            bd_width: 0,
            bd_radius: 0,
        }
    }
}

impl Draw for Rect {
    fn draw(&mut self, constraints: Constraints) -> bool {
        static PROG: OnceLock<Prog> = OnceLock::new();
        static VBO: OnceLock<Vec<Vertex>> = OnceLock::new();
        static VAO: OnceLock<GlVArray> = OnceLock::new();

        let prog = PROG.get_or_init(|| Prog::new());
        let vert = VBO.get_or_init(|| {
            vec![
                Vertex::new(-1.0, 1.0),
                Vertex::new(1.0, 1.0),
                Vertex::new(-1.0, -1.0),
                Vertex::new(1.0, -1.0),
            ]
        });
        let vao = VAO.get_or_init(|| {
            let vao = GlVArray::create();
            vao.add_v_buffer(&vert).unwrap();
            vao
        });

        let loc = constraints;

        gl::view_port(loc.x, loc.y, loc.width, loc.height).unwrap();

        vao.bind();
        prog.bind();

        prog.set_uniform("uRes", &[loc.width as f32, loc.height as f32])
            .unwrap();
        prog.set_uniform("uSize", &[loc.width as f32, loc.height as f32])
            .unwrap();

        prog.set_uniform("uBGColor", self.bg_color.as_arr())
            .unwrap();
        prog.set_uniform("uBDColor", self.bd_color.as_arr())
            .unwrap();
        prog.set_uniform("uBDWidth", self.bd_width as f32).unwrap();
        prog.set_uniform("uBDRadius", self.bd_radius as f32)
            .unwrap();

        gl::draw_arrays(GlDrawMode::TriangleStrip, 0, 4).unwrap();

        true
    }
}

struct Prog {
    prog: GlProgram,
}

impl Prog {
    fn new() -> Prog {
        let v_src =
            fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/v.glsl")).unwrap();
        let v = GlShader::create(GlShaderType::Vertex, &v_src).unwrap();
        v.compile().unwrap();

        let f_src = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/f_rect.glsl"))
            .unwrap();
        let f = GlShader::create(GlShaderType::Fragment, &f_src).unwrap();
        f.compile().unwrap();

        let prog = GlProgram::create().unwrap();
        prog.attach(&v).unwrap();
        prog.attach(&f).unwrap();
        prog.link().unwrap();

        Prog { prog }
    }
}

impl Deref for Prog {
    type Target = GlProgram;
    fn deref(&self) -> &Self::Target {
        &self.prog
    }
}
