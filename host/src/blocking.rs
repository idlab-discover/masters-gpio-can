use socketcan::CanSocket;

use crate::HostComponent;
use crate::types::Frame;

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
            Err(error) => {
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

                Ok(Err(error))
            }
        }
    }

    fn receive(
        &mut self,
        self_: wasmtime::component::Resource<Can>,
    ) -> wasmtime::Result<Result<wasmtime::component::Resource<Frame>, ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;

        match embedded_can::blocking::Can::receive(&mut self_.0) {
            Ok(frame) => Ok(Ok(self.table.push(Frame(frame))?)),
            Err(error) => {
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

                Ok(Err(error))
            }
        }
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<Can>) -> wasmtime::Result<()> {
        self.table.delete(self_)?;
        Ok(())
    }
}
