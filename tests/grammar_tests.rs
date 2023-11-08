use pest::Parser;
use anyhow::anyhow;
use my_parser_kma_group3_Smetaniuk::*;

#[test]
fn test_integer() -> anyhow::Result<()> {
    let pair = MyParser::parse(Rule::atom, "8")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "8" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 1 );

    let pair = MyParser::parse(Rule::atom, "1234567890")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "1234567890" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 10 );

    let pair = MyParser::parse(Rule::atom, "x");
    assert!(pair.is_err());

    let pair = MyParser::parse(Rule::atom, "");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_operations() -> anyhow::Result<()> {
    for s in "+-*/".chars() {
        let symb = s.to_string();
        let pair = MyParser::parse(Rule::bin_op, symb.as_str())?.next().ok_or_else( || anyhow!( "no pair" ) )?;
        assert_eq!( pair.as_str(), symb.as_str() );
        assert_eq!( pair.as_span().start(), 0 );
        assert_eq!( pair.as_span().end(), 1 );
    }

    let pair = MyParser::parse(Rule::bin_op, "/")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "/" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 1 );

    let pair = MyParser::parse(Rule::bin_op, "x");
    assert!(pair.is_err());

    let pair = MyParser::parse(Rule::bin_op, "%");
    assert!(pair.is_err());

    let pair = MyParser::parse(Rule::bin_op, "");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_expresion() -> anyhow::Result<()> {
    let pair = MyParser::parse(Rule::expr, "8")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "8" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 1 );

    let pair = MyParser::parse(Rule::expr, "8 + 1")?.next().ok_or_else( || anyhow!( "no pair" ) )?;
    assert_eq!( pair.as_str(), "8 + 1" );
    assert_eq!( pair.as_span().start(), 0 );
    assert_eq!( pair.as_span().end(), 5 );

    let pair = MyParser::parse(Rule::expr, "8 +");
    dbg!(&pair);
    assert!(pair.is_err());

    let pair = MyParser::parse(Rule::expr, "x");
    assert!(pair.is_err());

    let pair = MyParser::parse(Rule::expr, "");
    assert!(pair.is_err());

    Ok(())
}