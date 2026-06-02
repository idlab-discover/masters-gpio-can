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

struct HostState {
    table: ResourceTable,
    wasi_ctx: WasiCtx,
    can_ctx: WasiCanCtx,
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

    let state = HostState {
        table: ResourceTable::new(),
        wasi_ctx: WasiCtx::builder().inherit_stdio().build(),
        can_ctx: WasiCanCtx::new(""),
    };

    let mut store = Store::new(&engine, state);
    let mut linker = Linker::new(&engine);

    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    wasmtime_wasi_can::add_to_linker(&mut linker)?;

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
