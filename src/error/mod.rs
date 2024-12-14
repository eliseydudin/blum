mod handler;
#[macro_use]
mod macros;
mod exception;

use std::process::exit;

pub use exception::{Exception, SourceException};
pub use handler::Handler;

pub fn stop_if_errors_occured() {
    if Handler::lock().error_counter > 0 {
        exit(1);
    }
}
