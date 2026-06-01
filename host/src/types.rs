use socketcan::CanFrame;

use crate::HostComponent;

use crate::wasi::can::types::{ErrorCode, Id};

pub struct Frame(pub CanFrame);

impl crate::wasi::can::types::Host for HostComponent {}
impl crate::wasi::can::types::HostFrame for HostComponent {
    fn new(
        &mut self,
        id: Id,
        data: Vec<u8>,
    ) -> wasmtime::Result<Option<wasmtime::component::Resource<Frame>>> {
        let Some(embedded_id) = (match id {
            Id::Standard(raw) => embedded_can::StandardId::new(raw).map(embedded_can::Id::Standard),
            Id::Extended(raw) => embedded_can::ExtendedId::new(raw).map(embedded_can::Id::Extended),
        }) else {
            return Ok(None);
        };

        let Some(frame) = embedded_can::Frame::new(embedded_id, &data) else {
            return Ok(None);
        };

        let resource = self.table.push(Frame(frame))?;
        Ok(Some(resource))
    }

    fn new_remote(
        &mut self,
        id: Id,
        dlc: u64,
    ) -> wasmtime::Result<Option<wasmtime::component::Resource<Frame>>> {
        let Some(embedded_id) = (match id {
            Id::Standard(raw) => embedded_can::StandardId::new(raw).map(embedded_can::Id::Standard),
            Id::Extended(raw) => embedded_can::ExtendedId::new(raw).map(embedded_can::Id::Extended),
        }) else {
            return Ok(None);
        };

        let Ok(dlc) = usize::try_from(dlc) else {
            return Ok(None);
        };

        let Some(frame) = embedded_can::Frame::new_remote(embedded_id, dlc) else {
            return Ok(None);
        };

        let resource = self.table.push(Frame(frame))?;
        Ok(Some(resource))
    }

    fn is_extended(
        &mut self,
        self_: wasmtime::component::Resource<Frame>,
    ) -> wasmtime::Result<bool> {
        let self_ = self.table.get(&self_)?;

        Ok(embedded_can::Frame::is_extended(&self_.0))
    }

    fn is_standard(
        &mut self,
        self_: wasmtime::component::Resource<Frame>,
    ) -> wasmtime::Result<bool> {
        let self_ = self.table.get(&self_)?;

        Ok(embedded_can::Frame::is_standard(&self_.0))
    }

    fn is_remote_frame(
        &mut self,
        self_: wasmtime::component::Resource<Frame>,
    ) -> wasmtime::Result<bool> {
        let self_ = self.table.get(&self_)?;

        Ok(embedded_can::Frame::is_remote_frame(&self_.0))
    }

    fn is_data_frame(
        &mut self,
        self_: wasmtime::component::Resource<Frame>,
    ) -> wasmtime::Result<bool> {
        let self_ = self.table.get(&self_)?;

        Ok(embedded_can::Frame::is_data_frame(&self_.0))
    }

    fn id(&mut self, self_: wasmtime::component::Resource<Frame>) -> wasmtime::Result<Id> {
        let self_ = self.table.get(&self_)?;

        match embedded_can::Frame::id(&self_.0) {
            embedded_can::Id::Standard(embedded_id) => Ok(Id::Standard(embedded_id.as_raw())),
            embedded_can::Id::Extended(embedded_id) => Ok(Id::Extended(embedded_id.as_raw())),
        }
    }

    fn dlc(&mut self, self_: wasmtime::component::Resource<Frame>) -> wasmtime::Result<u64> {
        let self_ = self.table.get(&self_)?;

        Ok(embedded_can::Frame::dlc(&self_.0) as u64)
    }

    fn data(&mut self, self_: wasmtime::component::Resource<Frame>) -> wasmtime::Result<Vec<u8>> {
        let self_ = self.table.get(&self_)?;

        Ok(embedded_can::Frame::data(&self_.0).to_vec())
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<Frame>) -> wasmtime::Result<()> {
        self.table.delete(self_)?;
        Ok(())
    }
}

pub fn map_hal_error<E: embedded_can::Error>(err: E) -> ErrorCode {
    match err.kind() {
        embedded_can::ErrorKind::Overrun => ErrorCode::Overrun,
        embedded_can::ErrorKind::Bit => ErrorCode::Bit,
        embedded_can::ErrorKind::Stuff => ErrorCode::Stuff,
        embedded_can::ErrorKind::Crc => ErrorCode::Crc,
        embedded_can::ErrorKind::Form => ErrorCode::Form,
        embedded_can::ErrorKind::Acknowledge => ErrorCode::Acknowledge,
        embedded_can::ErrorKind::Other => ErrorCode::Other,
        _ => ErrorCode::Other,
    }
}
