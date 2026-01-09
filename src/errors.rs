use std::fmt;

pub(crate) type Result<T> = std::result::Result<T, FreshfetchError>;

#[derive(Debug)]
pub(crate) enum FreshfetchError {
    Lua(String),
    Command(String, String),
    Io(String, String),
    General(String),
}

impl fmt::Display for FreshfetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FreshfetchError::Lua(details) => write!(f, "A Lua error occurred. Details:\n{}", details),
            FreshfetchError::Command(cmd, details) => write!(f, "An error occurred while executing \"{}\". Details:\n{}", cmd, details),
            FreshfetchError::Io(path, details) => write!(f, "An I/O error occurred while trying to read from \"{}\". Details:\n{}", path, details),
            FreshfetchError::General(details) => write!(f, "An error occurred: {}", details),
        }
    }
}

impl std::error::Error for FreshfetchError {}

pub(crate) fn handle(err: &FreshfetchError) {
    eprintln!("\u{001b}[38;5;1mError.\u{001b}[0m\n{}", err);
    std::process::exit(1);
}
