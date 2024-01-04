use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::{
        complete::{anychar, digit1},
        is_alphanumeric,
    },
    combinator::{map, peek, value},
    multi::many1,
    IResult,
};

const INPUT: &str = include_str!("../day_01_1_input.txt");

fn main() {
    advent_of_code_2023::initialize();
    let res = process(INPUT);
    tracing::info!("{}", res);
}

fn numbers(input: &str) -> IResult<&str, Vec<i32>> {
    fn parser(input: &str) -> IResult<&str, Option<i32>> {
        let parsed = alt((
            // digit spelled out
            peek(alt((
                value(Some(1), tag("one")),
                value(Some(2), tag("two")),
                value(Some(3), tag("three")),
                value(Some(4), tag("four")),
                value(Some(5), tag("five")),
                value(Some(6), tag("six")),
                value(Some(7), tag("seven")),
                value(Some(8), tag("eight")),
                value(Some(9), tag("nine")),
            ))),
            // digit
            peek(map(anychar, |str| str.to_digit(10).map(|num| num as i32))),
        ))(input);
        let (rem, parsed) = parsed?;
        match rem.len() {
            2.. => Ok((&rem[1..], parsed)),
            _ => Ok(("", parsed)),
        }
    }
    // return the original input as the remaining input
    many1(parser)(input).map(|(rem, parsed)| (rem, parsed.into_iter().flatten().collect()))
}

fn process(input: &str) -> i32 {
    advent_of_code_2023::initialize();
    let get_number = |digits: &[i32]| {
        let (first, last) = digits
            .first()
            .zip(digits.last())
            .expect("there should be at least 2 digits");
        first * 10 + last
    };
    let res = input
        .lines()
        .flat_map(|line| numbers(line))
        .map(|(_, res)| get_number(&res))
        .inspect(|el| tracing::info!(?el))
        .sum::<i32>();
    res
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::{numbers, process};

    #[rstest]
    #[case("two1nine", vec![2, 1, 9])]
    #[case("eightwothree", vec![8, 2, 3])]
    #[case("abcone2threexyz", vec![1, 2, 3])]
    #[case("xtwone3four", vec![2, 1, 3, 4])]
    #[case("4nineeightseven2", vec![4, 9, 8, 7, 2])]
    #[case("vqjvxtc79mvdnktdsxcqc1sevenone", vec![7, 9, 1, 7, 1])]
    fn test_numbers(#[case] input: &str, #[case] expected: Vec<i32>) {
        advent_of_code_2023::initialize();
        match numbers(input) {
            Ok((_, res)) => assert_eq!(res, expected),
            Err(err) => panic!("{:?}", err),
        }
    }

    #[test]
    fn test_input() {
        advent_of_code_2023::initialize();
        assert_eq!(process(include_str!("../day_01_2_test.txt")), 281);
    }
}
