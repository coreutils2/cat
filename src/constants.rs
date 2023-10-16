pub const BUFFER_SIZE: usize = 8192;
pub const VERSION_STRING: &'static str = env!("CARGO_PKG_VERSION");

pub const OS: &str = std::env::consts::OS;

pub const WINDOWS_ERR: &str = "Windows stdin in console mode does not support non-UTF-16 input; encountered unpaired surrogate";