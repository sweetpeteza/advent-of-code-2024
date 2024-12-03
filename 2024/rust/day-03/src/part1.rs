use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_in, instructions) = parse(input).map_err(|e| miette!("parse failed due to {}", e))?;

    let result: u32 = instructions.iter().map(|(a, b)| a * b).sum();

    Ok(result.to_string())
}

type Multiplication = (u32, u32);

fn instruction(input: &str) -> IResult<&str, Multiplication> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;

    Ok((input, pair))
}

fn parse(input: &str) -> IResult<&str, Vec<Multiplication>> {
    many1(many_till(anychar, instruction).map(|(_, instr)| instr))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }
}
