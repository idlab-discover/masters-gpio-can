use clap::{Parser, Subcommand};
use std::path::PathBuf;

use wasmtime::{
    Engine, Store,
    component::{Component, Linker, ResourceTable},
    error::Context,
};
use wasmtime_wasi::p2::bindings::sync::Command;
use wasmtime_wasi::{WasiCtx, WasiCtxView, WasiView};
use wasmtime_wasi_can::{WasiCanCtx, WasiCanCtxView, WasiCanView};
use wasmtime_wasi_gpio::{WasiGpioCtx, WasiGpioCtxView, WasiGpioView};

use embedded_hal::digital::PinState;
use gpiocdev_embedded_hal::{InputPin, OutputPin};
use socketcan::{CanSocket, Socket};
use sysfs_pwm_embedded_hal::SysfsPwm;

struct HostState {
    table: ResourceTable,
    wasi_ctx: WasiCtx,
    can_ctx: WasiCanCtx,
    gpio_ctx: WasiGpioCtx,
}

impl WasiView for HostState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.table,
        }
    }
}

impl WasiCanView for HostState {
    fn can_ctx(&mut self) -> WasiCanCtxView<'_> {
        WasiCanCtxView {
            ctx: &mut self.can_ctx,
            table: &mut self.table,
        }
    }
}

impl WasiGpioView for HostState {
    fn gpio_ctx(&mut self) -> WasiGpioCtxView<'_> {
        WasiGpioCtxView {
            ctx: &mut self.gpio_ctx,
            table: &mut self.table,
        }
    }
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs a WebAssembly component
    Run {
        /// Path to the WebAssembly component (.wasm file)
        #[clap(value_name = "WASM")]
        component: PathBuf,
    },
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();

    let engine = Engine::default();

    let blocking_socket = CanSocket::open("can0")?;
    let nonblocking_socket = CanSocket::open("can0")?;
    let mut can_ctx = WasiCanCtx::new();
    can_ctx.add_blocking_can("can", blocking_socket);
    can_ctx.add_nonblocking_can("can", nonblocking_socket);

    let output_pin = OutputPin::from_name("GPIO2", PinState::Low)?;
    let input_pin = InputPin::from_name("GPIO3")?;
    let pwm_pin = SysfsPwm::new(0, 0, 20_000_000)?;
    let mut gpio_ctx = WasiGpioCtx::new();
    gpio_ctx.add_output_pin("pin2", output_pin);
    gpio_ctx.add_input_pin("pin3", input_pin);
    gpio_ctx.add_pwm_pin("pwm", pwm_pin);

    let state = HostState {
        table: ResourceTable::new(),
        wasi_ctx: WasiCtx::builder().inherit_stdio().build(),
        can_ctx: can_ctx,
        gpio_ctx: gpio_ctx,
    };

    let mut store = Store::new(&engine, state);
    let mut linker = Linker::new(&engine);

    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    wasmtime_wasi_can::add_to_linker(&mut linker)?;
    wasmtime_wasi_gpio::add_to_linker(&mut linker)?;

    match cli.command {
        Commands::Run { component } => {
            let component =
                Component::from_file(&engine, component).context("Component file not found")?;

            let command = Command::instantiate(&mut store, &component, &linker)?;
            command
                .wasi_cli_run()
                .call_run(&mut store)
                .context("Failed to run component")?
                .map_err(|()| anyhow::anyhow!("Component exited with an error"))
        }
    }
}
