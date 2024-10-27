use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::{Parser};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result<()> {
    let input = String::from("-273.15,-15\n");
    let parsed  = Grammar::parse(Rule::record, &input)?;
    println!("{:?}", parsed);

    Ok(())
}
