use super::input_stream::InputStream;

const KEYWORDS: &'static str = " if then else lambda λ true false ";
const ID_KEYWORDS: &'static str = "?!-<>=0123456789";

pub struct TokenStream {
    input: InputStream,
    current: Option<char>,
}

impl TokenStream {
    pub fn new(input: InputStream) -> Self {
        Self {
            input,
            current: None,
        }
    }
}

impl TokenStream {
    pub fn is_keyword(&self, str: String) -> bool {
        let str = format!(" {str} ");
        return KEYWORDS.contains(&str);
    }

    pub fn is_id(&self, id: char) -> bool {
        id.is_ascii_alphabetic() || ID_KEYWORDS.contains(id)
    }
}
