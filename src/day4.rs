use std::time::Instant;

pub fn part1() -> usize {
    let start = Instant::now();
    let res = (136760u32..=595730).filter(test_criteria1).count();

    println!("time taken {:?}", (Instant::now() - start));
    res
}

pub fn part2() -> usize {
    let start = Instant::now();
    let res = (136760u32..=595730).filter(test_criteria1).filter(test_critera2).count();

    println!("time taken {:?}", (Instant::now() - start));
    res
}

fn test_criteria1(input: &u32) -> bool {
    let mut digits = input
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let mut has_double = false;

    let all_increasing = digits.iter().zip(digits.iter().skip(1)).all(|(c1, c2)| c1 <= c2);
    let has_adjacent = digits.iter().zip(digits.iter().skip(1)).any(|(c1, c2)| c1 == c2);

    all_increasing && has_adjacent

}

fn test_critera2(input: &u32) -> bool {
    let digits = input
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    (0..digits.len() - 3).any(|i| digits[i + 1] == digits[i + 2] && digits[i] != digits[i + 1] && digits[i + 2] != digits[i + 3])
        || (digits[0] == digits[1] && digits[1] != digits[2])
        || (digits[digits.len() - 1] == digits[digits.len() - 2] && digits[digits.len() - 2] != digits[digits.len() - 3])
}
