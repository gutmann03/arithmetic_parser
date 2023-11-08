pub use pest::Parser;
pub use pest_derive::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "./calc.pest"]
pub struct MyParser;
