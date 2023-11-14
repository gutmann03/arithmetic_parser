# arithmetic_parser_smetaniuk

Arithmetic expression calculator parser.

### Description
This parser can be used to parse and calculate arithmetic expressions, including addition (+), subtraction (-), multiplication (*), division (/), and parentheses using operations.

### Rules description
// This rule describes a valid number, which can be integer or decimal.
number = @{
    ("0" | ASCII_NONZERO_DIGIT ~ ASCII_DIGIT*)
    ~ ("." ~ ASCII_DIGIT*)?
}

// This rule represents the unary minus operator.
unary_minus = { "-" }

// This rule represents the primary expression, which can be a number or an enclosed expression.
primary = _{ number | "(" ~ expr ~ ")" }

// This rule represents the atom, which can be a primary expression with an optional unary minus.
atom = _{ unary_minus? ~ primary }

// This rule represents the binary operators, including add, subtract, multiply, and divide.
bin_op = _{ add | subtract | multiply | divide }
  add = { "+" }
  subtract = { "-" }
  multiply = { "*" }
  divide = { "/" }

// This rule represents the overall expression, composed of atoms and binary operators.
expr = { atom ~ (bin_op ~ atom)* }

// This rule represents an equation, starting and ending with specific markers and having an expression in between.
equation = _{ SOI ~ expr ~ EOI }

// This rule defines whitespace as a space character.
WHITESPACE = _{ " " }

### Launching
To run parset in console mode, please, run in command prompt next commands.
```shell
make pretty run_console
```

To run parset in file calculation mode, please, run in command prompt next commands.
```shell
make pretty run_file file=<your_file_path_here>
```

## Flow
The code defines a parser for arithmetic expressions using the Pratt parsing algorithm.

The parse_expr function takes a sequence of tokens as input and parses it into an abstract syntax tree representing the expression. It uses the PrattParser to do this, which is initialized in a lazy_static block. The PrattParser handles the order of operations and builds the expression tree accordingly.

After the expression is parsed, the eval_expr function evaluates the expression represented by the abstract syntax tree and returns the result. This involves recursively evaluating sub-expressions and performing the appropriate arithmetic operations.

Overall, this code allows you to parse and evaluate arithmetic expressions by first converting them into a structured form and then computing their values.