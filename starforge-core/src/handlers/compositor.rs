use crate::StarforgeState;
use crate::state::StarforgeClientState;
use smithay::{
    delegate_compositor,
    reexports::wayland_server::{Client, protocol::wl_surface::WlSurface},
    wayland::compositor::{CompositorClientState, CompositorHandler, CompositorState},
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
