use crate::gpio::WasiGpioCtxView;
use crate::gpio::bindings::wasi;
use crate::gpio::types::map_pwm_hal_error;
use wasi::gpio::types::ErrorCode;

use embedded_hal::pwm::SetDutyCycle as HalPwmPin;

pub struct PwmPin {
    pub name: String,
    pub erased_pin: Box<dyn ErasedPwmPin + Send>,
}

pub trait ErasedPwmPin {
    fn max_duty_cycle(&self) -> u16;
    fn set_duty_cycle(&mut self, duty: u16, debug: bool) -> Result<(), ErrorCode>;
    fn set_duty_cycle_fully_off(&mut self, debug: bool) -> Result<(), ErrorCode>;
    fn set_duty_cycle_fully_on(&mut self, debug: bool) -> Result<(), ErrorCode>;
    fn set_duty_cycle_fraction(
        &mut self,
        num: u16,
        denom: u16,
        debug: bool,
    ) -> Result<(), ErrorCode>;
    fn set_duty_cycle_percent(&mut self, percent: u8, debug: bool) -> Result<(), ErrorCode>;
}

impl<T: HalPwmPin> ErasedPwmPin for T {
    fn max_duty_cycle(&self) -> u16 {
        HalPwmPin::max_duty_cycle(self)
    }

    fn set_duty_cycle(&mut self, duty: u16, debug: bool) -> Result<(), ErrorCode> {
        HalPwmPin::set_duty_cycle(self, duty).map_err(|err| map_pwm_hal_error(err, debug))
    }

    fn set_duty_cycle_fully_off(&mut self, debug: bool) -> Result<(), ErrorCode> {
        HalPwmPin::set_duty_cycle_fully_off(self).map_err(|err| map_pwm_hal_error(err, debug))
    }

    fn set_duty_cycle_fully_on(&mut self, debug: bool) -> Result<(), ErrorCode> {
        HalPwmPin::set_duty_cycle_fully_on(self).map_err(|err| map_pwm_hal_error(err, debug))
    }

    fn set_duty_cycle_fraction(
        &mut self,
        num: u16,
        denom: u16,
        debug: bool,
    ) -> Result<(), ErrorCode> {
        HalPwmPin::set_duty_cycle_fraction(self, num, denom)
            .map_err(|err| map_pwm_hal_error(err, debug))
    }

    fn set_duty_cycle_percent(&mut self, percent: u8, debug: bool) -> Result<(), ErrorCode> {
        HalPwmPin::set_duty_cycle_percent(self, percent)
            .map_err(|err| map_pwm_hal_error(err, debug))
    }
}

impl wasi::gpio::pwm::Host for WasiGpioCtxView<'_> {
    fn open(
        &mut self,
        name: String,
    ) -> wasmtime::Result<Result<wasmtime::component::Resource<PwmPin>, ErrorCode>> {
        let Some(index) = self
            .ctx
            .pwm_pin
            .iter()
            .position(|named_pin| named_pin.0 == name)
        else {
            return Ok(Err(ErrorCode::Other("Hardware unavailable".to_string())));
        };

        let (name, erased_pin) = self.ctx.pwm_pin.remove(index);

        Ok(Ok(self.table.push(PwmPin { name, erased_pin })?))
    }
}

impl wasi::gpio::pwm::HostPwmPin for WasiGpioCtxView<'_> {
    fn max_duty_cycle(
        &mut self,
        self_: wasmtime::component::Resource<PwmPin>,
    ) -> wasmtime::Result<u16> {
        let self_ = self.table.get_mut(&self_)?;

        Ok(self_.erased_pin.max_duty_cycle())
    }

    fn set_duty_cycle(
        &mut self,
        self_: wasmtime::component::Resource<PwmPin>,
        duty: u16,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let debug = self.ctx.debug;

        Ok(self_.erased_pin.set_duty_cycle(duty, debug))
    }

    fn set_duty_cycle_fully_off(
        &mut self,
        self_: wasmtime::component::Resource<PwmPin>,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let debug = self.ctx.debug;

        Ok(self_.erased_pin.set_duty_cycle_fully_off(debug))
    }

    fn set_duty_cycle_fully_on(
        &mut self,
        self_: wasmtime::component::Resource<PwmPin>,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let debug = self.ctx.debug;

        Ok(self_.erased_pin.set_duty_cycle_fully_on(debug))
    }

    fn set_duty_cycle_fraction(
        &mut self,
        self_: wasmtime::component::Resource<PwmPin>,
        num: u16,
        denom: u16,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let debug = self.ctx.debug;

        Ok(self_.erased_pin.set_duty_cycle_fraction(num, denom, debug))
    }

    fn set_duty_cycle_percent(
        &mut self,
        self_: wasmtime::component::Resource<PwmPin>,
        percent: u8,
    ) -> wasmtime::Result<Result<(), ErrorCode>> {
        let self_ = self.table.get_mut(&self_)?;
        let debug = self.ctx.debug;

        Ok(self_.erased_pin.set_duty_cycle_percent(percent, debug))
    }

    fn drop(&mut self, self_: wasmtime::component::Resource<PwmPin>) -> wasmtime::Result<()> {
        let pin = self.table.delete(self_)?;
        self.ctx.pwm_pin.push((pin.name, pin.erased_pin));
        Ok(())
    }
}
