use smithay::{
    backend::winit::{self, WinitEventLoop},
    reexports::{calloop, wayland_server},
};
use thiserror::Error;

/// Error type for Starforge
#[derive(Error, Debug)]
pub enum StarforgeError {
    #[error("Winit Error: {0}")]
    WinitError(#[from] winit::Error),
    #[error("Wayland Server Init Error: {0}")]
    WaylandServerInitError(#[from] wayland_server::backend::InitError),
    #[error("Calloop Winit Insert Error: {0}")]
    CalloopWinitInsertError(#[from] calloop::InsertError<WinitEventLoop>),
    #[error("Renderer Error: {0}")]
    RendererError(String),
}

/// Result type for Starforge
pub type StarforgeResult<T> = Result<T, StarforgeError>;
