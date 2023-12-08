use std::fmt::{Display, Formatter};
use strum_macros::EnumIter;

#[derive(Clone,Copy,EnumIter)]
pub enum LogLevel {
    INFO,
    DEBUG,
    ERROR,
    WARNING,
    NONE
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            LogLevel::INFO => write!(f, "INFO"),
            LogLevel::DEBUG => write!(f, "DEBUG"),
            LogLevel::ERROR => write!(f, "ERROR"),
            LogLevel::WARNING => write!(f, "WARNING"),
            LogLevel::NONE => write!(f, "NONE"),
        }
    }
}