use crate::template::Context;

pub fn interpreter(template: String, context: Context) -> String {
    let mut tokenizer = Tokenizer::new("".into());
    tokenizer.tokenize();
    let _tokens = tokenizer.get_tokens();
    "".into()
}

struct Tokenizer {
    template: Vec<char>,
    position: usize,
    tokens: Vec<Token>,
    context: TokenizerContext,
}

impl Tokenizer {
    fn new(template: String) -> Self {
        Tokenizer {
            template: template.chars().collect(),
            position: 0,
            tokens: Vec::new(),
            context: TokenizerContext::TemplateContent,
        }
    }

    fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    fn reset_state(&mut self) {
        self.context = TokenizerContext::TemplateContent;
        self.position = 0;
        self.tokens = Vec::new();
    }

    fn tokenize(&mut self) {
        self.reset_state();

        while self.position < self.template.len() {
            if self.position == self.template.len() - 1 {
                break;
            }

            self.position += 1;
            match self.context {
                TokenizerContext::TemplateContent => self.template_content(),
                TokenizerContext::PrintVariable => self.print_variable(),
            };
        }
    }

    fn template_content(&mut self) {
        let start_position = if self.position == 1 { 0 } else { self.position };
        let mut eof = false;
        while self.template[self.position - 1] != '{' || self.template[self.position] != '{' {
            if self.position == self.template.len() - 1 {
                eof = true;
                break;
            }

            self.position += 1;
        }

        if self.position > 1 {
            let end_position = if eof {
                self.position
            } else {
                self.position - 2
            };
            let content = self.build_string_from_positions(start_position, end_position);
            if content.len() > 0 {
                self.tokens.push(Token::Content(content));
            }
        }

        self.context = TokenizerContext::PrintVariable;
    }

    fn print_variable(&mut self) {
        if self.template[self.position - 1] == '{' && self.template[self.position] == '{' {
            return;
        }

        self.read_variable_name();
    }

    fn build_string_from_positions(&self, start: usize, end: usize) -> String {
        let mut content = String::new();
        let mut position = start;

        while position <= end {
            content.push(self.template[position]);
            position += 1;
        }

        content
    }

    fn read_variable_name(&mut self) {
        let start_position = self.position;

        while self.template[self.position - 1] != '}' || self.template[self.position] != '}' {
            self.position += 1;
        }

        let variable_name = self.build_string_from_positions(start_position, self.position - 2);
        self.tokens
            .push(Token::Variable(variable_name.trim().into()));
        self.context = TokenizerContext::TemplateContent;
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Variable(String),
    Content(String),
}

#[derive(Debug, PartialEq)]
enum TokenizerContext {
    TemplateContent,
    PrintVariable,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_variable() {
        assert_template_has_tokens("{{ var }}", vec![Token::Variable("var".into())]);
    }

    #[test]
    fn many_variables() {
        assert_template_has_tokens(
            "{{ var }}{{ var }}{{ var }}",
            vec![
                Token::Variable("var".into()),
                Token::Variable("var".into()),
                Token::Variable("var".into()),
            ]
        );
    }

    #[test]
    fn content_only() {
        assert_template_has_tokens("content 123 !@#", vec![Token::Content("content 123 !@#".into())]);
    }

    #[test]
    fn simple_template() {
        assert_template_has_tokens(
            "Hello {{ person_name }}!",
            vec![
                Token::Content("Hello ".into()),
                Token::Variable("person_name".into()),
                Token::Content("!".into()),
            ],
        );
    }

    #[test]
    fn tokens_on_both_ends() {
        assert_template_has_tokens(
            "{{ a }} content {{ b }}",
            vec![
                Token::Variable("a".into()),
                Token::Content(" content ".into()),
                Token::Variable("b".into()),
            ],
        );

        assert_template_has_tokens(
            "-{{ a }} content {{ b }}-",
            vec![
                Token::Content("-".into()),
                Token::Variable("a".into()),
                Token::Content(" content ".into()),
                Token::Variable("b".into()),
                Token::Content("-".into()),
            ],
        );

        assert_template_has_tokens(
            "--{{ a }} content {{ b }}--",
            vec![
                Token::Content("--".into()),
                Token::Variable("a".into()),
                Token::Content(" content ".into()),
                Token::Variable("b".into()),
                Token::Content("--".into()),
            ],
        );

        assert_template_has_tokens(
            "---{{ a }} content {{ b }}---",
            vec![
                Token::Content("---".into()),
                Token::Variable("a".into()),
                Token::Content(" content ".into()),
                Token::Variable("b".into()),
                Token::Content("---".into()),
            ],
        );
    }

    fn assert_template_has_tokens(template: &'static str, expected_tokens: Vec<Token>) {
        let mut tokenizer = Tokenizer::new(template.into());
        tokenizer.tokenize();
        let actual_tokens = tokenizer.get_tokens();
        assert_eq!(*actual_tokens, expected_tokens);
    }
}
