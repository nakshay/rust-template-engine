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

        let end_position = if eof {
            self.position
        } else {
            self.position - 2
        };
        let content = self.build_string_from_positions(start_position, end_position);
        self.tokens.push(Token::Content(content));
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
    fn can_detect_tokens() {
        let template = String::from("Hello {{ person_name }}!");
        let mut tokenizer = Tokenizer::new(template);
        tokenizer.tokenize();
        let tokens = tokenizer.get_tokens();
        assert_eq!(
            *tokens,
            vec![
                Token::Content("Hello ".into()),
                Token::Variable("person_name".into()),
                Token::Content("!".into()),
            ]
        );
    }
}
