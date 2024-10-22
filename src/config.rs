use clap::ArgMatches;
use std::{env, fmt, fs, path::PathBuf};

pub enum Source {
    Default,
    File(PathBuf),
    EnvVar(String),
    Arg(String),
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Source::Default => write!(f, "default")?,
            Source::File(path) => write!(f, "file: {}", path.display())?,
            Source::EnvVar(var) => write!(f, "environment variable: {var}")?,
            Source::Arg(arg) => write!(f, "argument: --{arg}")?,
        };
        Ok(())
    }
}

pub struct ConfigValue<T> {
    value: T,
    source: Source,
}

impl<T> ConfigValue<T> {
    pub fn new(value: T, source: Source) -> Self {
        Self { value, source }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn source(&self) -> &Source {
        &self.source
    }
}

pub struct Config {
    pub test: ConfigValue<bool>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test = {}", self.test.value())?;
        Ok(())
    }
}

impl Config {
    pub fn format_with_sources(&self) -> String {
        format!("test = {} # via {}", self.test.value(), self.test.source())
    }
}

struct ConfigBuilder {
    test: Option<ConfigValue<bool>>,
}

impl ConfigBuilder {
    fn new() -> Self {
        Self { test: None }
    }

    fn load_defaults(mut self) -> Self {
        self.test = Some(ConfigValue::new(false, Source::Default));
        self
    }

    fn load_file(mut self, path: PathBuf) -> Self {
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(toml) = contents.parse::<toml::Table>() {
                if let Some(test) = toml.get("test").and_then(toml::Value::as_bool) {
                    self.test = Some(ConfigValue::new(test, Source::File(path)));
                }
            }
        }
        self
    }

    fn get_config_file_path() -> Option<PathBuf> {
        if let Ok(p) = env::var("XDG_CONFIG_HOME") {
            Some(PathBuf::from(format!("{p}/zeke/config.toml")))
        } else if let Ok(p) = env::var("HOME") {
            Some(PathBuf::from(format!("{p}/zeke/config.toml")))
        } else {
            None
        }
    }

    fn load_config_file(mut self) -> Self {
        let path = Self::get_config_file_path();
        if let Some(path) = path {
            self = self.load_file(path);
        }
        self
    }

    fn load_env_vars(mut self) -> Self {
        if let Ok(unparsed_val) = env::var("ZEKE_TEST") {
            if let Ok(parsed_val) = unparsed_val.parse::<bool>() {
                self.test = Some(ConfigValue::new(
                    parsed_val,
                    Source::EnvVar("ZEKE_TEST".into()),
                ));
            }
        }
        self
    }

    fn load_args(mut self, args: &ArgMatches) -> Self {
        if args.get_flag("test") {
            self.test = Some(ConfigValue::new(true, Source::Arg("test".into())));
        }
        self
    }

    fn build(self) -> Config {
        Config {
            test: self
                .test
                .expect("missing 'test' ConfigValue while attempting to build Config"),
        }
    }
}

pub fn get_configuration(args: &ArgMatches) -> Config {
    ConfigBuilder::new()
        .load_defaults()
        .load_config_file()
        .load_env_vars()
        .load_args(args)
        .build()
}

pub fn get_default_configuration() -> Config {
    ConfigBuilder::new().load_defaults().build()
}
