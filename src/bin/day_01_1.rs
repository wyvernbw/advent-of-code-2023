use nom::{character::complete::anychar, combinator::map, multi::many0, IResult};

const INPUT: &str = include_str!("../day_01_1_input.txt");

pub fn main() {
    advent_of_code_2023::initialize();
    let result = INPUT
        .lines()
        .map(calibration)
        .inspect(|el| tracing::info!(el))
        .sum::<u32>();
    tracing::info!(?result)
}

fn digits(input: &str) -> IResult<&str, Vec<u32>> {
    many0(map(anychar, |ch| ch.to_digit(10)))(input)
        .map(|(input, digits)| (input, digits.into_iter().flatten().collect()))
}

fn calibration(input: &str) -> u32 {
    let (_, digits) = digits(input).expect("digits");
    let (first, last) = digits
        .first()
        .zip(digits.last())
        .expect("expected at least one digit");
    first * 10 + last
}
