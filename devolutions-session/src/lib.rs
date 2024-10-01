#[macro_use]
extern crate tracing;

use ::{ctrlc as _, futures as _};

#[cfg(windows)]
pub mod dvc;

mod config;
mod log;

pub use config::{get_data_dir, ConfHandle};
pub use log::init_log;
