use socketcan::{CanSocket, Socket};
use wasmtime::{
    Store,
    component::{Component, Linker},
};

use crate::HostState;

wasmtime::component::bindgen!({
    path: "../wit",
    world: "guest-nonblocking",
    with: {
        "wasi:can/nonblocking.can": crate::nonblocking::Can,
        "wasi:can/types.frame": crate::types::Frame,
    },
});

pub fn run(
    linker: Linker<HostState>,
    component: Component,
    mut store: Store<HostState>,
) -> Result<(), anyhow::Error> {
    let socket = CanSocket::open("can0")?;
    socket.set_nonblocking(true)?;
    let connection = store
        .data_mut()
        .host
        .table
        .push(crate::nonblocking::Can(socket))?;

    let guest = GuestNonblocking::instantiate(&mut store, &component, &linker)?;

    guest.call_run(&mut store, connection)?;
    Ok(())
}
