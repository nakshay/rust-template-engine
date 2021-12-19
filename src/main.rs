fn main() {
    let texts = ["a", "aa", "aaa"];
    let expressions = [
        ("{{}}", ShouldBe::Text("{{}}")),
        ("{{ }}", ShouldBe::Text("{{ }}")),
        ("{{  }}", ShouldBe::Text("{{  }}")),
        ("{{   }}", ShouldBe::Text("{{   }}")),
        ("{{a}}", ShouldBe::Expression("a")),
        ("{{a }}", ShouldBe::Expression("a")),
        ("{{a  }}", ShouldBe::Expression("a")),
        ("{{a   }}", ShouldBe::Expression("a")),
        ("{{ a}}", ShouldBe::Expression("a")),
        ("{{ a }}", ShouldBe::Expression("a")),
        ("{{ a  }}", ShouldBe::Expression("a")),
        ("{{ a   }}", ShouldBe::Expression("a")),
        ("{{  a}}", ShouldBe::Expression("a")),
        ("{{  a }}", ShouldBe::Expression("a")),
        ("{{  a  }}", ShouldBe::Expression("a")),
        ("{{  a   }}", ShouldBe::Expression("a")),
        ("{{   a}}", ShouldBe::Expression("a")),
        ("{{   a }}", ShouldBe::Expression("a")),
        ("{{   a  }}", ShouldBe::Expression("a")),
        ("{{   a   }}", ShouldBe::Expression("a")),
        ("{{aa}}", ShouldBe::Expression("aa")),
        ("{{aa }}", ShouldBe::Expression("aa")),
        ("{{aa  }}", ShouldBe::Expression("aa")),
        ("{{aa   }}", ShouldBe::Expression("aa")),
        ("{{ aa}}", ShouldBe::Expression("aa")),
        ("{{ aa }}", ShouldBe::Expression("aa")),
        ("{{ aa  }}", ShouldBe::Expression("aa")),
        ("{{ aa   }}", ShouldBe::Expression("aa")),
        ("{{  aa}}", ShouldBe::Expression("aa")),
        ("{{  aa }}", ShouldBe::Expression("aa")),
        ("{{  aa  }}", ShouldBe::Expression("aa")),
        ("{{  aa   }}", ShouldBe::Expression("aa")),
        ("{{   aa}}", ShouldBe::Expression("aa")),
        ("{{   aa }}", ShouldBe::Expression("aa")),
        ("{{   aa  }}", ShouldBe::Expression("aa")),
        ("{{   aa   }}", ShouldBe::Expression("aa")),
        ("{{aaa}}", ShouldBe::Expression("aaa")),
        ("{{aaa }}", ShouldBe::Expression("aaa")),
        ("{{aaa  }}", ShouldBe::Expression("aaa")),
        ("{{aaa   }}", ShouldBe::Expression("aaa")),
        ("{{ aaa}}", ShouldBe::Expression("aaa")),
        ("{{ aaa }}", ShouldBe::Expression("aaa")),
        ("{{ aaa  }}", ShouldBe::Expression("aaa")),
        ("{{ aaa   }}", ShouldBe::Expression("aaa")),
        ("{{  aaa}}", ShouldBe::Expression("aaa")),
        ("{{  aaa }}", ShouldBe::Expression("aaa")),
        ("{{  aaa  }}", ShouldBe::Expression("aaa")),
        ("{{  aaa   }}", ShouldBe::Expression("aaa")),
        ("{{   aaa}}", ShouldBe::Expression("aaa")),
        ("{{   aaa }}", ShouldBe::Expression("aaa")),
        ("{{   aaa  }}", ShouldBe::Expression("aaa")),
        ("{{   aaa   }}", ShouldBe::Expression("aaa")),
    ];
    let combinations_to_test = [
        [FormsOf::None, FormsOf::None, FormsOf::Texts(&texts)],
        [
            FormsOf::None,
            FormsOf::None,
            FormsOf::Expressions(&expressions),
        ],
        [
            FormsOf::None,
            FormsOf::Texts(&texts),
            FormsOf::Expressions(&expressions),
        ],
        [
            FormsOf::None,
            FormsOf::Expressions(&expressions),
            FormsOf::Texts(&texts),
        ],
        [
            FormsOf::None,
            FormsOf::Expressions(&expressions),
            FormsOf::Expressions(&expressions),
        ],
        [
            FormsOf::Texts(&texts),
            FormsOf::Expressions(&expressions),
            FormsOf::Texts(&texts),
        ],
        [
            FormsOf::Texts(&texts),
            FormsOf::Expressions(&expressions),
            FormsOf::Expressions(&expressions),
        ],
        [
            FormsOf::Expressions(&expressions),
            FormsOf::Texts(&texts),
            FormsOf::Expressions(&expressions),
        ],
        [
            FormsOf::Expressions(&expressions),
            FormsOf::Expressions(&expressions),
            FormsOf::Texts(&texts),
        ],
        [
            FormsOf::Expressions(&expressions),
            FormsOf::Expressions(&expressions),
            FormsOf::Expressions(&expressions),
        ],
    ];

    for combination in combinations_to_test {
        make_combinations(&combination, 2, "".into());
    }
}

fn make_combinations(forms: &[FormsOf; 3], position: usize, template: String) {
    match forms[position] {
        FormsOf::Texts(texts) => {
            if position == 0 {
                for text in texts {
                    println!("{}{}", text, template);
                }
            } else {
                for text in texts {
                    make_combinations(forms, position - 1, format!("{}{}", text, template));
                }
            }
        },
        FormsOf::Expressions(expressions) => {
            if position == 0 {
                for expression in expressions {
                    println!("{}{}", expression.0, template);
                }
            } else {
                for expression in expressions {
                    make_combinations(forms, position - 1, format!("{}{}", expression.0, template));
                }
            }
        },
        FormsOf::None => {
            println!("{}", template);
        }
    }
}

#[derive(Debug, PartialEq)]
enum ShouldBe {
    Expression(&'static str),
    Text(&'static str),
}

#[derive(Debug, PartialEq)]
enum FormsOf<'a> {
    None,
    Expressions(&'a [(&'static str, ShouldBe); 52]),
    Texts(&'a [&'static str; 3]),
}
