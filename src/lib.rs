#[derive(Debug, PartialEq, Eq)]
struct StartToken {}

impl StartToken {
    fn new() -> Self {
        Self {}
    }

    fn next(&self) -> BeginStatementToken {
        BeginStatementToken::new()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BeginStatementToken {}

impl BeginStatementToken {
    fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_a_start_token() {
        let template = "{".chars();
        let token = StartToken::new();
        let next = token.next();
        assert_eq!(next, BeginStatementToken::new());
    }
}
