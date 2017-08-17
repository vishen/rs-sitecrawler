
pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        Parser {
            pos: 0,
            input: input,
        }
    }

    // Read the current character without consuming it.
    pub fn peek_char(&mut self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    pub fn next_char(&mut self) -> char {
        // Increment to pos by the number of bytes in the codepoint
        // TODO: How does this work?

        // Look at how the following does it, it seems much nice than this?
        // >> https://github.com/pwoolcoc/crafting-interpreters-rust/blob/6d75fe54c58278fffc2213b623103d2673d0e9c1/src/scanner.rs#L16

        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;

        let lower_char = cur_char.to_lowercase().last().unwrap();
        lower_char
    }

    pub fn consume_until<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.finished() && !test(self.peek_char()) {
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
        self.pos >= self.input.len()
    }
}
