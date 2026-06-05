use crate::gpio::WasiGpioCtxView;
use crate::gpio::bindings::wasi;
use wasi::gpio::types::{ErrorCode, PinState};

use embedded_hal::digital::PinState as HalPinState;

impl wasi::gpio::types::Host for WasiGpioCtxView<'_> {}

impl PinState {
    pub fn to_hal(&self) -> HalPinState {
        match self {
            PinState::Low => HalPinState::Low,
            PinState::High => HalPinState::High,
        }
    }
}

pub fn map_digital_hal_error<E: embedded_hal::digital::Error>(err: E, debug: bool) -> ErrorCode {
    match err.kind() {
        _ if debug => ErrorCode::Other(format!("{err:?}")),
        _ => ErrorCode::Other("Hardware GPIO error".to_string()),
    }
}

pub fn map_pwm_hal_error<E: embedded_hal::pwm::Error>(err: E, debug: bool) -> ErrorCode {
    match err.kind() {
        _ if debug => ErrorCode::Other(format!("{err:?}")),
        _ => ErrorCode::Other("Hardware PWM error".to_string()),
    }
}
