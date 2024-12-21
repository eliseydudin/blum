use std::{cell::Cell, process::exit, sync::OnceLock};

#[derive(Clone)]
pub struct Handler {
    source_file: String,
    error_counter: Cell<usize>,
}

unsafe impl Sync for Handler {}

static HANDLER_LOCK: OnceLock<Handler> = OnceLock::new();

impl Handler {
    pub fn new(source_file: String) -> Self {
        Self {
            source_file,
            error_counter: Cell::new(0),
        }
    }

    pub fn error(pos: usize, message: impl Into<String>) {
        let message: String = message.into();

        match HANDLER_LOCK.get() {
            Some(handle) => {
                println!("{}:{pos} error: {message}", handle.source_file);
                handle.error_counter.set(handle.error_counter.get() + 1);

                if handle.error_counter.get() == 20 {
                    exit(20);
                }
            }
            None => {
                println!("blum: error: {message}")
            }
        }
    }

    pub fn set_source_file(path: String) {
        HANDLER_LOCK.get_or_init(|| Handler::new(path));
    }
}
