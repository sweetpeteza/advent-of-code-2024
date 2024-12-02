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
        .filter(|set| !set.iter().all_equal())
        .filter(|set| {
            set.iter().tuple_windows().all(|(curr, next)| curr < next)
                || set.iter().tuple_windows().all(|(curr, next)| curr > next)
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
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
