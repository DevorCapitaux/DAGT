use crate::vertex::Vertex;
use dagt_core::{Constraints, Draw};
use dagt_gl::gles::{
    func as gl,
    program::{GlProgram, Uniform},
    shader::GlShader,
    types::{GlDrawMode, GlShaderType},
    vertex::GlVArray,
};
use std::{fs, ops::Deref, sync::OnceLock};

#[derive(Clone, Debug)]
pub struct Path {
    vbo: Vec<Vertex>,
    vao: GlVArray,
    cycled: bool,
}

impl Path {
    pub fn new(path: &[Vertex], cycled: bool) -> Path {
        let vbo: Vec<Vertex> = path.iter().map(|i| i.clone()).collect();
        let vao = GlVArray::create();
        vao.add_v_buffer(&vbo).unwrap();
        Path { vbo, vao, cycled }
    }
}

impl Draw for Path {
    fn draw(&mut self, constraints: Constraints) -> bool {
        static PROG: OnceLock<Prog> = OnceLock::new();
        let prog = PROG.get_or_init(|| Prog::new());

        let vao = self.vao;

        let loc = constraints;
        gl::view_port(loc.x, loc.y, loc.width, loc.height).unwrap();

        vao.bind();
        prog.bind();

        prog.set_uniform("uRes", &[loc.width as f32, loc.height as f32])
            .unwrap();
        prog.set_uniform("uSize", &[loc.width as f32, loc.height as f32])
            .unwrap();

        let mode = if self.cycled {
            GlDrawMode::LineLoop
        } else {
            GlDrawMode::LineStrip
        };

        gl::draw_arrays(mode, 0, self.vbo.len() as i32).unwrap();

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

        let f_src =
            fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/f.glsl")).unwrap();
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
