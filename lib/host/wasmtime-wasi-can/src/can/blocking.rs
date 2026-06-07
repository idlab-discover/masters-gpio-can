use crate::can::WasiCanCtxView;
use crate::can::bindings::wasi;
use crate::can::types::map_hal_error;
use wasi::can::types::{ErrorCode, Frame};

use embedded_can::blocking::Can as HalCan;

pub struct Can {
    pub name: String,
    pub erased_can: Box<dyn ErasedCan + Send>,
}

pub trait ErasedCan {
    fn transmit(&mut self, frame: &Frame, debug: bool) -> Result<(), ErrorCode>;
    fn receive(&mut self, debug: bool) -> Result<Frame, ErrorCode>;
}

impl<T: HalCan> ErasedCan for T {
    fn transmit(&mut self, frame: &Frame, debug: bool) -> Result<(), ErrorCode> {
        let Some(hal_frame) = Frame::to_hal(&frame) else {
            return Err(ErrorCode::Other("Invalid frame".to_string()));
        };

        HalCan::transmit(self, &hal_frame).map_err(|err| map_hal_error(err, debug))
    }

    fn receive(&mut self, debug: bool) -> Result<Frame, ErrorCode> {
        let hal_frame = HalCan::receive(self).map_err(|err| map_hal_error(err, debug))?;

        Ok(Frame::from_hal(&hal_frame))
    }
}

impl wasi::can::blocking::Host for WasiCanCtxView<'_> {
    fn open(
        &mut self,
        name: String,
    ) -> wasmtime::Result<Result<wasmtime::component::Resource<Can>, ErrorCode>> {
        let Some(index) = self
            .ctx
            .blocking_can
            .iter()
            .position(|named_can| named_can.0 == name)
        else {
            return Ok(Err(ErrorCode::Other("Hardware unavailable".to_string())));
        };

        let (name, erased_can) = self.ctx.blocking_can.remove(index);

        Ok(Ok(self.table.push(Can { name, erased_can })?))
    }
}

impl wasi::can::blocking::HostCan for WasiCanCtxView<'_> {
    fn transmit(
        &mut self,
        self_: wasmtime::component::Resource<Can>,
        frame: Frame,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        let debug = self.ctx.debug;
        let self_ = self.table.get_mut(&self_)?;

        Ok(self_.erased_can.transmit(&frame, debug))
    }

    fn receive(
        &mut self,
        self_: wasmtime::component::Resource<Can>,
    ) -> wasmtime::Result<Result<Frame, ErrorCode>> {
        let debug = self.ctx.debug;
        let self_ = self.table.get_mut(&self_)?;

        Ok(self_.erased_can.receive(debug))
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<Can>) -> wasmtime::Result<()> {
        let can = self.table.delete(self_)?;
        self.ctx.blocking_can.push((can.name, can.erased_can));
        Ok(())
    }
}
