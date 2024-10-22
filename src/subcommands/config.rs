use crate::config;
use anyhow::Result;
use clap::ArgMatches;

#[allow(clippy::unnecessary_wraps)]
pub fn run(args: &ArgMatches, config: config::Config) -> Result<()> {
    let config = if args.get_flag("show-defaults") {
        config::get_default_configuration()
    } else {
        config
    };

    if args.get_flag("show-sources") {
        println!("{}", config.format_with_sources());
    } else {
        println!("{config}");
    }

    Ok(())
}
