struct StartToken { }

impl StartToken {
    fn new() -> Self {
        Self { }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_the_start_token() {
        StartToken::new();
    }
}