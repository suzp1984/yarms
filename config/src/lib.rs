mod directive;
mod token_stream;

use std::default;

pub struct Directive<'a> {
    name: &'a str,
    args: Vec<&'a str>,
    directives: Vec<Box<Directive<'a>>>,
}

enum DirectiveState {
    Init,
    Entrie,
    BlockStart,
    BlockEnd,
    EOF,
}

impl<'a> Directive<'a> {
    fn parse(mut self, buf: &'a str) {}
}

pub struct Conf<'a> {
    bufs: Vec<String>,
    root: Directive<'a>,
}

impl<'a> Conf<'a> {
    pub fn parse(buf: &str) -> Self {
        let buf = buf.to_owned();
        let mut root = Directive {
            name: "root",
            args: vec![],
            directives: vec![],
        };
        // parse buf
        // root().parse(0, &buf);

        Conf {
            bufs: vec![buf],
            root,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token_stream::*;
}
