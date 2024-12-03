use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    use Instruction::*;
    use ProcessState::*;
    let (_in, instructions) = parse(input).map_err(|e| miette!("parse failed due to {}", e))?;

    let result: u32 = instructions
        .iter()
        .fold((Active, 0), |(state, total), instr| match instr {
            Mul((a, b)) => match state {
                Active => (state, total + a * b),
                Inactive => (state, total),
            },
            Do => (Active, total),
            Dont => (Inactive, total),
        })
        .1;

    Ok(result.to_string())
}

#[derive(Debug, Clone)]
enum Instruction {
    Mul(MultiplicationParams),
    Do,
    Dont,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ProcessState {
    Active,
    Inactive,
}

type MultiplicationParams = (u32, u32);

fn instruction(input: &str) -> IResult<&str, Instruction> {
    use Instruction::*;
    alt((
        value(Do, tag("do()")),
        value(Dont, tag("don't()")),
        multiply,
    ))(input)
}

fn multiply(input: &str) -> IResult<&str, Instruction> {
    use Instruction::Mul;
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;

    Ok((input, Mul(pair)))
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_, instr)| instr))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
        Ok(())
    }
}
