use socketcan::CanSocket;

use crate::HostComponent;
use crate::types::Frame;
use crate::types::map_hal_error;

use crate::wasi::can::types::ErrorCode;

pub struct Can(pub CanSocket);

impl crate::wasi::can::blocking::Host for HostComponent {}
impl crate::wasi::can::blocking::HostCan for HostComponent {
    fn transmit(
        &mut self,
        self_: wasmtime::component::Resource<Can>,
        frame: wasmtime::component::Resource<Frame>,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        let frame = self.table.get(&frame)?.0;
        let self_ = self.table.get_mut(&self_)?;

        match embedded_can::blocking::Can::transmit(&mut self_.0, &frame) {
            Ok(()) => Ok(Ok(())),
            Err(err) => Ok(Err(map_hal_error(err))),
        }
    }

    fn receive(
        &mut self,
        self_: wasmtime::component::Resource<Can>,
    ) -> wasmtime::Result<Result<wasmtime::component::Resource<Frame>, ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;

        match embedded_can::blocking::Can::receive(&mut self_.0) {
            Ok(frame) => Ok(Ok(self.table.push(Frame(frame))?)),
            Err(err) => Ok(Err(map_hal_error(err))),
        }
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<Can>) -> wasmtime::Result<()> {
        self.table.delete(self_)?;
        Ok(())
    }
}
