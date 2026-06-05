mod bindings;
mod blocking;
mod nonblocking;
mod types;

use embedded_can::{blocking::Can as BlockingCan, nb::Can as NonBlockingCan};
use wasmtime::component::{HasData, Linker, ResourceTable};

pub struct WasiCanCtx {
    pub blocking_can: Vec<(String, Box<dyn blocking::ErasedCan + Send>)>,
    pub nonblocking_can: Vec<(String, Box<dyn nonblocking::ErasedCan + Send>)>,
    pub debug: bool,
}

impl WasiCanCtx {
    pub fn new() -> Self {
        Self {
            blocking_can: Vec::new(),
            nonblocking_can: Vec::new(),
            debug: false,
        }
    }

    pub fn set_debug_mode(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn add_blocking_can<T: BlockingCan + Send + 'static>(
        &mut self,
        name: impl Into<String>,
        can: T,
    ) {
        self.blocking_can.push((name.into(), Box::new(can)));
    }

    pub fn add_nonblocking_can<T: NonBlockingCan + Send + 'static>(
        &mut self,
        name: impl Into<String>,
        can: T,
    ) {
        self.nonblocking_can.push((name.into(), Box::new(can)));
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
