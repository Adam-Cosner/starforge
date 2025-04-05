use crate::StarforgeState;
use smithay::{delegate_output, wayland::output::OutputHandler};

impl OutputHandler for StarforgeState {}

delegate_output!(StarforgeState);
