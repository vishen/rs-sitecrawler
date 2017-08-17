
pub struct Parser {
    pos: usize,
    chars: Vec<char>,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        let chars = input.chars().collect::<Vec<_>>();
        Parser {
            pos: 0,
            chars: chars,
        }
    }

    // Read the current character without consuming it.
    pub fn peek_char(&mut self) -> char {
        self.chars[self.pos]
    }

    pub fn next_char(&mut self) -> char {

        self.pos += 1;
        self.chars[self.pos - 1]
    }

    pub fn consume_until<F>(&mut self, test_fn: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.finished() && !test_fn(self.peek_char()) {
            result.push(self.next_char());
        }

        result
    }

    pub fn consume_whitespaces(&mut self) {
        let test = |c| match c {
            ' ' | '\t' | '\n' => false,
            _ => true,
        };
        self.consume_until(test);
    }

    pub fn finished(&self) -> bool {
        self.pos >= self.chars.len()
    }
}
