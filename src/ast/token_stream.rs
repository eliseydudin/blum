use super::input_stream::InputStream;

pub struct TokenStream {
    input: InputStream,
}

impl TokenStream {
    pub fn new(input: InputStream) -> Self {
        Self { input }
    }
}

impl TokenStream {}
