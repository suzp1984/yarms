use crate::token_stream::{Token, TokenStream};
use log::{debug, error};

#[derive(Clone)]
pub struct Directive<'a> {
    name: &'a str,
    args: Vec<&'a str>,
    directives: Vec<Box<Directive<'a>>>,
}

impl<'a> Directive<'a> {
    pub fn new(name: &'a str) -> Self {
        Directive {
            name,
            args: vec![],
            directives: vec![],
        }
    }

    pub fn parse(&mut self, buf: &'a str) {
        let mut token_stream = TokenStream::new(buf);
        let mut blocks: Vec<Directive> = vec![];
        let mut args = vec![];

        for token in token_stream {
            match token {
                Token::Val(v) => {
                    args.push(v);
                }
                Token::DirectiveEntire => {
                    if args.len() > 0 {
                        let directive = Directive {
                            name: args.remove(0),
                            args: args.clone(),
                            directives: vec![],
                        };
                        if let Some(parent_directive) = blocks.pop() {
                            let mut t = parent_directive;
                            t.directives.push(Box::new(directive));
                            blocks.push(t);
                        } else {
                            self.directives.push(Box::new(directive));
                        }

                        args.clear();
                        // TODO: resolve include directive.
                    } else {
                        debug!("args is empty")
                    }
                }
                Token::DirectiveBlockStart => {
                    if args.len() > 0 {
                        let name = args.remove(0);
                        let directive = Directive {
                            name,
                            args: args.clone(),
                            directives: vec![],
                        };

                        blocks.push(directive);
                        args.clear();
                    } else {
                        error!("args is empty before Directive Block.");
                    }
                }
                Token::DirectiveBlockEnd => {
                    if args.len() > 0 {
                        error!("unexpected args before Directive Block end.");
                        args.clear();
                    }

                    if let Some(directive) = blocks.pop() {
                        if let Some(parent_directive) = blocks.pop() {
                            let mut t = parent_directive;
                            t.directives.push(Box::new(directive));
                            blocks.push(t);
                        } else {
                            self.directives.push(Box::new(directive));
                        }
                    } else {
                        panic!("unclosed directive block.");
                    }
                }
                Token::LineEnd => {
                    if args.len() > 0 {
                        error!("unexpected line end!");
                        args.clear();
                    }
                }
                Token::EOF => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_directives() {
        let mut directive = Directive::new("root");
        directive.parse("key arg0 arg1;");
        assert_eq!(directive.name, "root");
        assert_eq!(directive.args.len(), 0);
        assert_eq!(directive.directives.len(), 1);
        assert_eq!(directive.directives[0].name, "key");
        assert_eq!(directive.directives[0].args.len(), 2);
        assert_eq!(directive.directives[0].args[0], "arg0");
        assert_eq!(directive.directives[0].args[1], "arg1");
    }
}
