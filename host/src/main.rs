use clap::Parser;
use host::GuestType;
use std::path::PathBuf;

#[derive(Parser)]
struct Config {
    /// The path to the component
    #[clap(value_name = "COMPONENT_PATH")]
    component: PathBuf,

    #[clap(value_enum)]
    guest_type: GuestType,
}

fn main() -> Result<(), anyhow::Error> {
    let config = Config::parse();

    host::execute(config.component, config.guest_type)
}
