mod bindings;
mod blocking;
mod nonblocking;
mod types;

use wasmtime::component::{HasData, Linker, ResourceTable};

pub struct WasiCanCtx {}

impl WasiCanCtx {
    pub fn new(_name: &str) -> Self {
        Self {}
    }
}

pub struct WasiCanCtxView<'a> {
    pub ctx: &'a mut WasiCanCtx,
    pub table: &'a mut ResourceTable,
}

struct WasiCan;

impl HasData for WasiCan {
    type Data<'a> = WasiCanCtxView<'a>;
}

pub trait WasiCanView {
    fn can_ctx(&mut self) -> WasiCanCtxView<'_>;
}

pub fn add_to_linker<T: WasiCanView + Send + 'static>(
    linker: &mut Linker<T>,
) -> wasmtime::Result<()> {
    bindings::wasi::can::blocking::add_to_linker::<T, WasiCan>(linker, T::can_ctx)?;
    bindings::wasi::can::nonblocking::add_to_linker::<T, WasiCan>(linker, T::can_ctx)?;
    bindings::wasi::can::types::add_to_linker::<T, WasiCan>(linker, T::can_ctx)?;
    Ok(())
}
