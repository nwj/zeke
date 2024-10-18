use crate::config::Config;
use anyhow::Result;
use clap::ArgMatches;

pub fn run(args: &ArgMatches, config: Config) -> Result<()> {
    let config = if args.get_flag("show-defaults") {
        Config::default()
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
