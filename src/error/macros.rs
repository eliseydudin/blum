#[macro_export]
macro_rules! throw {
    ($l:expr) => {
        $crate::error::Handler::lock().throw($l)
    };
}
