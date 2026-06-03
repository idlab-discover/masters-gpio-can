use crate::can::bindings::wasi;
use crate::can::{ErrorDetailPolicy, WasiCanCtxView};
use wasi::can::types::{ErrorCode, Id};

use embedded_can::{
    ExtendedId as HalExtendedId, Frame as HalFrame, Id as HalId, StandardId as HalStandardId,
};

#[derive(Clone)]
pub struct Frame {
    id: HalId,
    data: Vec<u8>,
    dlc: usize,
    is_standard: bool,
    is_data_frame: bool,
}

impl Frame {
    pub fn from_hal<T: HalFrame>(frame: &T) -> Self {
        Self {
            id: frame.id(),
            data: frame.data().to_vec(),
            dlc: frame.dlc(),
            is_standard: frame.is_standard(),
            is_data_frame: frame.is_data_frame(),
        }
    }

    pub fn to_hal<T: HalFrame>(&self) -> Option<T> {
        if self.is_data_frame {
            T::new(self.id, &self.data)
        } else {
            T::new_remote(self.id, self.dlc as usize)
        }
    }
}

impl Id {
    pub fn from_hal_id(id: &HalId) -> Self {
        match id {
            HalId::Standard(embedded_id) => Id::Standard(embedded_id.as_raw()),
            HalId::Extended(embedded_id) => Id::Extended(embedded_id.as_raw()),
        }
    }

    pub fn to_hal_id(&self) -> Option<HalId> {
        match self {
            Id::Standard(embedded_id) => Some(HalId::Standard(HalStandardId::new(*embedded_id)?)),
            Id::Extended(embedded_id) => Some(HalId::Extended(HalExtendedId::new(*embedded_id)?)),
        }
    }
}

impl wasi::can::types::Host for WasiCanCtxView<'_> {}
impl wasi::can::types::HostFrame for WasiCanCtxView<'_> {
    fn is_extended(
        &mut self,
        self_: wasmtime::component::Resource<Frame>,
    ) -> wasmtime::Result<bool> {
        let self_ = self.table.get(&self_)?;

        Ok(!self_.is_standard)
    }

    fn is_standard(
        &mut self,
        self_: wasmtime::component::Resource<Frame>,
    ) -> wasmtime::Result<bool> {
        let self_ = self.table.get(&self_)?;

        Ok(self_.is_standard)
    }

    fn is_remote_frame(
        &mut self,
        self_: wasmtime::component::Resource<Frame>,
    ) -> wasmtime::Result<bool> {
        let self_ = self.table.get(&self_)?;

        Ok(!self_.is_data_frame)
    }

    fn is_data_frame(
        &mut self,
        self_: wasmtime::component::Resource<Frame>,
    ) -> wasmtime::Result<bool> {
        let self_ = self.table.get(&self_)?;

        Ok(self_.is_data_frame)
    }

    fn id(&mut self, self_: wasmtime::component::Resource<Frame>) -> wasmtime::Result<Id> {
        let self_ = self.table.get(&self_)?;

        Ok(Id::from_hal_id(&self_.id))
    }

    fn dlc(&mut self, self_: wasmtime::component::Resource<Frame>) -> wasmtime::Result<u64> {
        let self_ = self.table.get(&self_)?;

        Ok(self_.dlc as u64)
    }

    fn data(&mut self, self_: wasmtime::component::Resource<Frame>) -> wasmtime::Result<Vec<u8>> {
        let self_ = self.table.get(&self_)?;

        Ok(self_.data.clone())
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<Frame>) -> wasmtime::Result<()> {
        self.table.delete(self_)?;
        Ok(())
    }
}

pub fn map_hal_error<E: embedded_can::Error>(err: E, policy: ErrorDetailPolicy) -> ErrorCode {
    match err.kind() {
        embedded_can::ErrorKind::Overrun => ErrorCode::Overrun,
        embedded_can::ErrorKind::Bit => ErrorCode::Bit,
        embedded_can::ErrorKind::Stuff => ErrorCode::Stuff,
        embedded_can::ErrorKind::Crc => ErrorCode::Crc,
        embedded_can::ErrorKind::Form => ErrorCode::Form,
        embedded_can::ErrorKind::Acknowledge => ErrorCode::Acknowledge,
        _ => match policy {
            ErrorDetailPolicy::Opaque => ErrorCode::Other("Hardware CAN error".to_string()),
            ErrorDetailPolicy::Debug => ErrorCode::Other(format!("{err:?}")),
        },
    }
}
