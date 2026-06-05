use crate::gpio::WasiGpioCtxView;
use crate::gpio::bindings::wasi;
use crate::gpio::types::map_digital_hal_error;
use wasi::gpio::types::{ErrorCode, PinState};

use embedded_hal::digital::{
    InputPin as HalInputPin, OutputPin as HalOutputPin, PinState as HalPinState,
    StatefulOutputPin as HalStatefulOutputPin,
};

pub struct OutputPin {
    pub name: String,
    pub erased_pin: Box<dyn ErasedOutputPin + Send>,
}

pub struct StatefulOutputPin {
    pub name: String,
    pub erased_pin: Box<dyn ErasedStatefulOutputPin + Send>,
}

pub struct InputPin {
    pub name: String,
    pub erased_pin: Box<dyn ErasedInputPin + Send>,
}

pub trait ErasedOutputPin {
    fn set_low(&mut self, debug: bool) -> Result<(), ErrorCode>;
    fn set_high(&mut self, debug: bool) -> Result<(), ErrorCode>;
    fn set_state(&mut self, state: HalPinState, debug: bool) -> Result<(), ErrorCode>;
}

impl<T: HalOutputPin> ErasedOutputPin for T {
    fn set_low(&mut self, debug: bool) -> Result<(), ErrorCode> {
        HalOutputPin::set_low(self).map_err(|err| map_digital_hal_error(err, debug))
    }

    fn set_high(&mut self, debug: bool) -> Result<(), ErrorCode> {
        HalOutputPin::set_high(self).map_err(|err| map_digital_hal_error(err, debug))
    }

    fn set_state(&mut self, state: HalPinState, debug: bool) -> Result<(), ErrorCode> {
        HalOutputPin::set_state(self, state).map_err(|err| map_digital_hal_error(err, debug))
    }
}

pub trait ErasedStatefulOutputPin: ErasedOutputPin {
    fn is_set_high(&mut self, debug: bool) -> Result<bool, ErrorCode>;
    fn is_set_low(&mut self, debug: bool) -> Result<bool, ErrorCode>;
    fn toggle(&mut self, debug: bool) -> Result<(), ErrorCode>;
}

impl<T: HalStatefulOutputPin> ErasedStatefulOutputPin for T {
    fn is_set_high(&mut self, debug: bool) -> Result<bool, ErrorCode> {
        HalStatefulOutputPin::is_set_high(self).map_err(|err| map_digital_hal_error(err, debug))
    }

    fn is_set_low(&mut self, debug: bool) -> Result<bool, ErrorCode> {
        HalStatefulOutputPin::is_set_low(self).map_err(|err| map_digital_hal_error(err, debug))
    }

    fn toggle(&mut self, debug: bool) -> Result<(), ErrorCode> {
        HalStatefulOutputPin::toggle(self).map_err(|err| map_digital_hal_error(err, debug))
    }
}

pub trait ErasedInputPin {
    fn is_high(&mut self, debug: bool) -> Result<bool, ErrorCode>;
    fn is_low(&mut self, debug: bool) -> Result<bool, ErrorCode>;
}

impl<T: HalInputPin> ErasedInputPin for T {
    fn is_high(&mut self, debug: bool) -> Result<bool, ErrorCode> {
        HalInputPin::is_high(self).map_err(|err| map_digital_hal_error(err, debug))
    }

    fn is_low(&mut self, debug: bool) -> Result<bool, ErrorCode> {
        HalInputPin::is_low(self).map_err(|err| map_digital_hal_error(err, debug))
    }
}

impl wasi::gpio::digital::Host for WasiGpioCtxView<'_> {
    fn open_output(
        &mut self,
        name: String,
    ) -> wasmtime::Result<Result<wasmtime::component::Resource<OutputPin>, ErrorCode>> {
        let Some(index) = self
            .ctx
            .output_pin
            .iter()
            .position(|named_pin| named_pin.0 == name)
        else {
            return Ok(Err(ErrorCode::Other("Hardware unavailable".to_string())));
        };

        let (name, erased_pin) = self.ctx.output_pin.remove(index);

        Ok(Ok(self.table.push(OutputPin { name, erased_pin })?))
    }

    fn open_stateful_output(
        &mut self,
        name: String,
    ) -> wasmtime::Result<Result<wasmtime::component::Resource<StatefulOutputPin>, ErrorCode>> {
        let Some(index) = self
            .ctx
            .stateful_output_pin
            .iter()
            .position(|named_pin| named_pin.0 == name)
        else {
            return Ok(Err(ErrorCode::Other("Hardware unavailable".to_string())));
        };

        let (name, erased_pin) = self.ctx.stateful_output_pin.remove(index);

        Ok(Ok(self
            .table
            .push(StatefulOutputPin { name, erased_pin })?))
    }

    fn open_input(
        &mut self,
        name: String,
    ) -> wasmtime::Result<Result<wasmtime::component::Resource<InputPin>, ErrorCode>> {
        let Some(index) = self
            .ctx
            .input_pin
            .iter()
            .position(|named_pin| named_pin.0 == name)
        else {
            return Ok(Err(ErrorCode::Other("Hardware unavailable".to_string())));
        };

        let (name, erased_pin) = self.ctx.input_pin.remove(index);

        Ok(Ok(self.table.push(InputPin { name, erased_pin })?))
    }
}

