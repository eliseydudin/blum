use std::{
    process::exit,
    sync::{LazyLock, Mutex, MutexGuard},
};

pub struct Handler {
    error_counter: usize,
}

static ERROR_LOCK: LazyLock<Mutex<Handler>> = LazyLock::new(|| Mutex::new(Handler::new()));

impl Handler {
    pub fn new() -> Self {
        Self { error_counter: 0 }
    }

    pub fn lock<'a>() -> MutexGuard<'a, Handler> {
        ERROR_LOCK.clear_poison();
        ERROR_LOCK.lock().unwrap()
    }

    pub fn throw(&mut self, error: String) {
        self.error_counter += 1;

        if self.error_counter >= 20 {
            eprintln!("too many errors encountered, stopping");
            exit(20)
        }

        eprintln!("error: {error}");
    }
}
