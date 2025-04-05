use crate::StarforgeState;
use smithay::{
    delegate_xdg_shell,
    reexports::wayland_server::protocol::wl_seat::WlSeat,
    utils::Serial,
    wayland::shell::xdg::{
        PopupSurface, PositionerState, ToplevelSurface, XdgShellHandler, XdgShellState,
    },
};

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
        _surface: PopupSurface,
        _positioner: PositionerState,
        _token: u32,
    ) {
        // Handle reposition requests
    }
}

// Delegate the XDG shell protocol implementation to our handler
delegate_xdg_shell!(StarforgeState);
