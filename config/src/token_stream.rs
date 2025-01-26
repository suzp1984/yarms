#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Token<'a> {
    Val(&'a str),
    LineEnd,
    DirectiveEntire,
    DirectiveBlockStart,
    DirectiveBlockEnd,
    EOF,
}
pub struct TokenStream<'a> {
    buf: &'a str,
    offset: usize,
}

impl<'a> TokenStream<'a> {
    pub fn new(buf: &'a str) -> Self {
        TokenStream { buf, offset: 0 }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        let mut token_start = self.buf.len();
        while self.offset < self.buf.len() {
            let c = self.buf.chars().nth(self.offset).unwrap();
            // space
            match c {
                ' ' | '\t' => {
                    if token_start != self.buf.len() {
                        let v = self.buf.get(token_start..self.offset).unwrap();
                        self.offset += 1;
                        return Token::Val(v);
                    }
                }
                '#' => {
                    self.offset += 1;
                    while self.offset < self.buf.len() {
                        let c = self.buf.chars().nth(self.offset).unwrap();
                        self.offset += 1;
                        match c {
                            '\n' | '\r' => break,
                            _ => {}
                        }
                    }
                }
                ';' => {
                    if token_start != self.buf.len() {
                        let v = self.buf.get(token_start..self.offset).unwrap();
                        return Token::Val(v);
                    }
                    self.offset += 1;
                    return Token::DirectiveEntire;
                }
                '\n' | '\r' => {
                    self.offset += 1;
                    return Token::LineEnd;
                }
                '{' => {
                    if token_start != self.buf.len() {
                        let v = self.buf.get(token_start..self.offset).unwrap();
                        return Token::Val(v);
                    }
                    self.offset += 1;
                    return Token::DirectiveBlockStart;
                }
                '}' => {
                    self.offset += 1;
                    return Token::DirectiveBlockEnd;
                }
                _ => {
                    if token_start == self.buf.len() {
                        token_start = self.offset;
                    }
                }
            }

            self.offset += 1;
        }

        Token::EOF
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.next_token();
        match t {
            Token::EOF => None,
            _ => Some(t),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_token_stream() {
        let mut s = TokenStream::new("key arg0 arg1;");

        assert_eq!(s.next().unwrap(), Token::Val("key"));
        assert_eq!(s.next().unwrap(), Token::Val("arg0"));
        assert_eq!(s.next().unwrap(), Token::Val("arg1"));
        assert_eq!(s.next().unwrap(), Token::DirectiveEntire);
        assert_eq!(s.next(), None);
    }

    #[test]
    fn test_block_token_stream() {
        let mut s = TokenStream::new("key val { hello; }");
        assert_eq!(s.next().unwrap(), Token::Val("key"));
        assert_eq!(s.next().unwrap(), Token::Val("val"));
        assert_eq!(s.next().unwrap(), Token::DirectiveBlockStart);
        assert_eq!(s.next().unwrap(), Token::Val("hello"));
        assert_eq!(s.next().unwrap(), Token::DirectiveEntire);
        assert_eq!(s.next().unwrap(), Token::DirectiveBlockEnd);
        assert_eq!(s.next(), None);
    }
}
