use std::collections::HashSet;

fn main() {
    let text = vec!["a", "aa", "aaa", "aaaa"];
    let spaces = vec!["", " ", "  ", "   ", "    "];
    let mut expressions: HashSet<String> = HashSet::new();
    let mut texts: HashSet<String> = HashSet::new();
    for s1 in &spaces {
        texts.insert(format!("{}", s1));
        expressions.insert(format!("{{{{{}}}}}", s1));
        for t in &text {
            for s2 in &spaces {
                texts.insert(format!("{}{}{}", s1, t, s2));
                expressions.insert(format!("{{{{{}{}{}}}}}", s1, t, s2));
            }
        }
    }

    let mut one_token_combinations = texts.clone();
    one_token_combinations.extend(expressions.clone());
    
    let mut two_tokens_combinations: HashSet<String> = HashSet::new();
    for e1 in &expressions {
        for t in &texts {
            two_tokens_combinations.insert(format!("{}{}", t, e1));
            two_tokens_combinations.insert(format!("{}{}", e1, t));
        }

        for e2 in &expressions {
            two_tokens_combinations.insert(format!("{}{}", e1, e2));
        }
    }

    println!("one token combinations: {}", one_token_combinations.len());
    println!("two token combinations: {}", two_tokens_combinations.len());
}
