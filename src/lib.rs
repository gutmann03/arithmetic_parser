pub use pest::iterators::Pairs;
pub use pest::pratt_parser::PrattParser;
pub use pest::Parser;
pub use pest_derive::Parser;
pub use std::io::{self, BufRead};
pub use thiserror::*;

#[derive(pest_derive::Parser)]
#[grammar = "./calc.pest"]
pub struct MyParser;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left))
            .op(Op::prefix(unary_minus))
    };
}

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    UnaryMinus(Box<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: Op,
        rhs: Box<Expr>,
    },
    Unreachable,
}

/// Parse the input pairs into an abstract syntax tree representing the expression.
///
/// # Arguments
///
/// * `pairs` - A sequence of tokens representing the input expression.
///
/// # Returns
///
/// An abstract syntax tree representing the parsed expression.
pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::number => Expr::Number(primary.as_str().parse::<f64>().unwrap()),
            Rule::expr => parse_expr(primary.into_inner()),
            _ => Expr::Unreachable,
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Op::Add,
                Rule::subtract => Op::Subtract,
                Rule::multiply => Op::Multiply,
                Rule::divide => Op::Divide,
                _ => Op::Invalid,
            };
            Expr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => Expr::UnaryMinus(Box::new(rhs)),
            _ => Expr::Unreachable,
        })
        .parse(pairs)
}

/// Evaluate the given expression and return the result.
///
/// # Arguments
///
/// * `expr` - An abstract syntax tree representing the expression to be evaluated.
///
/// # Returns
///
/// The numerical result of the evaluated expression.
pub fn eval_expr(expr: Expr) -> Result<f64, MyError> {
    match expr {
        Expr::Number(n) => Ok(n),
        Expr::UnaryMinus(expr) => Ok(-(eval_expr(*expr)?)),
        Expr::BinOp { lhs, op, rhs } => match op {
            Op::Add => Ok(eval_expr(*lhs)? + eval_expr(*rhs)?),
            Op::Subtract => Ok(eval_expr(*lhs)? - eval_expr(*rhs)?),
            Op::Multiply => Ok(eval_expr(*lhs)? * eval_expr(*rhs)?),
            Op::Divide => Ok(eval_expr(*lhs)? / eval_expr(*rhs)?),
            Op::Invalid => Err(MyError::UnreachableError)
        },
        Expr::Unreachable => Err(MyError::UnreachableError),
    }
}

/// Evaluate an expression from a string input and return the result.
///
/// # Arguments
///
/// * `s` - A string representing the expression to be evaluated.
///
/// # Returns
///
/// A Result containing the numerical result of the evaluated expression, or an error if parsing fails.
pub fn eval_expr_from_string(s: &str) -> Result<f64, MyError> {
    let pairs = MyParser::parse(Rule::equation, s);
    match pairs {
        Ok(mut pairs_) => {
            let expr = parse_expr(pairs_.next().unwrap().into_inner());
            Ok(eval_expr(expr)?)
        }
        Err(e) => Err(MyError::ParseError(e.to_string())),
    }
}

/// MyError is an error type used to return errors from the parser.
#[derive(Error, Debug)]
pub enum MyError {
    #[error("io error")]
    IOError(io::Error),

    #[error("parse error")]
    ParseError(String),

    #[error("unknown error")]
    Unknown,

    #[error("cli error")]
    CLIError(String),

    #[error("ureachale error")]
    UnreachableError,
}

/// Op describes a mathematical operation.
#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    Invalid,
}

/// cli ia a modul with cli implementation.
pub mod cli {
    use super::*;
    use clap::Parser as ParserClap;
    use std::fs;

    /// Parser CLI.
    #[derive(ParserClap, Debug)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        /// Defines console mode.
        #[arg(short, long)]
        console: bool,

        /// Defines file mode.
        #[arg(short, long)]
        file: Option<String>,
    }

    /// run runs the programm.
    pub fn run() -> Result<(), MyError> {
        let args = Args::parse();

        if args.console && args.file.is_some() {
            return Err(MyError::CLIError(
                "Cannot use console mode and file mode at the same time.".to_string(),
            ));
        }

        if args.console {
            println!("Welcome to Arithmetic expression calculator parser.\nType you expression below and press enter.\nTo exit enter ':q'.");
            for line in io::stdin().lock().lines() {
                match line {
                    Ok(line) => {
                        if line.contains(":q") {
                            return Ok(());
                        }
                        let result = eval_expr_from_string(&line);
                        match result {
                            Ok(result) => println!("{} = {}", &line, result),
                            Err(e) => eprintln!("{}", MyError::ParseError(e.to_string())),
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", MyError::IOError(e));
                    }
                }
            }
            return Ok(());
        }

        match args.file {
            Some(file) => {
                let content = fs::read_to_string(&file);
                match content {
                    Ok(content) => {
                        let mut outs: Vec<String> = Vec::new();

                        for line in content.lines() {
                            let result = eval_expr_from_string(line)?;
                            outs.push(format!("{} = {}", &line, result));
                        }
                        match fs::write(file + ".out", outs.join("\n")) {
                            Ok(_) => (),
                            Err(e) => return Err(MyError::IOError(e)),
                        }
                    }
                    Err(e) => {
                        return Err(MyError::IOError(e));
                    }
                }
            }
            None => {
                return Err(MyError::CLIError("No file specified.".to_string()));
            }
        }

        Ok(())
    }
}
