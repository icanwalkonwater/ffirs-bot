#![recursion_limit = "1024"]

use log::LevelFilter;

pub mod event_handler;
pub mod module_manager;

#[cfg(debug_assertions)]
pub const LOG_LEVEL: LevelFilter = LevelFilter::Trace;
#[cfg(not(debug_assertions))]
pub const LOG_LEVEL: LevelFilter = LevelFilter::Info;
