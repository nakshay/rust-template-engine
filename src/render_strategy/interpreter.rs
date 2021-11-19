use crate::template::Context;

pub fn interpreter(template: String, context: Context) -> String {
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

    fn tokenize(&mut self) {
        self.reset_state();

        while self.position < self.template.len() {
            self.position += 1;

            if self.context == TokenizerContext::TemplateContent {
                self.read_template_content();
                continue;
            }

            if self.template[self.position - 1] == '{' && self.template[self.position] == '{' {
                self.tokens.push(Token::PrintVariableStart);
                continue;
            }
        }
    }

    fn reset_state(&mut self) {
        self.context = TokenizerContext::TemplateContent;
        self.position = 0;
        self.tokens = Vec::new();
    }

    fn read_template_content(&mut self) {
        let start_position = self.position - 1;

        while self.template[start_position] != '{' && self.template[self.position] != '{' {
            self.position += 1;
        }

        self.context = TokenizerContext::PrintVariable;
        let content = self.build_template_content_string(start_position);
        self.tokens.push(Token::TemplateContent(content));
    }

    fn build_template_content_string(&self, mut position: usize) -> String {
        let mut content = String::new();

        while position < self.position {
            content.push(self.template[position]);
            position += 1;
        }

        content
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    PrintVariableStart,
    PrintVariableEnd,
    VariableName(String),
    TemplateContent(String),
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
                Token::TemplateContent("Hello ".into()),
                Token::PrintVariableStart,
                Token::VariableName("person_name".into()),
                Token::PrintVariableEnd,
                Token::TemplateContent("!".into()),
            ]
        );
    }
}
