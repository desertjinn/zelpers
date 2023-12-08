use std::borrow::BorrowMut;
use std::fmt::{Debug, Display};
use std::sync::Once;

use strum::IntoEnumIterator;

use crate::datastore::DataStore;
use crate::enums::log_level::LogLevel;
use crate::traits::storage::Storage;
use crate::traits::tty::Tty;

/// Ref: https://www.sitepoint.com/rust-global-variables/
static mut LOGLEVEL: LogLevel = LogLevel::NONE;

static INIT: Once = Once::new();

#[derive(Default)]
pub struct Logger {
    pub data_store: Option<DataStore>
}

impl Logger {
    pub fn set_level(&self, cli_option: String) {
        let level: LogLevel = self.translate_log_level(cli_option);

        // FIXME: is a global singleton needed here??
        INIT.call_once(|| {
            // Since this access is inside a call_once, before any other accesses, it is safe
            unsafe {
                *LOGLEVEL.borrow_mut() = level;
            }
        });

        self.print("Log level set");
    }

    fn translate_log_level(&self, cli_option: String) -> LogLevel {
        for level in LogLevel::iter() {
            if cli_option == level.to_string() || cli_option == level.to_string().to_lowercase() {
                return level;
            }
        }

        return LogLevel::NONE;
    }

    fn get_level(&self) -> LogLevel {
        unsafe { LOGLEVEL }
    }
}

// -------------------------------------------------------------------------------------------------

impl Tty for Logger {
    fn print<T: Display+?Sized>(&self, item: &T) {
        println!("{}: {}", self.get_level(), item);
    }
}

// -------------------------------------------------------------------------------------------------

impl Storage for Logger {
    fn save<T>(&self, item: &T) {
        if !self.data_store.is_none()  {
            self.data_store.as_ref().unwrap().save(item);
        }
    }
}

// -------------------------------------------------------------------------------------------------