use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Config {
    /// The path to the component
    #[clap(value_name = "COMPONENT_PATH")]
    component: PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    let config = Config::parse();

    host::execute(config.component)
}
