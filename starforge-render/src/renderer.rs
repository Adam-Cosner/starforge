//! Renderer implementations for Starforge

use crate::RenderError;
use smithay::backend::{
    egl::{EGLContext, EGLError},
    renderer::{
        Renderer,
        gles::{GlesError, GlesRenderer},
    },
};

/// Initialize a GLES renderer
///
/// This is a utility function to create a GLES renderer from an EGL context
pub fn init_gles_renderer(egl_context: EGLContext) -> Result<GlesRenderer, RenderError> {
    let renderer = unsafe { GlesRenderer::new(egl_context)? };
    Ok(renderer)
}
