use crate::StarforgeState;
use smithay::delegate_shm;
use smithay::reexports::wayland_server::protocol::wl_buffer::WlBuffer;
use smithay::wayland::buffer::BufferHandler;
use smithay::wayland::shm::{ShmHandler, ShmState};

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
