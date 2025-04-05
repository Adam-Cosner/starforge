use crate::StarforgeState;
use smithay::{
    delegate_data_device, delegate_seat,
    input::{SeatHandler, SeatState},
    reexports::wayland_server::protocol::wl_surface::WlSurface,
    wayland::selection::{
        SelectionHandler,
        data_device::{
            ClientDndGrabHandler, DataDeviceHandler, DataDeviceState, ServerDndGrabHandler,
        },
    },
};

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
