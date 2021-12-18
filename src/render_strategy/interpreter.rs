use crate::template::Context;

pub fn interpreter(template: String, context: Context) -> String {
    let mut tokenizer = Tokenizer::new("".chars());
    tokenizer.tokenize();
    "".into()
}

struct Tokenizer<T: Iterator<Item = char>> {
    buffer: Vec<char>,
    current_context: TokenizerContext,
    template: T,
    tokens: Vec<Token>,
}

#[derive(Debug, PartialEq)]
enum Token {
    Expression(String),
    Text(String),
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum TokenizerContext {
    ExpressionEnd,
    ExpressionStart,
    Start,
    StatementEnd,
    StatementStart,
}

impl<T: Iterator<Item = char>> Tokenizer<T> {
    fn new(template: T) -> Self {
        Tokenizer {
            buffer: Vec::new(),
            current_context: TokenizerContext::Start,
            template,
            tokens: Vec::new(),
        }
    }

    fn tokenize(&mut self) {
        while let Some(char_value) = self.template.next() {
            self.buffer.push(char_value);
            let context = self.detect_context();
            if context != self.current_context {
                self.current_context = context;
                self.make_token();
            }
        }
        self.make_last_token();
    }

    fn detect_context(&self) -> TokenizerContext {
        match self.last_two_chars() {
            (Some('{'), Some('{')) => TokenizerContext::ExpressionStart,
            (Some('}'), Some('}')) => TokenizerContext::ExpressionEnd,
            (Some('{'), Some('%')) => TokenizerContext::StatementStart,
            (Some('%'), Some('}')) => TokenizerContext::StatementEnd,
            _ => self.current_context,
        }
    }

    fn last_two_chars(&self) -> (Option<&char>, Option<&char>) {
        if self.buffer.len() > 1 {
            let previous = self.buffer.get(self.buffer.len() - 2);
            let current = self.buffer.last();
            (previous, current)
        } else if self.buffer.len() == 1 {
            (None, self.buffer.last())
        } else {
            (None, None)
        }
    }

    fn make_token(&mut self) {
        self.buffer.pop();
        self.buffer.pop();
        let token_value: String = self.buffer.iter().collect();

        if token_value.len() > 0 {
            let token = match self.current_context {
                TokenizerContext::ExpressionStart | TokenizerContext::StatementStart => {
                    Token::Text(token_value)
                }
                TokenizerContext::ExpressionEnd => {
                    let token_value = token_value.trim().into();
                    Token::Expression(token_value)
                }
                TokenizerContext::StatementEnd => {
                    let token_value = token_value.trim().into();
                    Token::Expression(token_value)
                }
                _ => return,
            };

            self.tokens.push(token);
        }

        self.buffer = Vec::new();
    }

    fn make_last_token(&mut self) {
        let buffer_rest: String = self.buffer.iter().collect();
        if buffer_rest.len() > 0 {
            let token = Token::Text(buffer_rest);
            self.tokens.push(token);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_lot_of_combinations() {
        let text = vec!["", "a", "aa", "aaa"];
        let spaces = vec!["", " ", "  ", "   "];
        let mut one_token_combinations: Vec<String> = Vec::new();
        for t in &text {
            for s in &spaces {
                let template = format!("{{{}{}{}}}", s, t, s);
                println!("{}", template);
                one_token_combinations.push(template);
            }
        }
    }

    fn assert_template_has_tokens(template: &'static str, expected_tokens: Vec<Token>) {
        let mut tokenizer = Tokenizer::new(template.chars());
        tokenizer.tokenize();
        assert_eq!(*tokenizer.tokens, expected_tokens);
    }
}
