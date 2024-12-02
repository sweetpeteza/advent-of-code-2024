use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};
use tracing::instrument;

enum Direction {
    Increasing,
    Decreasing,
}

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, reports) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    let result = reports
        .iter()
        .filter(|report| match check_safety(report) {
            Ok(_) => true,
            Err(_) => {
                for index in 0..report.len() {
                    let mut new_report = (*report).clone();
                    new_report.remove(index);

                    match check_safety(&new_report) {
                        Ok(_) => return true,
                        Err(_) => continue,
                    }
                }
                false
            }
        })
        .count();
    Ok(result.to_string())
}

#[instrument(ret)]
fn check_safety(report: &Report) -> Result<(), String> {
    use Direction::*;
    let mut direction: Option<Direction> = None;
    for (a, b) in report.iter().tuple_windows() {
        let diff = a - b;
        match diff.signum() {
            -1 => match direction {
                Some(Increasing) => {
                    return Err(format!("{};{} now increasing", a, b));
                }
                Some(Decreasing) => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(format!("{};{} diff of {}", a, b, diff.abs()));
                    } else {
                        continue;
                    }
                }
                None => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(format!("{};{} diff of {}", a, b, diff.abs()));
                    } else {
                        direction = Some(Decreasing);
                        continue;
                    }
                }
            },
            1 => match direction {
                Some(Increasing) => {
                    if !(1..=3).contains(&diff) {
                        return Err(format!("{};{} diff of {}", a, b, diff.abs()));
                    } else {
                        continue;
                    }
                }
                Some(Decreasing) => {
                    return Err(format!("{};{} now decreasing", a, b));
                }
                None => {
                    if !(1..=3).contains(&diff) {
                        return Err(format!("{};{} diff of {}", a, b, diff.abs()));
                    } else {
                        direction = Some(Increasing);
                        continue;
                    }
                }
            },
            0 => {
                return Err(format!("{};{} has no diff", a, b));
            }
            _ => {
                panic!("I didn't park my car here");
            }
        }
    }
    Ok(())
}

type Report = Vec<i32>;

fn parse(input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(line_ending, separated_list1(space1, complete::i32))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
