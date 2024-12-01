advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    Some(input
        .chars()
        .map(|char| match char {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        })
        .sum())
}

pub fn part_one_alt(input: &str) -> Option<i32> {
    Some(input
    .matches('(')
    .count()
    )

}


pub fn part_two(input: &str) -> Option<u32> {
    let mut sum: i32 = 0;
    for (idx, char) in input.chars().enumerate() {
        sum += match char {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        };
        if sum == -1 {
            return Some(idx as u32 + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("(())"), Some(0));
    }

    #[test]
    fn test_part_two() {
        //let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(part_two(")"), Some(1));
        // ()()) causes him to enter the basement at character position 5.
        assert_eq!(part_two("()())"), Some(5));
    }
}
