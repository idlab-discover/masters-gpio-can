use crate::can::WasiCanCtxView;
use crate::can::bindings::wasi;
use wasi::can::types::{ErrorCode, Frame, FrameKind, Id};

use embedded_can::{
    ExtendedId as HalExtendedId, Frame as HalFrame, Id as HalId, StandardId as HalStandardId,
};

impl Frame {
    pub fn from_hal<T: HalFrame>(frame: &T) -> Self {
        if frame.is_data_frame() {
            let frame_kind = FrameKind::Data(frame.data().to_vec());
            Self {
                id: Id::from_hal_id(&frame.id()),
                kind: frame_kind,
            }
        } else {
            let frame_kind = FrameKind::Remote(
                u32::try_from(frame.dlc()).expect("CAN frame DLC does not fit in u32"),
            );
            Self {
                id: Id::from_hal_id(&frame.id()),
                kind: frame_kind,
            }
        }
    }

    pub fn to_hal<T: HalFrame>(&self) -> Option<T> {
        match &self.kind {
            FrameKind::Data(data) => T::new(self.id.to_hal_id()?, &data),
            FrameKind::Remote(dlc) => {
                T::new_remote(self.id.to_hal_id()?, usize::try_from(*dlc).ok()?)
            }
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

pub fn map_hal_error<E: embedded_can::Error>(err: E, debug: bool) -> ErrorCode {
    match err.kind() {
        embedded_can::ErrorKind::Overrun => ErrorCode::Overrun,
        embedded_can::ErrorKind::Bit => ErrorCode::Bit,
        embedded_can::ErrorKind::Stuff => ErrorCode::Stuff,
        embedded_can::ErrorKind::Crc => ErrorCode::Crc,
        embedded_can::ErrorKind::Form => ErrorCode::Form,
        embedded_can::ErrorKind::Acknowledge => ErrorCode::Acknowledge,
        _ if debug => ErrorCode::Other(format!("{err:?}")),
        _ => ErrorCode::Other("Hardware CAN error".to_string()),
    }
}
