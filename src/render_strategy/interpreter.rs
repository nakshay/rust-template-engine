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

    #[test]
    fn test_many_combinations() {
        let texts = ["a", "aa", "aaa"];
        let expressions = [
            ("{{}}", ShouldBe::Text("{{}}")),
            ("{{ }}", ShouldBe::Text("{{ }}")),
            ("{{  }}", ShouldBe::Text("{{  }}")),
            ("{{   }}", ShouldBe::Text("{{   }}")),
            ("{{a}}", ShouldBe::Expression("a")),
            ("{{a }}", ShouldBe::Expression("a")),
            ("{{a  }}", ShouldBe::Expression("a")),
            ("{{a   }}", ShouldBe::Expression("a")),
            ("{{ a}}", ShouldBe::Expression("a")),
            ("{{ a }}", ShouldBe::Expression("a")),
            ("{{ a  }}", ShouldBe::Expression("a")),
            ("{{ a   }}", ShouldBe::Expression("a")),
            ("{{  a}}", ShouldBe::Expression("a")),
            ("{{  a }}", ShouldBe::Expression("a")),
            ("{{  a  }}", ShouldBe::Expression("a")),
            ("{{  a   }}", ShouldBe::Expression("a")),
            ("{{   a}}", ShouldBe::Expression("a")),
            ("{{   a }}", ShouldBe::Expression("a")),
            ("{{   a  }}", ShouldBe::Expression("a")),
            ("{{   a   }}", ShouldBe::Expression("a")),
            ("{{aa}}", ShouldBe::Expression("aa")),
            ("{{aa }}", ShouldBe::Expression("aa")),
            ("{{aa  }}", ShouldBe::Expression("aa")),
            ("{{aa   }}", ShouldBe::Expression("aa")),
            ("{{ aa}}", ShouldBe::Expression("aa")),
            ("{{ aa }}", ShouldBe::Expression("aa")),
            ("{{ aa  }}", ShouldBe::Expression("aa")),
            ("{{ aa   }}", ShouldBe::Expression("aa")),
            ("{{  aa}}", ShouldBe::Expression("aa")),
            ("{{  aa }}", ShouldBe::Expression("aa")),
            ("{{  aa  }}", ShouldBe::Expression("aa")),
            ("{{  aa   }}", ShouldBe::Expression("aa")),
            ("{{   aa}}", ShouldBe::Expression("aa")),
            ("{{   aa }}", ShouldBe::Expression("aa")),
            ("{{   aa  }}", ShouldBe::Expression("aa")),
            ("{{   aa   }}", ShouldBe::Expression("aa")),
            ("{{aaa}}", ShouldBe::Expression("aaa")),
            ("{{aaa }}", ShouldBe::Expression("aaa")),
            ("{{aaa  }}", ShouldBe::Expression("aaa")),
            ("{{aaa   }}", ShouldBe::Expression("aaa")),
            ("{{ aaa}}", ShouldBe::Expression("aaa")),
            ("{{ aaa }}", ShouldBe::Expression("aaa")),
            ("{{ aaa  }}", ShouldBe::Expression("aaa")),
            ("{{ aaa   }}", ShouldBe::Expression("aaa")),
            ("{{  aaa}}", ShouldBe::Expression("aaa")),
            ("{{  aaa }}", ShouldBe::Expression("aaa")),
            ("{{  aaa  }}", ShouldBe::Expression("aaa")),
            ("{{  aaa   }}", ShouldBe::Expression("aaa")),
            ("{{   aaa}}", ShouldBe::Expression("aaa")),
            ("{{   aaa }}", ShouldBe::Expression("aaa")),
            ("{{   aaa  }}", ShouldBe::Expression("aaa")),
            ("{{   aaa   }}", ShouldBe::Expression("aaa")),
        ];
        let combinations_to_test = [
            [FormsOf::None, FormsOf::None, FormsOf::Texts(&texts)],
            [
                FormsOf::None,
                FormsOf::None,
                FormsOf::Expressions(&expressions),
            ],
            [
                FormsOf::None,
                FormsOf::Texts(&texts),
                FormsOf::Expressions(&expressions),
            ],
            [
                FormsOf::None,
                FormsOf::Expressions(&expressions),
                FormsOf::Texts(&texts),
            ],
            [
                FormsOf::None,
                FormsOf::Expressions(&expressions),
                FormsOf::Expressions(&expressions),
            ],
            [
                FormsOf::Texts(&texts),
                FormsOf::Expressions(&expressions),
                FormsOf::Texts(&texts),
            ],
            [
                FormsOf::Texts(&texts),
                FormsOf::Expressions(&expressions),
                FormsOf::Expressions(&expressions),
            ],
            [
                FormsOf::Expressions(&expressions),
                FormsOf::Texts(&texts),
                FormsOf::Expressions(&expressions),
            ],
            [
                FormsOf::Expressions(&expressions),
                FormsOf::Expressions(&expressions),
                FormsOf::Texts(&texts),
            ],
            [
                FormsOf::Expressions(&expressions),
                FormsOf::Expressions(&expressions),
                FormsOf::Expressions(&expressions),
            ],
        ];
        for combination in combinations_to_test {
            make_combinations(
                &combination,
                2,
                [ShouldBe::None, ShouldBe::None, ShouldBe::None],
            );
        }
    }

    fn make_combinations(forms: &[FormsOf; 3], position: usize, template: [ShouldBe; 3]) {
        match forms[position] {
            FormsOf::Texts(texts) => {
                if position == 0 {
                    for text in texts {
                        let mut template = template.clone();
                        template[position] = ShouldBe::Text(text);
                        assert_template_has_tokens(template);
                    }
                } else {
                    for text in texts {
                        let mut template = template.clone();
                        template[position] = ShouldBe::Text(text);
                        make_combinations(forms, position - 1, template);
                    }
                }
            },
            FormsOf::Expressions(expressions) => {
                if position == 0 {
                    for expression in expressions {
                        let mut template = template.clone();
                        template[position] = expression.1.clone();
                        assert_template_has_tokens(template);
                    }
                } else {
                    for expression in expressions {
                        let mut template = template.clone();
                        template[position] = expression.1.clone();
                        make_combinations(forms, position - 1, template.clone());
                    }
                }
            },
            FormsOf::None => {
                let template = template.clone();
                assert_template_has_tokens(template);
            },
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    enum ShouldBe {
        None,
        Expression(&'static str),
        Text(&'static str),
    }

    #[derive(Debug, PartialEq)]
    enum FormsOf<'a> {
        None,
        Expressions(&'a [(&'static str, ShouldBe); 52]),
        Texts(&'a [&'static str; 3]),
    }

    fn assert_template_has_tokens(template: [ShouldBe; 3]) {
        let mut template_string = String::new();
        let mut expected_tokens: Vec<Token> = Vec::new();

        for expectation in template {
            match expectation {
                ShouldBe::Text(text) => {
                    template_string.push_str(text);
                    let last = expected_tokens.last();

                    match last {
                        Some(last) => match (*last).clone() {
                            Token::Text(mut previous_text) => previous_text.push_str(text),
                            _ => expected_tokens.push(Token::Text(String::from(text))),
                        },
                        None => expected_tokens.push(Token::Text(String::from(text))),
                    }
                }
                ShouldBe::Expression(expression) => {
                    template_string.push_str(expression);
                    expected_tokens.push(Token::Expression(String::from(expression)));
                }
                ShouldBe::None => {}
            }
        }

        let mut tokenizer = Tokenizer::new(template_string.chars());
        tokenizer.tokenize();
        let tokens = tokenizer.tokens;
        let message = format!(
            "expected template \"{}\" to have tokens {:?}, but received {:?}",
            template_string, expected_tokens, tokens
        );
        assert_eq!(tokens, expected_tokens, "{}", message);
    }
}
