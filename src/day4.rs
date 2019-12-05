use std::time::Instant;

pub fn part1() -> usize {
    (136760u32..=595730).filter(test_criteria1).count()
}

pub fn part2() -> usize {
    (136760u32..=595730).filter(test_criteria1).filter(test_critera2).count()
}

fn test_criteria1(input: &u32) -> bool {
    let mut digits = to_digits(input);
    let mut has_double = false;

    let all_increasing = digits.iter().zip(digits.iter().skip(1)).all(|(c1, c2)| c1 <= c2);
    let has_adjacent = digits.iter().zip(digits.iter().skip(1)).any(|(c1, c2)| c1 == c2);

    all_increasing && has_adjacent
}

fn test_critera2(input: &u32) -> bool {
    let digits = to_digits(input);

    (0..digits.len() - 3).any(|i| digits[i + 1] == digits[i + 2] && digits[i] != digits[i + 1] && digits[i + 2] != digits[i + 3])
        || (digits[0] == digits[1] && digits[1] != digits[2])
        || (digits[digits.len() - 1] == digits[digits.len() - 2] && digits[digits.len() - 2] != digits[digits.len() - 3])
}

fn to_digits(input: &u32) -> Vec<u32> {
    input
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>()
}