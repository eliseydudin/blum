#[derive(Clone)]
pub struct InputStream {
    pub source: Vec<char>,
    pub line: usize,
    pub col: usize,
    pub pos: usize,
}

impl InputStream {
    pub fn new(source: &str) -> Self {
        let source = source.chars().collect();

        Self {
            source,
            line: 0,
            col: 0,
            pos: 0,
        }
    }

    pub fn eof(&self) -> bool {
        self.peek().is_none()
    }

    pub fn peek(&self) -> Option<char> {
        let s = self.source.get(self.pos);
        s.cloned()
    }
}
impl Iterator for InputStream {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let tk = self.peek();
        self.pos += 1;

        if tk == Some('\n') {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }

        tk
    }
}
