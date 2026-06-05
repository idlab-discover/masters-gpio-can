use crate::can::WasiCanCtxView;
use crate::can::bindings::wasi;
use crate::can::types::{Frame, map_hal_error};
use wasi::can::types::{ErrorCode, Id};

use embedded_can::{Frame as HalFrame, Id as HalId, blocking::Can as HalCan};

pub struct Can {
    pub name: String,
    pub erased_can: Box<dyn ErasedCan + Send>,
}

pub trait ErasedCan {
    fn new_frame(&mut self, id: HalId, data: &[u8]) -> Option<Frame>;
    fn new_remote_frame(&mut self, id: HalId, dlc: usize) -> Option<Frame>;
    fn transmit(&mut self, frame: &Frame, debug: bool) -> Result<(), ErrorCode>;
    fn receive(&mut self, debug: bool) -> Result<Frame, ErrorCode>;
}

impl<T: HalCan> ErasedCan for T {
    fn new_frame(&mut self, id: HalId, data: &[u8]) -> Option<Frame> {
        let hal_frame = T::Frame::new(id, data)?;

        Some(Frame::from_hal(&hal_frame))
    }

    fn new_remote_frame(&mut self, id: HalId, dlc: usize) -> Option<Frame> {
        let hal_frame = T::Frame::new_remote(id, dlc)?;

        Some(Frame::from_hal(&hal_frame))
    }

    fn transmit(&mut self, frame: &Frame, debug: bool) -> Result<(), ErrorCode> {
        let Some(hal_frame) = Frame::to_hal(&frame) else {
            return Err(ErrorCode::Other("Should not fail".to_string()));
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
    fn new_frame(
        &mut self,
        self_: wasmtime::component::Resource<Can>,
        id: Id,
        data: Vec<u8>,
    ) -> wasmtime::Result<Option<wasmtime::component::Resource<Frame>>> {
        let self_ = self.table.get_mut(&self_)?;

        let Some(hal_id) = Id::to_hal_id(&id) else {
            return Ok(None);
        };

        let Some(frame) = self_.erased_can.new_frame(hal_id, &data) else {
            return Ok(None);
        };

        Ok(Some(self.table.push(frame)?))
    }

    fn new_remote_frame(
        &mut self,
        self_: wasmtime::component::Resource<Can>,
        id: wasi::can::blocking::Id,
        dlc: u64,
    ) -> wasmtime::Result<Option<wasmtime::component::Resource<Frame>>> {
        let self_ = self.table.get_mut(&self_)?;

        let Some(hal_id) = Id::to_hal_id(&id) else {
            return Ok(None);
        };

        let Some(frame) = self_.erased_can.new_remote_frame(hal_id, dlc as usize) else {
            return Ok(None);
        };

        Ok(Some(self.table.push(frame)?))
    }

    fn transmit(
        &mut self,
        self_: wasmtime::component::Resource<Can>,
        frame: wasmtime::component::Resource<Frame>,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        let frame = self.table.get(&frame)?.clone();
        let debug = self.ctx.debug;
        let self_ = self.table.get_mut(&self_)?;

        Ok(self_.erased_can.transmit(&frame, debug))
    }

    fn receive(
        &mut self,
        self_: wasmtime::component::Resource<Can>,
    ) -> wasmtime::Result<Result<wasmtime::component::Resource<Frame>, ErrorCode>> {
        let debug = self.ctx.debug;
        let self_ = self.table.get_mut(&self_)?;

        let frame = self_.erased_can.receive(debug);

        match frame {
            Ok(frame) => Ok(Ok(self.table.push(frame)?)),
            Err(err) => Ok(Err(err)),
        }
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<Can>) -> wasmtime::Result<()> {
        let can = self.table.delete(self_)?;
        self.ctx.blocking_can.push((can.name, can.erased_can));
        Ok(())
    }
}
