# my_parser_kma_group3_smetaniuk

Arithmetic expression calculator parser.

### Description
This parser can be used to parse and calculate arithmetic expressions, including addition (+), subtraction (-), multiplication (*), division (/), and parentheses using operations.

### Example
To parse an expression into pairs, you can use the following code snippet.

```rust
use my_parser_kma_group3_Smetaniuk::*;
use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let pair = MyParser::parse(Rule::expr, "8 + 1")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    println!("{}", pair.as_str());

    Ok(())
}
```

### Further development
The final version of parser should be abel to calculate the statement as strings (as above) or from file.

### Example
```rust
use my_parser_kma_group3_Smetaniuk::*;
use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let pair = MyParser::parse(Rule::expr, "8 + 1")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    println!("{}", pair.as_str());

    /*
    let value = MyParser::Eval(pair);
    println!("result: {}", value); // should be 8 + 1 = 9
    */

    Ok(())
}
```
