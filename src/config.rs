use clap::ArgMatches;
use std::fmt::Write;

pub struct Config {
    pub debug: Setting<bool>,
}

pub struct PartialConfig {
    pub debug: Option<Setting<bool>>,
}

pub struct Setting<T: std::fmt::Display> {
    pub value: T,
    pub source: Source,
}

pub enum Source {
    Argument,
    Default,
}

pub fn get_configuration(args: &ArgMatches) -> Config {
    let mut config = Config::default();
    let args_partial_config = PartialConfig::from(args);

    config.merge_partial_config(args_partial_config);

    config
}

impl Config {
    pub fn merge_partial_config(&mut self, partial_config: PartialConfig) {
        if let Some(debug_setting) = partial_config.debug {
            self.debug = debug_setting;
        }
    }

    pub fn format_with_sources(&self) -> String {
        let mut output = String::new();
        write!(&mut output, "debug = {}", self.debug.format_with_source()).unwrap();
        output
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "debug = {}", self.debug)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            debug: Setting {
                value: false,
                source: Source::Default,
            },
        }
    }
}

impl From<&ArgMatches> for PartialConfig {
    fn from(args: &ArgMatches) -> Self {
        let debug = if args.get_flag("debug") {
            Some(Setting {
                value: true,
                source: Source::Argument,
            })
        } else {
            None
        };

        PartialConfig { debug }
    }
}

impl<T: std::fmt::Display> Setting<T> {
    pub fn format_with_source(&self) -> String {
        format!("{} ({})", self.value, self.source)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Setting<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Argument => write!(f, "via argument"),
            Self::Default => write!(f, "via default"),
        }
    }
}
