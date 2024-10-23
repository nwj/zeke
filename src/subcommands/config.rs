use crate::config;
use anyhow::Result;
use clap::ArgMatches;

#[allow(clippy::unnecessary_wraps)]
pub fn run(args: &ArgMatches, config: config::Config) -> Result<()> {
    log::info!("Running the 'config' subcommand");

    let config = if args.get_flag("show-defaults") {
        log::info!("Running with the 'show-defaults' argument");
        config::get_default_configuration()
    } else {
        config
    };

    if args.get_flag("show-sources") {
        log::info!("Running with the 'show-sources' argument");
        println!("{}", config.format_with_sources());
    } else {
        println!("{config}");
    }

    Ok(())
}
