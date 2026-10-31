//! `itp config` subcommand — view and manage ITP configuration.

use anyhow::Result;
use itp_core::config::{global_config_path, ItpConfig};
use clap::Args;

#[derive(Args, Debug)]
pub struct ConfigArgs {
    /// Print the effective (merged) configuration.
    #[arg(long, default_value_t = false)]
    pub show: bool,

    /// Initialize a default global config at ~/.atp/config.toml.
    #[arg(long, default_value_t = false)]
    pub init: bool,

    /// Print the path to the global config file.
    #[arg(long, default_value_t = false)]
    pub path: bool,
}

pub fn execute(args: &ConfigArgs) -> Result<()> {
    if args.init {
        let path = ItpConfig::write_default()?;
        println!("Created default config at {}", path.display());
        return Ok(());
    }

    if args.path {
        match global_config_path() {
            Some(p) => println!("{}", p.display()),
            None => println!("Cannot determine home directory"),
        }
        return Ok(());
    }

    // Default: show effective config
    let config = ItpConfig::load()?;
    let toml = config.to_toml()?;
    print!("{toml}");
    Ok(())
}
