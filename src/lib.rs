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
}

pub fn parse_expr(pairs: Pairs<Rule>) -> Expr {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::number => Expr::Number(primary.as_str().parse::<f64>().unwrap()),
            Rule::expr => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::add => Op::Add,
                Rule::subtract => Op::Subtract,
                Rule::multiply => Op::Multiply,
                Rule::divide => Op::Divide,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Expr::BinOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            }
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::unary_minus => Expr::UnaryMinus(Box::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
}

pub fn eval_expr(expr: Expr) -> f64 {
    match expr {
        Expr::Number(n) => n,
        Expr::UnaryMinus(expr) => -eval_expr(*expr),
        Expr::BinOp { lhs, op, rhs } => match op {
            Op::Add => eval_expr(*lhs) + eval_expr(*rhs),
            Op::Subtract => eval_expr(*lhs) - eval_expr(*rhs),
            Op::Multiply => eval_expr(*lhs) * eval_expr(*rhs),
            Op::Divide => eval_expr(*lhs) / eval_expr(*rhs),
        },
    }
}

pub fn eval_expr_from_string(s: &str) -> Result<f64, MyError> {
    let pairs = MyParser::parse(Rule::equation, s);
    match pairs {
        Ok(mut pairs_) => {
            let expr = parse_expr(pairs_.next().unwrap().into_inner());
            Ok(eval_expr(expr))
        }
        Err(e) => Err(MyError::ParseError(e.to_string())),
    }
}

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
}

#[derive(Debug)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

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

    pub fn run() -> Result<(), MyError> {
        let args = Args::parse();

        if args.console && args.file.is_some() {
            return Err(MyError::CLIError(
                "Cannot use console mode and file mode at the same time.".to_string(),
            ));
        }

        if args.console {
            println!("Welcome to Arithmetic expression calculator parser. Type you expression below and press enter. To exit prss ctrl+C.");
            for line in io::stdin().lock().lines() {
                match line {
                    Ok(line) => {
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
                            let result = eval_expr_from_string(&line)?;
                            outs.push(format!("{} = {}", &line, result));
                        }
                        match fs::write(file + &".out".to_string(), outs.join("\n")) {
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
