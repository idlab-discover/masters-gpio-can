use socketcan::{CanSocket, Socket};
use wasmtime::{
    Store,
    component::{Component, Linker},
};

use crate::HostState;

wasmtime::component::bindgen!({
    path: "../wit",
    world: "guest-blocking",
    with: {
        "wasi:can/blocking.can": crate::blocking::Can,
        "wasi:can/types.frame": crate::types::Frame,
    },
});

pub fn run(
    linker: Linker<HostState>,
    component: Component,
    mut store: Store<HostState>,
) -> Result<(), anyhow::Error> {
    let socket = CanSocket::open("can0")?;
    let connection = store
        .data_mut()
        .host
        .table
        .push(crate::blocking::Can(socket))?;

    let guest = GuestBlocking::instantiate(&mut store, &component, &linker)?;

    guest.call_run(&mut store, connection)?;
    Ok(())
}
