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

mod blocking;
mod blocking_guest;
mod nonblocking;
mod nonblocking_guest;
mod types;

use std::path::PathBuf;

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

#[derive(Clone, clap::ValueEnum)]
pub enum GuestType {
    Blocking,
    Nonblocking,
}

pub fn execute(component: PathBuf, guest_type: GuestType) -> Result<(), anyhow::Error> {
    let engine = Engine::default();
    let component = Component::from_file(&engine, component).context("Component file not found")?;
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;

    Imports::add_to_linker::<_, HasSelf<_>>(&mut linker, |state: &mut HostState| &mut state.host)?;

    let store = Store::new(
        &engine,
        HostState {
            host: HostComponent {
                table: ResourceTable::new(),
            },
            ctx: WasiCtx::builder().inherit_stdio().build(),
        },
    );

    match guest_type {
        GuestType::Blocking => blocking_guest::run(linker, component, store),
        GuestType::Nonblocking => nonblocking_guest::run(linker, component, store),
    }
}
