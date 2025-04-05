use smithay::{
    backend::{
        renderer::gles::GlesRenderer,
        winit::{self, WinitEvent},
    },
    output::{Mode, Output, PhysicalProperties, Subpixel},
    reexports::calloop::EventLoop,
    utils::Transform,
};
use starforge_core::{StarforgeResult, StarforgeState};

pub fn init_winit(
    event_loop: &mut EventLoop<StarforgeState>,
    state: &mut StarforgeState,
) -> StarforgeResult<()> {
    let display_handle = &mut state.dh;

    // Initialize Winit backend for a test window
    let (backend, winit) = winit::init::<GlesRenderer>()?;

    let mode = Mode {
        size: backend.window_size(),
        refresh: 60_000,
    };

    let output = Output::new(
        "Starforge Test Window".to_string(),
        PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "Starforge".to_string(),
            model: "Winit".to_string(),
        },
    );
    let _global = output.create_global::<StarforgeState>(display_handle);
    output.change_current_state(
        Some(mode),
        Some(Transform::Flipped180),
        None,
        Some((0, 0).into()),
    );
    output.set_preferred(mode);

    event_loop
        .handle()
        .insert_source(winit, move |event, _, state| {
            let display = &mut state.dh;

            match event {
                WinitEvent::Resized { size, .. } => output.change_current_state(
                    Some(Mode {
                        size,
                        refresh: 60_000,
                    }),
                    None,
                    None,
                    None,
                ),
                WinitEvent::Input(event) => {}
                WinitEvent::Redraw => {
                    // Ask for redraw to schedule new frame.
                    backend.window().request_redraw();
                }
                WinitEvent::CloseRequested => state.loop_signal.stop(),
                _ => {}
            }
        })?;

    Ok(())
}
