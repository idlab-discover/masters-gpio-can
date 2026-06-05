mod bindings;
mod digital;
// mod digital_async;
mod pwm;
mod types;

use embedded_hal::{
    digital::{InputPin, OutputPin, StatefulOutputPin},
    pwm::SetDutyCycle,
};
// use embedded_hal_async::digital::Wait;
use wasmtime::component::{HasData, Linker, ResourceTable};

pub struct WasiGpioCtx {
    pub output_pin: Vec<(String, Box<dyn digital::ErasedOutputPin + Send>)>,
    pub stateful_output_pin: Vec<(String, Box<dyn digital::ErasedStatefulOutputPin + Send>)>,
    pub input_pin: Vec<(String, Box<dyn digital::ErasedInputPin + Send>)>,
    // pub wait_pin: Vec<(String, Box<dyn digital_async::ErasedInputWaitPin + Send>)>,
    pub pwm_pin: Vec<(String, Box<dyn pwm::ErasedPwmPin + Send>)>,
    pub debug: bool,
}

impl WasiGpioCtx {
    pub fn new() -> Self {
        Self {
            output_pin: Vec::new(),
            stateful_output_pin: Vec::new(),
            input_pin: Vec::new(),
            // wait_pin: Vec::new(),
            pwm_pin: Vec::new(),
            debug: false,
        }
    }

    pub fn set_debug_mode(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn add_output_pin<T: OutputPin + Send + 'static>(
        &mut self,
        name: impl Into<String>,
        pin: T,
    ) {
        self.output_pin.push((name.into(), Box::new(pin)));
    }

    pub fn add_stateful_output_pin<T: StatefulOutputPin + Send + 'static>(
        &mut self,
        name: impl Into<String>,
        pin: T,
    ) {
        self.stateful_output_pin.push((name.into(), Box::new(pin)));
    }

    pub fn add_input_pin<T: InputPin + Send + 'static>(&mut self, name: impl Into<String>, pin: T) {
        self.input_pin.push((name.into(), Box::new(pin)));
    }

    // pub fn add_wait_pin<T: Wait + Send + 'static>(&mut self, name: impl Into<String>, pin: T) {
    //     self.wait_pin.push((name.into(), Box::new(pin)));
    // }

    pub fn add_pwm_pin<T: SetDutyCycle + Send + 'static>(
        &mut self,
        name: impl Into<String>,
        pin: T,
    ) {
        self.pwm_pin.push((name.into(), Box::new(pin)));
    }
}

pub struct WasiGpioCtxView<'a> {
    pub ctx: &'a mut WasiGpioCtx,
    pub table: &'a mut ResourceTable,
}

struct WasiGpio;

impl HasData for WasiGpio {
    type Data<'a> = WasiGpioCtxView<'a>;
}

pub trait WasiGpioView {
    fn gpio_ctx(&mut self) -> WasiGpioCtxView<'_>;
}

pub fn add_to_linker<T: WasiGpioView + Send + 'static>(
    linker: &mut Linker<T>,
) -> wasmtime::Result<()> {
    bindings::wasi::gpio::digital::add_to_linker::<T, WasiGpio>(linker, T::gpio_ctx)?;
    // bindings::wasi::gpio::digital_async::add_to_linker::<T, WasiGpio>(linker, T::gpio_ctx)?;
    bindings::wasi::gpio::pwm::add_to_linker::<T, WasiGpio>(linker, T::gpio_ctx)?;
    bindings::wasi::gpio::types::add_to_linker::<T, WasiGpio>(linker, T::gpio_ctx)?;
    Ok(())
}
