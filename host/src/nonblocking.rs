use socketcan::CanSocket;

use crate::HostComponent;

pub struct Can(pub CanSocket);

use crate::can::wasi::can::nonblocking::Error;
use crate::types::Frame;

impl crate::can::wasi::can::nonblocking::Host for HostComponent {}
impl crate::can::wasi::can::nonblocking::HostCan for HostComponent {
    fn transmit(
        &mut self,
        self_: wasmtime::component::Resource<Can>,
        frame: wasmtime::component::Resource<Frame>,
    ) -> wasmtime::Result<Result<Option<wasmtime::component::Resource<Frame>>, Error>> {
        todo!()
    }

    fn receive(
        &mut self,
        self_: wasmtime::component::Resource<Can>,
    ) -> wasmtime::Result<Result<wasmtime::component::Resource<Frame>, Error>> {
        todo!()
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<Can>) -> wasmtime::Result<()> {
        self.table.delete(self_)?;
        Ok(())
    }
}
