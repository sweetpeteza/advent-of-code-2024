use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut reports = vec![];

    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .flat_map(|x| x.parse::<i32>())
            .collect();

        reports.push(levels);
    }

    let total_safe_tests = reports
        .iter()
        .filter(|set| {
            set.iter().tuple_windows().all(|(curr, next)| curr < next)
                || set.iter().tuple_windows().all(|(curr, next)| curr > next)
        })
        .filter(|set| !set.iter().all_equal())
        .filter(|set| {
            (set.iter().tuple_windows().all(|(curr, next)| curr < next)
                || set.iter().tuple_windows().all(|(curr, next)| curr > next))
                || ((0..set.len()).any(|d| {
                    dbg!(&d);
                    let new_set = set[..d]
                        .iter()
                        .chain(&set[d + 1..])
                        .copied()
                        .collect::<Vec<i32>>();
                    dbg!(&new_set);
                    new_set
                        .iter()
                        .tuple_windows()
                        .all(|(curr, next)| curr < next)
                        || new_set
                            .iter()
                            .tuple_windows()
                            .all(|(curr, next)| curr > next)
                }))
        })
        .filter(|set| {
            set.iter()
                .tuple_windows()
                .all(|(curr, next)| (curr - next).abs() < 4)
        })
        .count();

    Ok(total_safe_tests.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        /*
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 _ 2 4 5
        8 6 _ 4 1
        1 3 6 7 9 */
        assert_eq!("4", process(input)?);
        Ok(())
    }
    #[test]
    fn should_pass() -> miette::Result<()> {
        let input = "
    55 55 58 59 62 64
    71 71 72 73 77 78
";

        assert_eq!("4", process(input)?);
        Ok(())
    }
}
