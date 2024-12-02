use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();

        left.push(items.next().unwrap().parse::<i32>().unwrap());
        right.push(items.next().unwrap().parse::<i32>().unwrap());
    }

    let mut right_totals: HashMap<i32, i32> = HashMap::new();

    for &num in &right {
        *right_totals.entry(num).or_insert(0) += 1;
    }

    let similarity_score: i32 = left
        .iter()
        .map(|x| x * right_totals.get(x).unwrap_or(&0))
        .sum();

    dbg!(&similarity_score);
    Ok(similarity_score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
