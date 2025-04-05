//! Core state management for the Starforge compositor.

use crate::StarforgeResult;
use smithay::{
    desktop::{Space, Window},
    input::SeatState,
    reexports::{
        calloop::{EventLoop, LoopSignal},
        wayland_server::{
            Display, DisplayHandle,
            backend::{ClientData, ClientId, DisconnectReason},
        },
    },
    wayland::{
        compositor::{CompositorClientState, CompositorState},
        selection::data_device::DataDeviceState,
        shell::xdg::XdgShellState,
        shm::ShmState,
    },
};

/// The core state of a Starforge compositor.
///
/// This structure contains all the essential state needed for
/// a functioning Wayland compositor. It can be extended with
/// additional state objects to add functionality.
pub struct StarforgeState {
    /// The Wayland display
    pub dh: DisplayHandle,
    /// Event loop signal
    pub loop_signal: LoopSignal,

    // Smithay state
    pub compositor_state: CompositorState,
    pub data_device_state: DataDeviceState,
    pub xdg_shell_state: XdgShellState,
    pub shm_state: ShmState,
    pub seat_state: SeatState<Self>,
}

impl StarforgeState {
    /// Create a new Starforge state
    pub fn new(event_loop: &EventLoop<Self>) -> StarforgeResult<Self> {
        let display: Display<Self> = Display::new()?;
        let dh = display.handle();

        let loop_signal = event_loop.get_signal();

        let compositor_state = CompositorState::new::<Self>(&dh);
        let data_device_state = DataDeviceState::new::<Self>(&dh);
        let xdg_shell_state = XdgShellState::new::<Self>(&dh);
        let shm_state = ShmState::new::<Self>(&dh, vec![]);
        let seat_state = SeatState::new();

        Ok(Self {
            dh,
            loop_signal,
            compositor_state,
            data_device_state,
            xdg_shell_state,
            shm_state,
            seat_state,
        })
    }

    /// Initialize the event loop
    pub fn init_event_loop<Data>(&mut self, _event_loop: &EventLoop<Data>) -> StarforgeResult<()> {
        Ok(())
    }
}

#[derive(Default)]
pub struct StarforgeClientState {
    pub compositor_state: CompositorClientState,
}

impl ClientData for StarforgeClientState {
    fn initialized(&self, _client_id: ClientId) {}

    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {}
}
