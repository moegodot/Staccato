use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[value()]
pub enum Configuration {
    Debug,
    Release,
}

impl AsRef<str> for Configuration {
    fn as_ref(&self) -> &str {
        match self {
            Configuration::Debug => "debug",
            Configuration::Release => "release",
        }
    }
}
