use glam::IVec2;
use std::collections::HashMap;
use tracing::info;

const DOWN: [IVec2; 3] = [IVec2::new(-1, 0), IVec2::new(-2, 0), IVec2::new(-3, 0)];
const UP: [IVec2; 3] = [IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)];
const LEFT: [IVec2; 3] = [IVec2::new(0, -1), IVec2::new(0, -2), IVec2::new(0, -3)];
const RIGHT: [IVec2; 3] = [IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)];
const DOWN_LEFT: [IVec2; 3] = [IVec2::new(-1, -1), IVec2::new(-2, -2), IVec2::new(-3, -3)];
const DOWN_RIGHT: [IVec2; 3] = [IVec2::new(-1, 1), IVec2::new(-2, 2), IVec2::new(-3, 3)];
const UP_LEFT: [IVec2; 3] = [IVec2::new(1, -1), IVec2::new(2, -2), IVec2::new(3, -3)];
const UP_RIGHT: [IVec2; 3] = [IVec2::new(1, 1), IVec2::new(2, 2), IVec2::new(3, 3)];

const DIRECTIONS: [[IVec2; 3]; 8] = [
    UP, DOWN, LEFT, RIGHT, UP_RIGHT, UP_LEFT, DOWN_RIGHT, DOWN_LEFT,
];

const MAS: [char; 3] = ['M', 'A', 'S'];

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, value)| (IVec2::new(x as i32, y as i32), value))
        })
        .collect::<HashMap<IVec2, char>>();

    let result: usize = positions
        .iter()
        .filter(is_x)
        .map(|(position, _value)| {
            DIRECTIONS
                .iter()
                .map(|mas_positions| {
                    mas_positions
                        .iter()
                        .map(|offset| positions.get(&(position + offset)))
                        .enumerate()
                        .all(|(index, value)| MAS.get(index) == value)
                })
                .filter(|b| *b)
                .inspect(|b| info!("Found MAS: {}", b))
                .count()
        })
        .sum();

    Ok(result.to_string())
}

fn is_x(position: &(&IVec2, &char)) -> bool {
    position.1 == &'X'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
