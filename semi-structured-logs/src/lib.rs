/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}
/// primary function for emitting logs
pub fn log0(level: LogLevel, message: &str) -> String {
    let level = format!("{:?}", level).to_uppercase();
    format!("[{}]: {}", level, message)
}

pub fn log(level: LogLevel, message: &str) -> String {
    let level = match level {
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error => "ERROR",
        LogLevel::Debug => "DEBUG",
    };
    format!("[{}]: {}", level, message)
}

pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
pub fn debug(message: &str) -> String {
    log(LogLevel::Debug, message)
}