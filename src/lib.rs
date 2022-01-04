pub fn render(template: String) -> String {
    let mut tokenizer = Tokenizer::new(template.chars());
    tokenizer.tokenize();
    "".into()
}

struct Tokenizer<T: Iterator<Item = char>> {
    buffer: Vec<char>,
    curr_context: TokenizerContext,
    prev_markers: (Option<char>, Option<char>),
    template: T,
    tokens: Vec<Token>,
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Expression(String),
    Statement(String),
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
            curr_context: TokenizerContext::Start,
            prev_markers: (None, None),
            template,
            tokens: Vec::new(),
        }
    }

    fn tokenize(&mut self) {
        while let Some(char_value) = self.template.next() {
            self.buffer.push(char_value);
            let context = self.detect_context();
            if context != self.curr_context {
                self.curr_context = context;
                self.make_token();
            }
        }

        self.make_last_token();
    }

    fn detect_context(&self) -> TokenizerContext {
        let last_two_chars = {
            if self.buffer.len() > 1 {
                let previous = self.buffer.get(self.buffer.len() - 2);
                let current = self.buffer.last();
                (previous, current)
            } else if self.buffer.len() == 1 {
                (None, self.buffer.last())
            } else {
                (None, None)
            }
        };

        match last_two_chars {
            (Some('{'), Some('{')) => TokenizerContext::ExpressionStart,
            (Some('}'), Some('}')) => TokenizerContext::ExpressionEnd,
            (Some('{'), Some('%')) => TokenizerContext::StatementStart,
            (Some('%'), Some('}')) => TokenizerContext::StatementEnd,
            _ => self.curr_context,
        }
    }

    fn make_token(&mut self) {
        let curr_2nd_marker = self.buffer.pop().unwrap();
        let curr_1st_marker = self.buffer.pop().unwrap();
        let token_value: String = self.buffer.iter().collect();

        if token_value.trim().len() > 0 {
            self.tokens.push(match self.curr_context {
                TokenizerContext::ExpressionStart => Token::Text(token_value),
                TokenizerContext::ExpressionEnd => Token::Expression(token_value.trim().into()),
                TokenizerContext::StatementStart => Token::Text(token_value),
                TokenizerContext::StatementEnd => Token::Statement(token_value.trim().into()),
                _ => return,
            });
        }

        self.prev_markers = (Some(curr_1st_marker), Some(curr_2nd_marker));
        self.buffer = Vec::new();
    }

    fn make_last_token(&mut self) {
        let mut rest = String::new();

        if self.curr_context == TokenizerContext::ExpressionStart || self.curr_context == TokenizerContext::StatementStart {
            rest = format!("{}{}{}", rest, self.prev_markers.0.unwrap(), self.prev_markers.1.unwrap());
        }

        if self.buffer.len() > 0 {
            rest = format!("{}", self.buffer.iter().collect::<String>());
        }

        if rest.len() > 0 {
            self.tokens.push(Token::Text(rest));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum FormsOf<'a> {
        Expressions(&'a [(&'static str, Token); 18]),
        Texts(&'a [&'static str; 2]),
    }

    #[test]
    fn test_many_combinations() {
        let texts = ["a", "ab"];
        let expressions = [
            ("{{a}}", Token::Expression("a".into())),
            ("{{a }}", Token::Expression("a".into())),
            ("{{a  }}", Token::Expression("a".into())),
            ("{{ a}}", Token::Expression("a".into())),
            ("{{ a }}", Token::Expression("a".into())),
            ("{{ a  }}", Token::Expression("a".into())),
            ("{{  a}}", Token::Expression("a".into())),
            ("{{  a }}", Token::Expression("a".into())),
            ("{{  a  }}", Token::Expression("a".into())),
            ("{{aa}}", Token::Expression("aa".into())),
            ("{{aa }}", Token::Expression("aa".into())),
            ("{{aa  }}", Token::Expression("aa".into())),
            ("{{ aa}}", Token::Expression("aa".into())),
            ("{{ aa }}", Token::Expression("aa".into())),
            ("{{ aa  }}", Token::Expression("aa".into())),
            ("{{  aa}}", Token::Expression("aa".into())),
            ("{{  aa }}", Token::Expression("aa".into())),
            ("{{  aa  }}", Token::Expression("aa".into())),
        ];

        let combinations_to_test = [
            [None, None, Some(FormsOf::Texts(&texts))],
            [None, None, Some(FormsOf::Expressions(&expressions))],
            [
                None,
                Some(FormsOf::Texts(&texts)),
                Some(FormsOf::Expressions(&expressions)),
            ],
            [
                None,
                Some(FormsOf::Expressions(&expressions)),
                Some(FormsOf::Texts(&texts)),
            ],
            [
                None,
                Some(FormsOf::Expressions(&expressions)),
                Some(FormsOf::Expressions(&expressions)),
            ],
            [
                Some(FormsOf::Texts(&texts)),
                Some(FormsOf::Expressions(&expressions)),
                Some(FormsOf::Texts(&texts)),
            ],
            [
                Some(FormsOf::Texts(&texts)),
                Some(FormsOf::Expressions(&expressions)),
                Some(FormsOf::Expressions(&expressions)),
            ],
            [
                Some(FormsOf::Expressions(&expressions)),
                Some(FormsOf::Texts(&texts)),
                Some(FormsOf::Expressions(&expressions)),
            ],
            [
                Some(FormsOf::Expressions(&expressions)),
                Some(FormsOf::Expressions(&expressions)),
                Some(FormsOf::Texts(&texts)),
            ],
            [
                Some(FormsOf::Expressions(&expressions)),
                Some(FormsOf::Expressions(&expressions)),
                Some(FormsOf::Expressions(&expressions)),
            ],
        ];

        for combination in combinations_to_test {
            make_combinations(&combination, 2, "".into(), [None, None, None]);
        }
    }

    fn make_combinations(
        scenario: &[Option<FormsOf>; 3],
        position: usize,
        template: String,
        tokens: [Option<Token>; 3],
    ) {
        match &scenario[position] {
            Some(forms) => match forms {
                FormsOf::Texts(texts) => {
                    if position == 0 {
                        for text in *texts {
                            let mut tokens = tokens.clone();
                            tokens[position] = Some(Token::Text(String::from(*text)));
                            assert_template_has_tokens(format!("{}{}", *text, template), tokens);
                        }
                    } else {
                        for text in *texts {
                            let mut tokens = tokens.clone();
                            tokens[position] = Some(Token::Text(String::from(*text)));
                            make_combinations(
                                scenario,
                                position - 1,
                                format!("{}{}", *text, template),
                                tokens,
                            );
                        }
                    }
                }
                FormsOf::Expressions(expressions) => {
                    if position == 0 {
                        for expression in (*expressions).clone() {
                            let mut tokens = tokens.clone();
                            tokens[position] = Some(expression.1);
                            assert_template_has_tokens(
                                format!("{}{}", expression.0, template),
                                tokens,
                            );
                        }
                    } else {
                        for expression in (*expressions).clone() {
                            let mut tokens = tokens.clone();
                            tokens[position] = Some(expression.1);
                            make_combinations(
                                scenario,
                                position - 1,
                                format!("{}{}", expression.0, template),
                                tokens,
                            );
                        }
                    }
                }
            },
            None => {
                let tokens = tokens.clone();
                assert_template_has_tokens(template, tokens);
            }
        }
    }

    fn assert_template_has_tokens(template: String, expected_tokens: [Option<Token>; 3]) {
        let expected_tokens: Vec<Token> = expected_tokens
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.clone().unwrap())
            .collect();
        let mut tokenizer = Tokenizer::new(template.chars());
        tokenizer.tokenize();
        let actual_tokens = tokenizer.tokens;
        let message = format!(
            "expected template \"{}\" to have tokens {:?}, but received {:?}",
            template, expected_tokens, actual_tokens
        );
        assert_eq!(actual_tokens, expected_tokens, "{}", message);
        println!("template \"{}\" should have tokens {:?} ... ok", template, expected_tokens);
    }
}
