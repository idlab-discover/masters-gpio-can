use async_trait::async_trait;

use crate::gpio::bindings::wasi;
use crate::gpio::types::map_digital_hal_error;
use crate::gpio::WasiGpioCtxView;
use wasi::gpio::digital_async::WaitError;
use wasi::gpio::types::ErrorCode;
use wasi::io::poll::Pollable;
use wasmtime_wasi_io::poll::{Pollable as HostPollable, subscribe};

use embedded_hal_async::digital::Wait as HalWaitPin;

#[derive(Clone, Copy)]
enum WaitKind {
    High,
    Low,
    RisingEdge,
    FallingEdge,
    AnyEdge,
}

enum WaitState {
    Idle,
    Waiting(WaitKind),
    Finished(Result<(), ErrorCode>),
}

pub struct InputWaitPin {
    pub name: String,
    pub erased_pin: Box<dyn ErasedInputWaitPin + Send>,
    wait_state: WaitState,
    debug: bool,
}

#[async_trait]
pub trait ErasedInputWaitPin: Send {
    async fn wait_for_high(&mut self, debug: bool) -> Result<(), ErrorCode>;
    async fn wait_for_low(&mut self, debug: bool) -> Result<(), ErrorCode>;
    async fn wait_for_rising_edge(&mut self, debug: bool) -> Result<(), ErrorCode>;
    async fn wait_for_falling_edge(&mut self, debug: bool) -> Result<(), ErrorCode>;
    async fn wait_for_any_edge(&mut self, debug: bool) -> Result<(), ErrorCode>;
}

#[async_trait]
impl<T: HalWaitPin + Send> ErasedInputWaitPin for T {
    async fn wait_for_high(&mut self, debug: bool) -> Result<(), ErrorCode> {
        HalWaitPin::wait_for_high(self)
            .await
            .map_err(|err| map_digital_hal_error(err, debug))
    }

    async fn wait_for_low(&mut self, debug: bool) -> Result<(), ErrorCode> {
        HalWaitPin::wait_for_low(self)
            .await
            .map_err(|err| map_digital_hal_error(err, debug))
    }

    async fn wait_for_rising_edge(&mut self, debug: bool) -> Result<(), ErrorCode> {
        HalWaitPin::wait_for_rising_edge(self)
            .await
            .map_err(|err| map_digital_hal_error(err, debug))
    }

    async fn wait_for_falling_edge(&mut self, debug: bool) -> Result<(), ErrorCode> {
        HalWaitPin::wait_for_falling_edge(self)
            .await
            .map_err(|err| map_digital_hal_error(err, debug))
    }

    async fn wait_for_any_edge(&mut self, debug: bool) -> Result<(), ErrorCode> {
        HalWaitPin::wait_for_any_edge(self)
            .await
            .map_err(|err| map_digital_hal_error(err, debug))
    }
}

impl wasi::gpio::digital_async::Host for WasiGpioCtxView<'_> {
    fn open_input_wait(
        &mut self,
        name: String,
    ) -> wasmtime::Result<Result<wasmtime::component::Resource<InputWaitPin>, ErrorCode>> {
        let Some(index) = self
            .ctx
            .wait_pin
            .iter()
            .position(|named_pin| named_pin.0 == name)
        else {
            return Ok(Err(ErrorCode::Other("Hardware unavailable".to_string())));
        };

        let (name, erased_pin) = self.ctx.wait_pin.remove(index);

        Ok(Ok(self.table.push(InputWaitPin {
            name,
            erased_pin,
            wait_state: WaitState::Idle,
            debug: self.ctx.debug,
        })?))
    }
}

impl wasi::gpio::digital_async::HostInputWaitPin for WasiGpioCtxView<'_> {
    fn start_wait_high(
        &mut self,
        self_: wasmtime::component::Resource<InputWaitPin>,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        self.start_wait(self_, WaitKind::High)
    }

    fn start_wait_low(
        &mut self,
        self_: wasmtime::component::Resource<InputWaitPin>,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        self.start_wait(self_, WaitKind::Low)
    }

    fn start_wait_rising_edge(
        &mut self,
        self_: wasmtime::component::Resource<InputWaitPin>,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        self.start_wait(self_, WaitKind::RisingEdge)
    }

    fn start_wait_falling_edge(
        &mut self,
        self_: wasmtime::component::Resource<InputWaitPin>,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        self.start_wait(self_, WaitKind::FallingEdge)
    }

    fn start_wait_any_edge(
        &mut self,
        self_: wasmtime::component::Resource<InputWaitPin>,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        self.start_wait(self_, WaitKind::AnyEdge)
    }

    fn subscribe(
        &mut self,
        self_: wasmtime::component::Resource<InputWaitPin>,
    ) -> wasmtime::Result<wasmtime::component::Resource<Pollable>> {
        subscribe(self.table, self_)
    }

    fn finish_wait(
        &mut self,
        self_: wasmtime::component::Resource<InputWaitPin>,
    ) -> wasmtime::Result<Result<(), WaitError>> {
        let self_ = self.table.get_mut(&self_)?;

        match &self_.wait_state {
            WaitState::Idle => Ok(Err(WaitError::NotInProgress)),
            WaitState::Waiting(_) => Ok(Err(WaitError::WouldBlock)),
            WaitState::Finished(Ok(())) => {
                self_.wait_state = WaitState::Idle;
                Ok(Ok(()))
            }
            WaitState::Finished(Err(err)) => {
                let err = err.clone();
                self_.wait_state = WaitState::Idle;
                Ok(Err(WaitError::Other(err)))
            }
        }
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<InputWaitPin>) -> wasmtime::Result<()> {
        let pin = self.table.delete(self_)?;
        self.ctx.wait_pin.push((pin.name, pin.erased_pin));
        Ok(())
    }
}

impl WasiGpioCtxView<'_> {
    fn start_wait(
        &mut self,
        self_: wasmtime::component::Resource<InputWaitPin>,
        kind: WaitKind,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;

        match self_.wait_state {
            WaitState::Idle => {
                self_.wait_state = WaitState::Waiting(kind);
                Ok(Ok(()))
            }
            _ => Ok(Err(ErrorCode::Other(
                "Wait already in progress".to_string(),
            ))),
        }
    }
}

#[async_trait]
impl HostPollable for InputWaitPin {
    async fn ready(&mut self) {
        let kind = match self.wait_state {
            WaitState::Idle | WaitState::Finished(_) => return,
            WaitState::Waiting(kind) => kind,
        };

        let result = match kind {
            WaitKind::High => self.erased_pin.wait_for_high(self.debug).await,
            WaitKind::Low => self.erased_pin.wait_for_low(self.debug).await,
            WaitKind::RisingEdge => {
                self.erased_pin
                    .wait_for_rising_edge(self.debug)
                    .await
            }
            WaitKind::FallingEdge => {
                self.erased_pin
                    .wait_for_falling_edge(self.debug)
                    .await
            }
            WaitKind::AnyEdge => self.erased_pin.wait_for_any_edge(self.debug).await,
        };

        self.wait_state = WaitState::Finished(result);
    }
}
