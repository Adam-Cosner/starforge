//! Wayland protocol handlers for Starforge

use crate::state::{StarforgeClientState, StarforgeState};
use smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer;
use smithay::{
    delegate_compositor, delegate_data_device, delegate_output, delegate_seat, delegate_shm,
    delegate_xdg_shell,
    input::{SeatHandler, SeatState},
    reexports::wayland_server::{
        Client,
        protocol::{wl_seat::WlSeat, wl_surface::WlSurface},
    },
    utils::Serial,
    wayland::{
        buffer::BufferHandler,
        compositor::{CompositorClientState, CompositorHandler, CompositorState},
        output::OutputHandler,
        selection::{
            SelectionHandler,
            data_device::{
                ClientDndGrabHandler, DataDeviceHandler, DataDeviceState, ServerDndGrabHandler,
            },
        },
        shell::xdg::{
            PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
        },
        shm::{ShmHandler, ShmState},
    },
};

/// Implementation of the Wayland compositor protocol
impl CompositorHandler for StarforgeState {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client
            .get_data::<StarforgeClientState>()
            .unwrap()
            .compositor_state
    }

    fn commit(&mut self, surface: &WlSurface) {
        // For now, just log that we got a commit
        tracing::debug!("Surface committed");

        // In a real implementation, we'd update our internal state,
        // mark the surface as needing a redraw, etc.
    }
}

// Delegate the compositor protocol implementation to our handler
delegate_compositor!(StarforgeState);

/// Implementation of the XDG shell protocol
impl XdgShellHandler for StarforgeState {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(&mut self, surface: ToplevelSurface) {
        // Store the new toplevel window in our surface list
        tracing::info!("New toplevel window created");

        // In a real implementation, we'd assign a position, size,
        // and add the surface to our rendering list
    }

    fn new_popup(&mut self, _surface: PopupSurface, _positioner: PositionerState) {
        // Handle popup windows
        tracing::info!("New popup created");
    }

    fn grab(&mut self, _surface: PopupSurface, _seat: WlSeat, _serial: Serial) {
        // Handle popup grabs
    }

    fn reposition_request(
        &mut self,
        surface: PopupSurface,
        positioner: PositionerState,
        token: u32,
    ) {
        // Handle reposition requests
    }
}

// Delegate the XDG shell protocol implementation to our handler
delegate_xdg_shell!(StarforgeState);

// Wl Seat

impl SeatHandler for StarforgeState {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;
    type TouchFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }
}

delegate_seat!(StarforgeState);

impl ClientDndGrabHandler for StarforgeState {}

impl ServerDndGrabHandler for StarforgeState {}

impl DataDeviceHandler for StarforgeState {
    fn data_device_state(&self) -> &DataDeviceState {
        &self.data_device_state
    }
}

impl SelectionHandler for StarforgeState {
    type SelectionUserData = ();
}

// Delegate the data device implementation to our handler
delegate_data_device!(StarforgeState);

/// Implementation of the SHM protocol
impl ShmHandler for StarforgeState {
    fn shm_state(&self) -> &ShmState {
        &self.shm_state
    }
}

/// Implementation of buffer handler
impl BufferHandler for StarforgeState {
    fn buffer_destroyed(&mut self, buffer: &WlBuffer) {}
}

// Delegate the shm state implementation to our handler
delegate_shm!(StarforgeState);

// Wl Output
impl OutputHandler for StarforgeState {}

delegate_output!(StarforgeState);
