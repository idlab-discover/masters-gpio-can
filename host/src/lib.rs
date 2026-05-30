pub mod can {
    wasmtime::component::bindgen!({
        path: "../wit/deps/can",
        world: "imports",
        with: {
            "wasi:can/blocking.can": crate::blocking::Can,
            "wasi:can/nonblocking.can": crate::nonblocking::Can,
            "wasi:can/types.frame": crate::types::Frame,
        },
        imports: { default: trappable },
    });
}

pub mod blocking_guest {
    wasmtime::component::bindgen!({
        path: "../wit",
        world: "guest-blocking",
        with: {
            "wasi:can/blocking.can": crate::blocking::Can,
            "wasi:can/types.frame": crate::types::Frame,
        },
    });
}

pub mod nonblocking_guest {
    wasmtime::component::bindgen!({
        path: "../wit",
        world: "guest-nonblocking",
        with: {
            "wasi:can/nonblocking.can": crate::nonblocking::Can,
            "wasi:can/types.frame": crate::types::Frame,
        },
    });
}

mod blocking;
mod nonblocking;
mod types;

use std::path::PathBuf;

use socketcan::Socket;
use wasmtime::{
    Engine, Store,
    component::{Component, HasSelf, Linker, ResourceTable},
    error::Context,
};
use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};

pub struct HostComponent {
    pub(crate) table: ResourceTable,
}

pub struct HostState {
    pub(crate) host: HostComponent,
    pub(crate) ctx: WasiCtx,
}

impl WasiView for HostState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            table: &mut self.host.table,
            ctx: &mut self.ctx,
        }
    }
}

pub fn execute(component: PathBuf) -> Result<(), anyhow::Error> {
    let engine = Engine::default();
    let component = Component::from_file(&engine, component).context("Component file not found")?;
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

    can::Imports::add_to_linker::<_, HasSelf<_>>(&mut linker, |state: &mut HostState| {
        &mut state.host
    })?;
    
    let mut store = Store::new(
        &engine,
        HostState {
            host: HostComponent {
                table: ResourceTable::new(),
            },
            ctx: WasiCtx::builder().inherit_stdio().build(),
        },
    );

    let socket = socketcan::CanSocket::open("can0")?;
    let connection = store.data_mut().host.table.push(crate::blocking::Can(socket))?;

    let guest = blocking_guest::GuestBlocking::instantiate(&mut store, &component, &linker)?;

    guest.call_run(&mut store, connection)?;

    Ok(())
}
