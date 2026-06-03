use crate::can::bindings::wasi;
use crate::can::types::{Frame, map_hal_error};
use crate::can::{ErrorDetailPolicy, WasiCanCtxView};
use wasi::can::nonblocking::{Error, ErrorCode, Id};

use embedded_can::{Frame as Halframe, Id as HalId, nb::Can as HalCan};

pub struct Can {
    pub name: String,
    pub erased_can: Box<dyn ErasedCan + Send>,
}

pub trait ErasedCan {
    fn new_frame(&mut self, id: HalId, data: &[u8]) -> Option<Frame>;
    fn new_remote_frame(&mut self, id: HalId, dlc: usize) -> Option<Frame>;
    fn transmit(
        &mut self,
        frame: &Frame,
        policy: ErrorDetailPolicy,
    ) -> Result<Option<Frame>, Error>;
    fn receive(&mut self, policy: ErrorDetailPolicy) -> Result<Frame, Error>;
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

    fn transmit(
        &mut self,
        frame: &Frame,
        policy: ErrorDetailPolicy,
    ) -> Result<Option<Frame>, Error> {
        let Some(hal_frame) = Frame::to_hal(&frame) else {
            return Err(Error::Other(ErrorCode::Other(
                "Should not fail".to_string(),
            )));
        };

        HalCan::transmit(self, &hal_frame)
            .map(|replaced| replaced.map(|frame| Frame::from_hal(&frame)))
            .map_err(|err| match err {
                nb::Error::WouldBlock => Error::WouldBlock,
                nb::Error::Other(err) => Error::Other(map_hal_error(err, policy)),
            })
    }

    fn receive(&mut self, policy: ErrorDetailPolicy) -> Result<Frame, Error> {
        let hal_frame = HalCan::receive(self).map_err(|err| match err {
            nb::Error::WouldBlock => Error::WouldBlock,
            nb::Error::Other(err) => Error::Other(map_hal_error(err, policy)),
        })?;

        Ok(Frame::from_hal(&hal_frame))
    }
}

impl wasi::can::nonblocking::Host for WasiCanCtxView<'_> {
    fn open(
        &mut self,
        name: String,
    ) -> wasmtime::Result<Result<wasmtime::component::Resource<Can>, ErrorCode>> {
        let Some(index) = self
            .ctx
            .nonblocking_can
            .iter()
            .position(|named_can| named_can.0 == name)
        else {
            return Ok(Err(ErrorCode::Other("Hardware unavailable".to_string())));
        };

        let (name, erased_can) = self.ctx.nonblocking_can.remove(index);

        Ok(Ok(self.table.push(Can { name, erased_can })?))
    }
}
impl wasi::can::nonblocking::HostCan for WasiCanCtxView<'_> {
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
    ) -> wasmtime::Result<Result<Option<wasmtime::component::Resource<Frame>>, Error>> {
        let frame = self.table.get(&frame)?.clone();
        let policy = self.ctx.error_detail_policy;
        let self_ = self.table.get_mut(&self_)?;

        match self_.erased_can.transmit(&frame, policy) {
            Ok(Some(replaced_frame)) => Ok(Ok(Some(self.table.push(replaced_frame)?))),
            Ok(None) => Ok(Ok(None)),
            Err(err) => Ok(Err(err)),
        }
    }

    fn receive(
        &mut self,
        self_: wasmtime::component::Resource<Can>,
    ) -> wasmtime::Result<Result<wasmtime::component::Resource<Frame>, Error>> {
        let policy = self.ctx.error_detail_policy;
        let self_ = self.table.get_mut(&self_)?;

        let frame = self_.erased_can.receive(policy);

        match frame {
            Ok(frame) => Ok(Ok(self.table.push(frame)?)),
            Err(err) => Ok(Err(err)),
        }
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<Can>) -> wasmtime::Result<()> {
        let can = self.table.delete(self_)?;
        self.ctx.nonblocking_can.push((can.name, can.erased_can));
        Ok(())
    }
}
