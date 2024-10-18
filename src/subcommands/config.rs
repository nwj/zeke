use crate::config::Config;
use anyhow::Result;
use clap::ArgMatches;

pub fn run(args: &ArgMatches, config: &Config) -> Result<()> {
    if args.get_flag("show-sources") {
        println!("{}", config.format_with_sources());
    } else {
        println!("{config}");
    }

    Ok(())
}
