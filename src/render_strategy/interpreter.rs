use crate::template::{Context};

pub fn interpreter(template: String, context: Context) -> String {
    Interpreter::new(template.chars(), context).parse()
}

struct Interpreter<T: Iterator<Item = char>> {
    template: T,
    context: Context,
    previous_character: Option<char>,
    current_token: Option<Token>,
}

impl<T: Iterator<Item = char>> Interpreter<T> {
    fn new(template: T, context: Context) -> Self {
        Interpreter {
            template,
            context,
            previous_character: None,
            current_token: None,
        }
    }

    fn parse(&self) -> String {
        "".into()
    }

    fn next_token(&mut self) {
        let mut character = self.template.next().unwrap();

        if let None = self.previous_character {
            self.previous_character = Some(character);
            character = self.template.next().unwrap();
        }

        let last_two_chars = format!("{}{}", self.previous_character.unwrap(), character);
        self.current_token = match last_two_chars.as_str() {
            "{{" => Some(Token::PrintBracesOpening),
            _ => None,
        };
    }

    fn current_token(&self) -> &Option<Token> {
        &self.current_token
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    PrintBracesOpening,
}

fn token_value(token: Token) -> String {
    match token { 
        Token::PrintBracesOpening => "{{".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pair::{Pair, PairGenerator};

    #[test]
    fn can_detect_a_token() {
        let template = String::from("{{");
        let template_iterator = template.chars();
        let context = Context::with(vec![Pair::of("x", "x")]);
        let mut interpreter = Interpreter::new(template_iterator, context);
        interpreter.next_token();
        let current_token = interpreter.current_token().as_ref().unwrap();
        assert_eq!(current_token, &Token::PrintBracesOpening);
    }
}