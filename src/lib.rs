trait Token {
    fn name(&self) -> &'static str;
}

struct StartToken { }

impl StartToken {
    fn new() -> Self {
        Self { }
    }
}

impl Token for StartToken {
    fn name(&self) -> &'static str {
        "Start"
    }
}

fn tokenize(template: String) -> Vec<impl Token> {
    return vec![StartToken::new()];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_the_start_token() {
        StartToken::new();
    }

    #[test]
    fn the_first_token_created_by_the_tokenizer_is_the_start_token() {
        let template = String::from("");
        let tokens = tokenize(template);
        assert_eq!(tokens[0].name(), String::from("Start"));
    }
}