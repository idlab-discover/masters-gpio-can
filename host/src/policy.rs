use anyhow::{Context, Result};
use embedded_hal::digital::PinState;
use gpiocdev_embedded_hal::{InputPin as HalInputPin, OutputPin as HalOutputPin};
use serde::Deserialize;
use socketcan::{CanSocket, Socket};
use sysfs_pwm_embedded_hal::SysfsPwm;
use wasmtime_wasi_can::WasiCanCtx;
use wasmtime_wasi_gpio::WasiGpioCtx;

#[derive(Deserialize, Debug)]
pub struct Policy {
    can: Option<Can>,
    gpio: Option<Gpio>,
}

impl Policy {
    pub fn build_can_context(&self) -> Result<WasiCanCtx> {
        let mut ctx = WasiCanCtx::new();
        let Some(policy) = &self.can else {
            return Ok(ctx);
        };

        ctx.set_debug_mode(policy.debug);

        for can in &policy.blocking {
            let socket = CanSocket::open(&can.ifname).with_context(|| {
                format!("failed to open blocking CAN interface {:?}", can.ifname)
            })?;
            ctx.add_blocking_can(&can.name, socket);
        }

        for can in &policy.nonblocking {
            let socket = CanSocket::open(&can.ifname).with_context(|| {
                format!("failed to open nonblocking CAN interface {:?}", can.ifname)
            })?;
            socket.set_nonblocking(true)?;
            ctx.add_nonblocking_can(&can.name, socket);
        }

        Ok(ctx)
    }

    pub fn build_gpio_context(&self) -> Result<WasiGpioCtx> {
        let mut ctx = WasiGpioCtx::new();
        let Some(policy) = &self.gpio else {
            return Ok(ctx);
        };

        ctx.set_debug_mode(policy.debug);

        for pin in &policy.output_pin {
            let output = HalOutputPin::from_name(&pin.pin_name, PinState::Low)
                .with_context(|| format!("failed to open output pin {:?}", pin.pin_name))?;
            ctx.add_output_pin(&pin.name, output);
        }

        for pin in &policy.stateful_output_pin {
            let output =
                HalOutputPin::from_name(&pin.pin_name, PinState::Low).with_context(|| {
                    format!("failed to open stateful output pin {:?}", pin.pin_name)
                })?;
            ctx.add_stateful_output_pin(&pin.name, output);
        }

        for pin in &policy.input_pin {
            let input = HalInputPin::from_name(&pin.pin_name)
                .with_context(|| format!("failed to open input pin {:?}", pin.pin_name))?;
            ctx.add_input_pin(&pin.name, input);
        }

        for pin in &policy.pwm_pin {
            let pwm = SysfsPwm::new(pin.chip, pin.channel, pin.period_ns).with_context(|| {
                format!(
                    "failed to open PWM chip {} channel {}",
                    pin.chip, pin.channel
                )
            })?;
            ctx.add_pwm_pin(&pin.name, pwm);
        }

        Ok(ctx)
    }
}

#[derive(Deserialize, Debug)]
pub struct Can {
    debug: bool,
    #[serde(default)]
    blocking: Vec<Blocking>,
    #[serde(default)]
    nonblocking: Vec<Nonblocking>,
}

#[derive(Deserialize, Debug)]
pub struct Blocking {
    ifname: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Nonblocking {
    ifname: String,
    name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Gpio {
    debug: bool,
    #[serde(default)]
    output_pin: Vec<OutputPin>,
    #[serde(default)]
    stateful_output_pin: Vec<StatefulOutputPin>,
    #[serde(default)]
    input_pin: Vec<InputPin>,
    #[serde(default)]
    pwm_pin: Vec<PwmPin>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct OutputPin {
    pin_name: String,
    name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct StatefulOutputPin {
    pin_name: String,
    name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct InputPin {
    pin_name: String,
    name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct PwmPin {
    chip: u32,
    channel: u32,
    period_ns: u32,
    name: String,
}
