use itertools::Itertools;
fn main() {
    let input = include_str!("../../../input/day01/input.txt");
    let parsed = parse(input);
    println!("Part One Iterators: {}", part_one_iterator(&parsed));
    println!("Part Two Indexes:   {}", part_one_indexes(&parsed));
    println!("Part Two Iterators: {}", part_two_iterator(&parsed));
    println!("Part Two Indexes:   {}", part_two_indexes(&parsed));
}

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn part_one_iterator(input: &[usize]) -> usize {
    input
        .iter()
        .tuple_windows()
        .filter(|(first, second)| second > first)
        .count()
}

fn part_one_indexes(input: &[usize]) -> usize {
    let mut count = 0;

    for i in 0..input.len() {
        if let Some(n) = input.get(i + 1) {
            if input[i] < *n {
                count += 1;
            }
        }
    }

    count
}

fn part_two_iterator(input: &[usize]) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(first, second)| second > first)
        .count()
}

fn part_two_indexes(input: &[usize]) -> usize {
    let mut count = 0;
    let windows: Vec<usize> = input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .collect();

    for i in 0..windows.len() {
        if let Some(n) = windows.get(i + 1) {
            if windows[i] < *n {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_one_test() {
        let input = include_str!("../../../input/day01/test_one.txt");
        let parsed = parse(input);
        assert_eq!(part_one_iterator(&parsed), 7);
        assert_eq!(part_one_indexes(&parsed), 7);
    }

    #[test]
    fn part_two_test() {
        let input = include_str!("../../../input/day01/test_two.txt");
        let parsed = parse(input);
        assert_eq!(part_two_iterator(&parsed), 5);
        assert_eq!(part_two_indexes(&parsed), 5);
    }
}
