/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let log_level_str: String = format!("[{:?}]: ", level).to_ascii_uppercase().to_owned();
    return log_level_str + &format!("{message}")
}
pub fn info(message: &str) -> String {
    return log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    return log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    return log(LogLevel::Error, message)
}
