use super::{binds::*, types::*};
use crate::error::{Error, Result};
use std::ffi::c_void;

pub fn clear_color((r, g, b, a): (u8, u8, u8, u8)) {
    unsafe {
        glClearColor(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        );
    }
}

pub fn clear(mask: GlClearMask) -> Result<()> {
    unsafe {
        glClear(mask.into());
        Error::gl("glClear")
    }
}

pub fn draw_arrays(mode: GlDrawMode, first: i32, count: i32) -> Result<()> {
    unsafe {
        glDrawArrays(mode.into(), first, count);
        Error::gl("glDrawArrays")
    }
}

pub fn draw_elements(mode: GlDrawMode, first: i32, count: i32) -> Result<()> {
    unsafe {
        glDrawElements(
            mode.into(),
            count,
            GlElType::UnsignedInt.into(),
            first as *const c_void,
        );
        Error::gl("glDrawElements")
    }
}

pub fn view_port(x: i32, y: i32, width: i32, height: i32) -> Result<()> {
    unsafe {
        glViewport(x, y, width, height);
        Error::gl("glViewport")
    }
}

pub fn enable(cap: GlCap) {
    unsafe {
        glEnable(cap.into());
    }
}

pub fn disable(cap: GlCap) {
    unsafe {
        glDisable(cap.into());
    }
}

pub fn blend_func(s_fact: GlBlendFact, d_fact: GlBlendFact) {
    unsafe {
        glBlendFunc(s_fact.into(), d_fact.into());
    }
}

pub fn blend_func_separate(
    s_rgb: GlBlendFact,
    d_rgb: GlBlendFact,
    s_alpha: GlBlendFact,
    d_alpha: GlBlendFact,
) {
    unsafe {
        glBlendFuncSeparate(s_rgb.into(), d_rgb.into(), s_alpha.into(), d_alpha.into());
    }
}

pub fn clear_stencil(s: i32) {
    unsafe {
        glClearStencil(s);
    }
}

pub fn color_mask(red: GlBoolean, green: GlBoolean, blue: GlBoolean, alpha: GlBoolean) {
    unsafe {
        glColorMask(red.into(), green.into(), blue.into(), alpha.into());
    }
}

pub fn depth_mask(flag: GlBoolean) {
    unsafe {
        glDepthMask(flag.into());
    }
}

pub fn stencil_mask(mask: u32) {
    unsafe {
        glStencilMask(mask);
    }
}

pub fn stencil_func(func: GlStencilFunc, ref_value: i32, mask: u32) {
    unsafe {
        glStencilFunc(func.into(), ref_value, mask);
        Error::gl("glStencilFunc").unwrap();
    }
}

pub fn stencil_op(sfail: GlStencilOp, dpfail: GlStencilOp, dppass: GlStencilOp) {
    unsafe {
        glStencilOp(sfail.into(), dpfail.into(), dppass.into());
        Error::gl("glStencilOp").unwrap();
    }
}
