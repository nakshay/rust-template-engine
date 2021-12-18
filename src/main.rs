use std::collections::HashSet;

fn main() {
    let texts = vec![
        String::from("a"),
        String::from("aa"),
        String::from("aaa"),
    ];
    let spaces = vec!["", " ", "  ", "   "];
    let mut expressions: HashSet<String> = HashSet::new();

    for s1 in &spaces {
        expressions.insert(format!("{{{{{}}}}}", s1));
        for t in &texts {
            for s2 in &spaces {
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

    let mut three_tokens_combinations: HashSet<String> = HashSet::new();
    for c in &two_tokens_combinations {
        for t in &texts {
            three_tokens_combinations.insert(format!("{}{}", t, c));
        }

        for e in &expressions {
            three_tokens_combinations.insert(format!("{}{}", e, c));
        }
    }

    println!("one token combinations: {}", one_token_combinations.len());
    println!("two token combinations: {}", two_tokens_combinations.len());
    println!("three token combinations: {}", three_tokens_combinations.len());
}