impl wasi::gpio::digital::HostOutputPin for WasiGpioCtxView<'_> {
    fn set_low(
        &mut self,
        self_: wasmtime::component::Resource<OutputPin>,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let debug = self.ctx.debug;

        Ok(self_.erased_pin.set_low(debug))
    }

    fn set_high(
        &mut self,
        self_: wasmtime::component::Resource<OutputPin>,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let debug = self.ctx.debug;

        Ok(self_.erased_pin.set_high(debug))
    }

    fn set_state(
        &mut self,
        self_: wasmtime::component::Resource<OutputPin>,
        state: PinState,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let debug = self.ctx.debug;

        Ok(self_.erased_pin.set_state(PinState::to_hal(&state), debug))
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<OutputPin>) -> wasmtime::Result<()> {
        let pin = self.table.delete(self_)?;
        self.ctx.output_pin.push((pin.name, pin.erased_pin));
        Ok(())
    }
}

impl wasi::gpio::digital::HostStatefulOutputPin for WasiGpioCtxView<'_> {
    fn set_low(
        &mut self,
        self_: wasmtime::component::Resource<StatefulOutputPin>,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let debug = self.ctx.debug;

        Ok(self_.erased_pin.set_low(debug))
    }

    fn set_high(
        &mut self,
        self_: wasmtime::component::Resource<StatefulOutputPin>,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let debug = self.ctx.debug;

        Ok(self_.erased_pin.set_high(debug))
    }

    fn set_state(
        &mut self,
        self_: wasmtime::component::Resource<StatefulOutputPin>,
        state: PinState,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let debug = self.ctx.debug;

        Ok(self_.erased_pin.set_state(PinState::to_hal(&state), debug))
    }

    fn is_set_high(
        &mut self,
        self_: wasmtime::component::Resource<StatefulOutputPin>,
    ) -> wasmtime::Result<Result<bool, ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let debug = self.ctx.debug;

        Ok(self_.erased_pin.is_set_high(debug))
    }

    fn is_set_low(
        &mut self,
        self_: wasmtime::component::Resource<StatefulOutputPin>,
    ) -> wasmtime::Result<Result<bool, ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let debug = self.ctx.debug;

        Ok(self_.erased_pin.is_set_low(debug))
    }

    fn toggle(
        &mut self,
        self_: wasmtime::component::Resource<StatefulOutputPin>,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let debug = self.ctx.debug;

        Ok(self_.erased_pin.toggle(debug))
    }

    fn drop(
        &mut self,
        self_: wasmtime::component::Resource<StatefulOutputPin>,
    ) -> wasmtime::Result<()> {
        let pin = self.table.delete(self_)?;
        self.ctx
            .stateful_output_pin
            .push((pin.name, pin.erased_pin));
        Ok(())
    }
}

impl wasi::gpio::digital::HostInputPin for WasiGpioCtxView<'_> {
    fn is_high(
        &mut self,
        self_: wasmtime::component::Resource<InputPin>,
    ) -> wasmtime::Result<Result<bool, ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let debug = self.ctx.debug;

        Ok(self_.erased_pin.is_high(debug))
    }

    fn is_low(
        &mut self,
        self_: wasmtime::component::Resource<InputPin>,
    ) -> wasmtime::Result<Result<bool, ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let debug = self.ctx.debug;

        Ok(self_.erased_pin.is_low(debug))
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<InputPin>) -> wasmtime::Result<()> {
        let pin = self.table.delete(self_)?;
        self.ctx.input_pin.push((pin.name, pin.erased_pin));
        Ok(())
    }
}
