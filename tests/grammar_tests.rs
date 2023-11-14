use anyhow::anyhow;
use arithmetic_parser_smetaniuk::*;
use pest::Parser;

#[test]
fn test_number() -> anyhow::Result<()> {
    let pair = MyParser::parse(Rule::number, "8")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "8");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 1);

    let pair = MyParser::parse(Rule::number, "1234567890")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "1234567890");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 10);

    let pair = MyParser::parse(Rule::number, "1.234")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "1.234");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 5);

    let pair = MyParser::parse(Rule::number, "0.00000123")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "0.00000123");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 10);

    let pair = MyParser::parse(Rule::number, "x");
    assert!(pair.is_err());

    let pair = MyParser::parse(Rule::number, "");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_unary_minus() -> anyhow::Result<()> {
    let pair = MyParser::parse(Rule::unary_minus, "-")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "-");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 1);

    let pair = MyParser::parse(Rule::unary_minus, "x");
    assert!(pair.is_err());

    let pair = MyParser::parse(Rule::unary_minus, "");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_primary() -> anyhow::Result<()> {
    let pair = MyParser::parse(Rule::primary, "8")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "8");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 1);

    let pair = MyParser::parse(Rule::primary, "1234567890")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "1234567890");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 10);

    let pair = MyParser::parse(Rule::primary, "(1234567890)")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "1234567890");
    assert_eq!(pair.as_span().start(), 1);
    assert_eq!(pair.as_span().end(), 11);

    let pair = MyParser::parse(Rule::primary, "(1.234)")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "1.234");
    assert_eq!(pair.as_span().start(), 1);
    assert_eq!(pair.as_span().end(), 6);

    let pair = MyParser::parse(Rule::primary, "(0.00000123)")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "0.00000123");
    assert_eq!(pair.as_span().start(), 1);
    assert_eq!(pair.as_span().end(), 11);

    let pair = MyParser::parse(Rule::primary, "0.00000123")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "0.00000123");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 10);

    let pair = MyParser::parse(Rule::primary, "x");
    assert!(pair.is_err());

    let pair = MyParser::parse(Rule::primary, "");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_atom() -> anyhow::Result<()> {
    let pair = MyParser::parse(Rule::atom, "8")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "8");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 1);

    let pair = MyParser::parse(Rule::atom, "-1234567890")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "-");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 1);

    let pair = MyParser::parse(Rule::atom, "1234567890")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "1234567890");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 10);

    let pair = MyParser::parse(Rule::atom, "(1.234)")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "1.234");
    assert_eq!(pair.as_span().start(), 1);
    assert_eq!(pair.as_span().end(), 6);

    let pair = MyParser::parse(Rule::atom, "-0.00000123")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "-");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 1);

    let pair = MyParser::parse(Rule::atom, "0.00000123")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "0.00000123");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 10);

    let pair = MyParser::parse(Rule::atom, "(0.00000123)")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "0.00000123");
    assert_eq!(pair.as_span().start(), 1);
    assert_eq!(pair.as_span().end(), 11);

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
        let pair = MyParser::parse(Rule::bin_op, symb.as_str())?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), symb.as_str());
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 1);
    }

    let pair = MyParser::parse(Rule::bin_op, "/")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "/");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 1);

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
    let pair = MyParser::parse(Rule::expr, "8")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "8");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 1);

    let pair = MyParser::parse(Rule::expr, "8 + 1")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "8 + 1");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 5);

    let pair = MyParser::parse(Rule::expr, "8 + 1 - (0.5 / 33.7)")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "8 + 1 - (0.5 / 33.7)");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 20);

    let pair = MyParser::parse(Rule::expr, "x");
    assert!(pair.is_err());

    let pair = MyParser::parse(Rule::expr, "");
    assert!(pair.is_err());

    Ok(())
}

#[test]
fn test_equation() -> anyhow::Result<()> {
    let pair = MyParser::parse(Rule::equation, "8 + 1 - (0.5 / 33.7)")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "8 + 1 - (0.5 / 33.7)");
    assert_eq!(pair.as_span().start(), 0);
    assert_eq!(pair.as_span().end(), 20);

    let pair = MyParser::parse(Rule::equation, "x");
    assert!(pair.is_err());

    let pair = MyParser::parse(Rule::equation, "x +");
    assert!(pair.is_err());

    let pair = MyParser::parse(Rule::equation, "x + (");
    assert!(pair.is_err());

    let pair = MyParser::parse(Rule::equation, "");
    assert!(pair.is_err());

    Ok(())
}
