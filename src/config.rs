use clap::ArgMatches;
use indexmap::IndexSet;
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

    fn load_file(mut self, path: PathBuf) -> Self {
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                log::warn!(
                    "Failed to read config file at: {}. Reason: {e}",
                    path.display()
                );
                return self;
            }
        };

        let parsed_content = match content.parse::<toml::Table>() {
            Ok(pc) => pc,
            Err(e) => {
                log::warn!(
                    "Failed to parse config file at: {}. Reason: {e}",
                    path.display()
                );
                return self;
            }
        };

        match parsed_content.get("test").and_then(toml::Value::as_bool) {
            Some(test) => self.test = Some(ConfigValue::new(test, Source::File(path))),
            None => {
                log::info!(
                    "Did not find boolean field 'test' in config file at: {}.",
                    path.display()
                );
            }
        }

        self
    }

    fn get_user_level_config_file_path() -> Option<PathBuf> {
        let possible_paths: IndexSet<PathBuf> = vec![
            env::var("XDG_CONFIG_HOME").ok().and_then(|v| {
                Some(
                    PathBuf::from(v)
                        .canonicalize()
                        .ok()?
                        .join("zeke")
                        .join("config.toml"),
                )
            }),
            env::var("XDG_CONFIG_HOME").ok().and_then(|v| {
                Some(
                    PathBuf::from(v)
                        .canonicalize()
                        .ok()?
                        .join("zeke")
                        .join("config.toml"),
                )
            }),
        ]
        .into_iter()
        .flatten()
        .collect();

        let found_path = possible_paths
            .into_iter()
            .inspect(|path| {
                log::debug!("Checking for user-level config file at: {}", path.display());
            })
            .find(|path| path.is_file());

        if let Some(ref path) = found_path {
            log::info!("Found a user-level config file at: {}", path.display());
        } else {
            log::info!("Did not find a user-level config file");
        }

        found_path
    }

    fn load_user_level_config_file(mut self) -> Self {
        let path = Self::get_user_level_config_file_path();
        if let Some(path) = path {
            self = self.load_file(path);
        }
        self
    }

    fn get_notebook_level_config_file_path() -> Option<PathBuf> {
        const MAX_SEARCH_DEPTH: usize = 25;

        let start_dir = std::env::current_dir().ok()?;

        let possible_paths: IndexSet<PathBuf> =
            std::iter::successors(Some(start_dir.canonicalize().ok()?), |dir| {
                dir.parent().map(std::path::Path::to_path_buf)
            })
            .take(MAX_SEARCH_DEPTH)
            .map(|dir| dir.join(".zeke").join("config.toml"))
            .collect();

        let found_path = possible_paths
            .into_iter()
            .inspect(|path| {
                log::debug!(
                    "Checking for a notebook-level config file at: {}",
                    path.display()
                );
            })
            .find(|path| path.is_file());

        if let Some(ref path) = found_path {
            log::info!("Found a notebook-level config file at: {}", path.display());
        } else {
            log::info!("Did not find a notebook-level config file");
        }

        found_path
    }

    fn load_notebook_level_config_file(mut self) -> Self {
        let path = Self::get_notebook_level_config_file_path();
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
                .unwrap_or(ConfigValue::new(false, Source::Default)),
        }
    }
}

pub fn get_configuration(args: &ArgMatches) -> Config {
    log::debug!("Getting configuration");
    ConfigBuilder::new()
        .load_user_level_config_file()
        .load_notebook_level_config_file()
        .load_env_vars()
        .load_args(args)
        .build()
}

pub fn get_default_configuration() -> Config {
    log::debug!("Getting default configuration");
    ConfigBuilder::new().build()
}
