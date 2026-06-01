use socketcan::CanSocket;

use crate::HostComponent;
use crate::types::Frame;
use crate::types::map_hal_error;

use crate::wasi::can::nonblocking::Error;

pub struct Can(pub CanSocket);

impl crate::wasi::can::nonblocking::Host for HostComponent {}
impl crate::wasi::can::nonblocking::HostCan for HostComponent {
    fn transmit(
        &mut self,
        self_: wasmtime::component::Resource<Can>,
        frame: wasmtime::component::Resource<Frame>,
    ) -> wasmtime::Result<Result<Option<wasmtime::component::Resource<Frame>>, Error>> {
        let frame = self.table.get(&frame)?.0;
        let self_ = self.table.get_mut(&self_)?;

        match embedded_can::nb::Can::transmit(&mut self_.0, &frame) {
            Ok(Some(replaced_frame)) => Ok(Ok(Some(self.table.push(Frame(replaced_frame))?))),
            Ok(None) => Ok(Ok(None)),
            Err(nb::Error::WouldBlock) => Ok(Err(Error::WouldBlock)),
            Err(nb::Error::Other(err)) => Ok(Err(Error::Other(map_hal_error(err)))),
        }
    }

    fn receive(
        &mut self,
        self_: wasmtime::component::Resource<Can>,
    ) -> wasmtime::Result<Result<wasmtime::component::Resource<Frame>, Error>> {
        let self_ = self.table.get_mut(&self_)?;

        match embedded_can::nb::Can::receive(&mut self_.0) {
            Ok(frame) => Ok(Ok(self.table.push(Frame(frame))?)),
            Err(nb::Error::WouldBlock) => Ok(Err(Error::WouldBlock)),
            Err(nb::Error::Other(err)) => Ok(Err(Error::Other(map_hal_error(err)))),
        }
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<Can>) -> wasmtime::Result<()> {
        self.table.delete(self_)?;
        Ok(())
    }
}
