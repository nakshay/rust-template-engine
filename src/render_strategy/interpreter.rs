use crate::template::Context;

pub fn interpreter(template: String, context: Context) -> String {
    let mut tokenizer = Tokenizer::new("".chars());
    tokenizer.tokenize();
    "".into()
}

struct Tokenizer<T: Iterator<Item = char>> {
    buffer: Vec<char>,
    current_context: TokenizerContext,
    previous_markers: (Option<char>, Option<char>),
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
            current_context: TokenizerContext::Start,
            previous_markers: (None, None),
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
        let second_marker_char = self.buffer.pop();
        let first_marker_char = self.buffer.pop();
        let current_markers = (first_marker_char, second_marker_char);
        let token_value: String = self.buffer.iter().collect();

        if token_value.trim().len() > 0 {
            let token = match self.current_context {
                TokenizerContext::ExpressionStart | TokenizerContext::StatementStart => {
                    Token::Text(token_value)
                }
                TokenizerContext::ExpressionEnd => Token::Expression(token_value.trim().into()),
                TokenizerContext::StatementEnd => Token::Statement(token_value.trim().into()),
                _ => return,
            };

            self.tokens.push(token);
        } else if let (
            (Some(previous_marker_first_char), Some(previous_marker_second_char)),
            (Some(current_marker_first_char), Some(current_marker_second_char)),
        ) = (self.previous_markers, current_markers)
        {
            let token = Token::Text(format!(
                "{}{}{}{}{}",
                previous_marker_first_char,
                previous_marker_second_char,
                token_value,
                current_marker_first_char,
                current_marker_second_char
            ));
            self.tokens.push(token);
        }

        self.buffer = Vec::new();
        self.previous_markers = (current_markers.0, current_markers.1);
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

    #[derive(Debug, PartialEq)]
    enum FormsOf<'a> {
        Expressions(&'a [(&'static str, Token); 52]),
        Texts(&'a [&'static str; 3]),
    }

    #[test]
    fn test_many_combinations() {
        let texts = ["a", "aa", "aaa"];
        let expressions = [
            ("{{}}", Token::Text("{{}}".into())),
            ("{{ }}", Token::Text("{{ }}".into())),
            ("{{  }}", Token::Text("{{  }}".into())),
            ("{{   }}", Token::Text("{{   }}".into())),
            ("{{a}}", Token::Expression("a".into())),
            ("{{a }}", Token::Expression("a".into())),
            ("{{a  }}", Token::Expression("a".into())),
            ("{{a   }}", Token::Expression("a".into())),
            ("{{ a}}", Token::Expression("a".into())),
            ("{{ a }}", Token::Expression("a".into())),
            ("{{ a  }}", Token::Expression("a".into())),
            ("{{ a   }}", Token::Expression("a".into())),
            ("{{  a}}", Token::Expression("a".into())),
            ("{{  a }}", Token::Expression("a".into())),
            ("{{  a  }}", Token::Expression("a".into())),
            ("{{  a   }}", Token::Expression("a".into())),
            ("{{   a}}", Token::Expression("a".into())),
            ("{{   a }}", Token::Expression("a".into())),
            ("{{   a  }}", Token::Expression("a".into())),
            ("{{   a   }}", Token::Expression("a".into())),
            ("{{aa}}", Token::Expression("aa".into())),
            ("{{aa }}", Token::Expression("aa".into())),
            ("{{aa  }}", Token::Expression("aa".into())),
            ("{{aa   }}", Token::Expression("aa".into())),
            ("{{ aa}}", Token::Expression("aa".into())),
            ("{{ aa }}", Token::Expression("aa".into())),
            ("{{ aa  }}", Token::Expression("aa".into())),
            ("{{ aa   }}", Token::Expression("aa".into())),
            ("{{  aa}}", Token::Expression("aa".into())),
            ("{{  aa }}", Token::Expression("aa".into())),
            ("{{  aa  }}", Token::Expression("aa".into())),
            ("{{  aa   }}", Token::Expression("aa".into())),
            ("{{   aa}}", Token::Expression("aa".into())),
            ("{{   aa }}", Token::Expression("aa".into())),
            ("{{   aa  }}", Token::Expression("aa".into())),
            ("{{   aa   }}", Token::Expression("aa".into())),
            ("{{aaa}}", Token::Expression("aaa".into())),
            ("{{aaa }}", Token::Expression("aaa".into())),
            ("{{aaa  }}", Token::Expression("aaa".into())),
            ("{{aaa   }}", Token::Expression("aaa".into())),
            ("{{ aaa}}", Token::Expression("aaa".into())),
            ("{{ aaa }}", Token::Expression("aaa".into())),
            ("{{ aaa  }}", Token::Expression("aaa".into())),
            ("{{ aaa   }}", Token::Expression("aaa".into())),
            ("{{  aaa}}", Token::Expression("aaa".into())),
            ("{{  aaa }}", Token::Expression("aaa".into())),
            ("{{  aaa  }}", Token::Expression("aaa".into())),
            ("{{  aaa   }}", Token::Expression("aaa".into())),
            ("{{   aaa}}", Token::Expression("aaa".into())),
            ("{{   aaa }}", Token::Expression("aaa".into())),
            ("{{   aaa  }}", Token::Expression("aaa".into())),
            ("{{   aaa   }}", Token::Expression("aaa".into())),
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
                            let mut template = template.clone();
                            template.push_str(*text);
                            assert_template_has_tokens(template, tokens);
                        }
                    } else {
                        for text in *texts {
                            let mut tokens = tokens.clone();
                            tokens[position] = Some(Token::Text(String::from(*text)));
                            let mut template = template.clone();
                            template.push_str(*text);
                            make_combinations(scenario, position - 1, template, tokens);
                        }
                    }
                }
                FormsOf::Expressions(expressions) => {
                    if position == 0 {
                        for expression in (*expressions).clone() {
                            let mut template = template.clone();
                            template.push_str(expression.0);
                            let mut tokens = tokens.clone();
                            tokens[position] = Some(expression.1);
                            assert_template_has_tokens(template, tokens);
                        }
                    } else {
                        for expression in (*expressions).clone() {
                            let mut template = template.clone();
                            template.push_str(expression.0);
                            let mut tokens = tokens.clone();
                            tokens[position] = Some(expression.1);
                            make_combinations(scenario, position - 1, template, tokens);
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
        println!("{:?} -> \"{}\"", expected_tokens, template);
        let expected_tokens: Vec<Token> = expected_tokens
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.clone().unwrap())
            .collect();
        let mut tokenizer = Tokenizer::new(template.chars());
        tokenizer.tokenize();
        let tokens = tokenizer.tokens;
        let message = format!(
            "expected template \"{}\" to have tokens {:?}, but received {:?}",
            template, expected_tokens, tokens
        );
        assert_eq!(tokens, expected_tokens, "{}", message);
    }
}
