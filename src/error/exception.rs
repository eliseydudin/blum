use core::{error::Error, fmt};

pub trait Exception: Error {
    /// Returns the position at which the error occured
    fn at(&self) -> (usize, usize);
}

#[derive(Debug)]
pub struct SourceException {
    pos: (usize, usize),
    message: String,
}

impl SourceException {
    #[inline]
    #[must_use]
    pub const fn new(pos: (usize, usize), message: String) -> Self {
        Self { pos, message }
    }
}

impl fmt::Display for SourceException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SourceException {}

impl Exception for SourceException {
    fn at(&self) -> (usize, usize) {
        self.pos
    }
}
