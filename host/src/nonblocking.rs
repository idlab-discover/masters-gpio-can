use socketcan::CanSocket;

use crate::HostComponent;
use crate::types::Frame;

use crate::wasi::can::nonblocking::Error;
use crate::wasi::can::types::ErrorCode;

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
            Err(nb::Error::Other(error)) => {
                let error = match embedded_can::Error::kind(&error) {
                    embedded_can::ErrorKind::Overrun => ErrorCode::Overrun,
                    embedded_can::ErrorKind::Bit => ErrorCode::Bit,
                    embedded_can::ErrorKind::Stuff => ErrorCode::Stuff,
                    embedded_can::ErrorKind::Crc => ErrorCode::Crc,
                    embedded_can::ErrorKind::Form => ErrorCode::Form,
                    embedded_can::ErrorKind::Acknowledge => ErrorCode::Acknowledge,
                    embedded_can::ErrorKind::Other => ErrorCode::Other,
                    _ => ErrorCode::Other,
                };

                Ok(Err(Error::Other(error)))
            }
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
            Err(nb::Error::Other(error)) => {
                let error = match embedded_can::Error::kind(&error) {
                    embedded_can::ErrorKind::Overrun => ErrorCode::Overrun,
                    embedded_can::ErrorKind::Bit => ErrorCode::Bit,
                    embedded_can::ErrorKind::Stuff => ErrorCode::Stuff,
                    embedded_can::ErrorKind::Crc => ErrorCode::Crc,
                    embedded_can::ErrorKind::Form => ErrorCode::Form,
                    embedded_can::ErrorKind::Acknowledge => ErrorCode::Acknowledge,
                    embedded_can::ErrorKind::Other => ErrorCode::Other,
                    _ => ErrorCode::Other,
                };

                Ok(Err(Error::Other(error)))
            }
        }
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<Can>) -> wasmtime::Result<()> {
        self.table.delete(self_)?;
        Ok(())
    }
}
