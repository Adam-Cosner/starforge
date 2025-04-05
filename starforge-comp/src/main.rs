//! Starforge Compositor - The reference Starforge compositor implementation

mod winit;

use starforge_config::StarforgeConfig;
use starforge_core::StarforgeState;

use smithay::reexports::calloop::EventLoop;
use std::error::Error;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting Starforge Compositor");

    // Load configuration
    let config = StarforgeConfig::default();
    info!("Configuration loaded");

    // Set up event loop
    let mut event_loop = EventLoop::try_new()?;

    // Initialize compositor state
    let mut compositor_state = StarforgeState::new(&event_loop)?;
    compositor_state.init_event_loop(&mut event_loop)?;
    info!("Compositor state initialized");

    // Initialize Winit backend
    winit::init_winit(&mut event_loop, &mut compositor_state)?;
    info!("Winit backend initialized");

    event_loop.run(None, &mut compositor_state, move |_| {
        // Starforge is running
    })?;

    info!("Starforge compositor exiting");
    Ok(())
}
