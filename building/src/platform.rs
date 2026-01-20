use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq,ValueEnum)]
pub enum Architecture {
    X64 = 0,
    Arm64
}

impl Default for Architecture{
    fn default() -> Self {
        get_architecture().expect("failed to get current architecture")
    }
}

impl AsRef<str> for Architecture{
    fn as_ref(&self) -> &str {
        match self{
            Architecture::X64 => "x64",
            Architecture::Arm64 => "arm64",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Os{
    Windows = 0,
    Linux,
    MacOS
}

impl Default for Os{
    fn default() -> Self {
        get_os().expect("failed to get current os")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Platform{
    pub os: Option<Os>,
    pub architecture: Option<Architecture>
}

impl Platform{
    pub fn rust_target_triple(&self) -> Option<&'static str> {
        if self.os.is_none() && self.architecture.is_none() {
            return None;
        }

        let os = self.os.unwrap_or_default();
        let architecture = self.architecture.unwrap_or_default();

        Some(match (os, architecture) {
            (Os::MacOS, Architecture::X64) => "x86_64-apple-darwin",
            (Os::MacOS, Architecture::Arm64) => "aarch64-apple-darwin",
            (Os::Linux, Architecture::X64) => "x86_64-unknown-linux-gnu",
            (Os::Linux, Architecture::Arm64) => "aarch64-unknown-linux-gnu",
            (Os::Windows, Architecture::X64) => "x86_64-pc-windows-msvc",
            (Os::Windows, Architecture::Arm64) => "aarch64-pc-windows-msvc",
        })
    }
}

impl AsRef<str> for Os{
    fn as_ref(&self) -> &str {
        match self{
            Os::Windows => "windows",
            Os::Linux => "linux",
            Os::MacOS => "macos",
        }
    }
}

pub fn is_os(os:Os) -> bool{
    let current = ::std::env::consts::OS;

    match os{
        Os::Windows => current == "windows",
        Os::Linux => current == "linux",
        Os::MacOS => current == "macos",
    }
}

pub fn is_architecture(arch:Architecture) -> bool{
    let current = ::std::env::consts::ARCH;

    match arch{
        Architecture::X64 => current == "x86_64",
        Architecture::Arm64 => current == "aarch64",
    }
}

pub fn get_os() -> Option<Os>{
    let current = ::std::env::consts::OS;

    Some(match current{
        "windows" => Os::Windows,
        "linux" => Os::Linux,
        "macos" => Os::MacOS,
        _ => return None
    })
}

pub fn get_architecture() -> Option<Architecture>{
    let current = ::std::env::consts::ARCH;

    Some(match current{
        "x86_64" => Architecture::X64,
        "aarch64" => Architecture::Arm64,
        _ => return None
    })
}
