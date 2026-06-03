mod bindings;
mod blocking;
mod nonblocking;
mod types;

use wasmtime::component::{HasData, Linker, ResourceTable};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorDetailPolicy {
    Opaque,
    Debug,
}

pub struct WasiCanCtx {
    pub blocking_can: Vec<(String, Box<dyn blocking::ErasedCan + Send>)>,
    pub nonblocking_can: Vec<(String, Box<dyn nonblocking::ErasedCan + Send>)>,
    pub error_detail_policy: ErrorDetailPolicy,
}

impl WasiCanCtx {
    pub fn new() -> Self {
        Self {
            blocking_can: Vec::new(),
            nonblocking_can: Vec::new(),
            error_detail_policy: ErrorDetailPolicy::Opaque,
        }
    }

    pub fn set_error_detail_policy(&mut self, policy: ErrorDetailPolicy) {
        self.error_detail_policy = policy;
    }

    pub fn add_blocking_can<T>(&mut self, name: impl Into<String>, can: T)
    where
        T: embedded_can::blocking::Can + Send + 'static,
    {
        self.blocking_can.push((name.into(), Box::new(can)));
    }

    pub fn add_nonblocking_can<T>(&mut self, name: impl Into<String>, can: T)
    where
        T: embedded_can::nb::Can + Send + 'static,
    {
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
