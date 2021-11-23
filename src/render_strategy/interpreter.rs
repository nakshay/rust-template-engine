use crate::template::Context;

pub fn interpreter(template: String, context: Context) -> String {
    let mut tokenizer = Tokenizer::new("".chars());
    tokenizer.tokenize();
    "".into()
}

struct Tokenizer<T: Iterator<Item = char>> {
    buffer: (Option<char>, Option<char>, Option<char>),
    current_context: TokenizerContext,
    template: T,
    tokens: Vec<Token>,
}

#[derive(Debug, PartialEq)]
enum Token {
    Expression(String),
    Text(String),
}

#[derive(Debug, PartialEq)]
enum TokenizerContext {
    End,
    Expression,
    None,
    Statement,
    Text,
}

impl<T: Iterator<Item = char>> Tokenizer<T> {
    fn new(template: T) -> Self {
        Tokenizer {
            buffer: (None, None, None),
            current_context: TokenizerContext::None,
            template,
            tokens: Vec::new(),
        }
    }

    fn tokenize(&mut self) {
        self.buffer = (None, self.template.next(), self.template.next());
        while self.current_context != TokenizerContext::End {
            self.current_context = self.detect_context();
            match self.current_context {
                TokenizerContext::Expression => self.tokenize_expression(),
                _ => self.tokenize_text(),
            }
        }
    }

    fn detect_context(&mut self) -> TokenizerContext {
        match (self.buffer.1, self.buffer.2) {
            (_, None) => TokenizerContext::End,
            (Some('{'), Some('{')) => TokenizerContext::Expression,
            (Some('{'), Some('%')) => TokenizerContext::Statement,
            _ => TokenizerContext::Text,
        }
    }

    fn tokenize_expression(&mut self) {
        
    }

    fn advance(&mut self) {
        self.buffer = (self.buffer.1, self.buffer.2, self.template.next());
    }

    fn tokenize_text(&mut self) {
        let mut text = String::new();
        text.push(self.buffer.1.unwrap());
        text.push(self.buffer.2.unwrap());
        while self.detect_context() == TokenizerContext::Text {
            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_variable() {
        assert_template_has_tokens("{{ var }}", vec![Token::Expression("var".into())]);
    }

    #[test]
    fn many_variables() {
        assert_template_has_tokens(
            "{{ var }}{{ var }}{{ var }}",
            vec![
                Token::Expression("var".into()),
                Token::Expression("var".into()),
                Token::Expression("var".into()),
            ],
        );
    }

    #[test]
    fn content_only() {
        assert_template_has_tokens(
            "content 123 !@#",
            vec![Token::Text("content 123 !@#".into())],
        );
    }

    #[test]
    fn simple_template() {
        assert_template_has_tokens(
            "Hello {{ person_name }}!",
            vec![
                Token::Text("Hello ".into()),
                Token::Expression("person_name".into()),
                Token::Text("!".into()),
            ],
        );
    }

    #[test]
    fn tokens_on_both_ends() {
        assert_template_has_tokens(
            "{{ a }} content {{ b }}",
            vec![
                Token::Expression("a".into()),
                Token::Text(" content ".into()),
                Token::Expression("b".into()),
            ],
        );

        assert_template_has_tokens(
            "-{{ a }} content {{ b }}-",
            vec![
                Token::Text("-".into()),
                Token::Expression("a".into()),
                Token::Text(" content ".into()),
                Token::Expression("b".into()),
                Token::Text("-".into()),
            ],
        );

        assert_template_has_tokens(
            "--{{ a }} content {{ b }}--",
            vec![
                Token::Text("--".into()),
                Token::Expression("a".into()),
                Token::Text(" content ".into()),
                Token::Expression("b".into()),
                Token::Text("--".into()),
            ],
        );

        assert_template_has_tokens(
            "---{{ a }} content {{ b }}---",
            vec![
                Token::Text("---".into()),
                Token::Expression("a".into()),
                Token::Text(" content ".into()),
                Token::Expression("b".into()),
                Token::Text("---".into()),
            ],
        );
    }

    fn assert_template_has_tokens(template: &'static str, expected_tokens: Vec<Token>) {
        let mut tokenizer = Tokenizer::new(template.chars());
        tokenizer.tokenize();
        assert_eq!(*tokenizer.tokens, expected_tokens);
    }
}
