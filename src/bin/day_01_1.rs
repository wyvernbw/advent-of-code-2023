const INPUT: &str = include_str!("../day_01_1_input.txt");

pub fn main() {
    advent_of_code_2023::initialize();
    let res = INPUT
        .lines()
        .flat_map(|line| {
            let iter = line.chars().filter(|c| c.is_numeric());
            iter.clone()
                .take(1)
                .chain(iter.rev().take(1))
                .filter_map(|c| c.to_digit(10))
                .reduce(|acc, el| acc * 10 + el)
        })
        .inspect(|el| tracing::info!(el))
        .sum::<u32>();
    tracing::info!(res)
}
