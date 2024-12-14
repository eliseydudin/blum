use std::{
    process::exit,
    sync::{LazyLock, Mutex, MutexGuard},
};

use super::Exception;

#[derive(Default)]
pub struct Handler {
    source_file: &'static str,
    pub error_counter: usize,
}

static ERROR_LOCK: LazyLock<Mutex<Handler>> = LazyLock::new(|| Mutex::new(Handler::new()));

impl Handler {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            source_file: "none",
            error_counter: 0,
        }
    }

    pub fn lock<'a>() -> MutexGuard<'a, Self> {
        ERROR_LOCK.clear_poison();
        ERROR_LOCK.lock().unwrap()
    }

    pub fn throw<T: Exception>(&mut self, error: &T) {
        self.error_counter += 1;

        if self.error_counter >= 20 {
            eprintln!("too many errors encountered, stopping");
            exit(20)
        }

        let at = error.at();
        eprintln!(
            "{}:{}:{} \x1b[0;31merror:\x1b[0m {error}",
            self.source_file, at.0, at.1
        );
    }
}
