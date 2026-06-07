use crate::can::WasiCanCtxView;
use crate::can::bindings::wasi;
use crate::can::types::map_hal_error;
use wasi::can::nonblocking::{Error, ErrorCode, Frame};

use embedded_can::nb::Can as HalCan;

pub struct Can {
    pub name: String,
    pub erased_can: Box<dyn ErasedCan + Send>,
}

pub trait ErasedCan {
    fn transmit(&mut self, frame: &Frame, debug: bool) -> Result<Option<Frame>, Error>;
    fn receive(&mut self, debug: bool) -> Result<Frame, Error>;
}

impl<T: HalCan> ErasedCan for T {
    fn transmit(&mut self, frame: &Frame, debug: bool) -> Result<Option<Frame>, Error> {
        let Some(hal_frame) = Frame::to_hal(&frame) else {
            return Err(Error::Other(ErrorCode::Other("Invalid frame".to_string())));
        };

        HalCan::transmit(self, &hal_frame)
            .map(|replaced| replaced.map(|frame| Frame::from_hal(&frame)))
            .map_err(|err| match err {
                nb::Error::WouldBlock => Error::WouldBlock,
                nb::Error::Other(err) => Error::Other(map_hal_error(err, debug)),
            })
    }

    fn receive(&mut self, debug: bool) -> Result<Frame, Error> {
        let hal_frame = HalCan::receive(self).map_err(|err| match err {
            nb::Error::WouldBlock => Error::WouldBlock,
            nb::Error::Other(err) => Error::Other(map_hal_error(err, debug)),
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
    fn transmit(
        &mut self,
        self_: wasmtime::component::Resource<Can>,
        frame: Frame,
    ) -> wasmtime::Result<Result<Option<Frame>, Error>> {
        let debug = self.ctx.debug;
        let self_ = self.table.get_mut(&self_)?;

        Ok(self_.erased_can.transmit(&frame, debug))
    }

    fn receive(
        &mut self,
        self_: wasmtime::component::Resource<Can>,
    ) -> wasmtime::Result<Result<Frame, Error>> {
        let debug = self.ctx.debug;
        let self_ = self.table.get_mut(&self_)?;

        Ok(self_.erased_can.receive(debug))
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<Can>) -> wasmtime::Result<()> {
        let can = self.table.delete(self_)?;
        self.ctx.nonblocking_can.push((can.name, can.erased_can));
        Ok(())
    }
}
