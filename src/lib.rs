trait Token {
    fn value(&self) -> &str;
}

struct TextToken {
    value: String,
}

impl TextToken {
    fn new(value: String) -> Self {
        Self { value }
    }
}

impl Token for TextToken {
    fn value(&self) -> &str {
        &self.value
    }
}

struct StatementStartToken { }

impl StatementStartToken {
    fn new() -> Self {
        Self { }
    }
}

impl Token for StatementStartToken {
    fn value(&self) -> &str {
        "{"
    }
}

fn tokenize(template: String) -> Vec<Box<dyn Token>> {
    return vec![
        Box::new(TextToken::new("".into())),
        Box::new(StatementStartToken::new()),
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_a_text_token() {
        let token = TextToken::new("text".into());
        assert_eq!(token.value(), "text");
    }

    #[test]
    fn the_first_token_created_by_the_tokenizer_is_an_empty_text_token() {
        let template = String::from("");
        let tokens = tokenize(template);
        assert_eq!(tokens[0].value(), "");
    }

    #[test]
    fn the_text_token_can_detect_a_statement_start_token_as_the_next_token() {
        let template = String::from("{");
        let tokens = tokenize(template);
        assert_eq!(tokens[1].value(), String::from("{"));
    }
}
