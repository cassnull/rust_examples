// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    #[cfg(feature = "add-a-variant")]
    Debug,
}

/// primary function for emitting logs
/// return a message for the given log level
pub fn log(level: LogLevel, message: &str) -> String {
    let level = match level {
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error => "ERROR",
        #[cfg(feature = "add-a-variant")]
        LogLevel::Debug => "DEBUG",
    };
    format!("[{level}]: {message}")
}

/// return a message for info log level.
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}

/// return a message for warn log level.
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}

/// return a message for error log level.
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}

/// return a message for debug log level.
#[cfg(feature = "add-a-variant")]
pub fn debug(message: &str) -> String {
    log(LogLevel::Debug, message)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emits_info() {
        assert_eq!(info("Timezone changed"), "[INFO]: Timezone changed");
    }

    #[test]
    fn emits_warning() {
        assert_eq!(warn("Timezone not set"), "[WARNING]: Timezone not set");
    }

    #[test]
    fn emits_error() {
        assert_eq!(error("Disk full"), "[ERROR]: Disk full");
    }

    #[test]
    fn log_emits_info() {
        assert_eq!(
            log(LogLevel::Info, "Timezone changed"),
            "[INFO]: Timezone changed"
        );
    }

    #[test]
    fn log_emits_warning() {
        assert_eq!(
            log(LogLevel::Warning, "Timezone not set"),
            "[WARNING]: Timezone not set"
        );
    }

    #[test]
    fn log_emits_error() {
        assert_eq!(log(LogLevel::Error, "Disk full"), "[ERROR]: Disk full");
    }

    #[test]
    #[cfg(feature = "add-a-variant")]
    fn add_a_variant() {
        // this test won't even compile until the enum is complete, which is why it is feature-gated.
        assert_eq!(
            log(LogLevel::Debug, "reached line 123"),
            "[DEBUG]: reached line 123",
        );
    }
}
