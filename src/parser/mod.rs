use pest::Parser;
use pest::error::Error;
use crate::machine::Instruction;
use crate::machine::instruction::Graphics;

#[derive(Parser)]
#[grammar = "parser/chipper8.pest"]
struct Chipper8Parser;

pub fn parse_instruction(raw: &str) -> Result<Instruction, Error<Rule>> {
    let result = Chipper8Parser::parse(Rule::instruction, raw)?.next().unwrap();
    Ok(match result.as_rule() {
        Rule::exit => Instruction::Exit,
        Rule::graphics => Instruction::Graphics(match result.into_inner().next().unwrap().as_rule() {
            Rule::clear => Graphics::Clear,
            Rule::graphics => {
            }
            _ => unreachable!(),
        }),
        _ => unreachable!(),
    })
}