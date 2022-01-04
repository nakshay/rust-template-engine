# Syntax

## Big Picture:

![Diagram(
    ZeroOrMore(
        Choice(
            2,
            NonTerminal("include statement"),
            NonTerminal("if statement"),
            Terminal("Text"),
            NonTerminal("print variable"),
            NonTerminal("foreach loop"),
        )
    )
)](./img/big-picture.svg)