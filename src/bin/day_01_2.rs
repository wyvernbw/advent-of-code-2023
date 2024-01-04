use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::{
        complete::{char, digit1},
        is_alphabetic, is_alphanumeric,
    },
    combinator::{iterator, map, map_opt, map_res, peek, recognize, value},
    multi::many1,
    Err, IResult, InputIter, InputTake,
};

const INPUT: &str = include_str!("../day_01_2_test.txt");

fn main() {
    advent_of_code_2023::initialize();
    let res = process(INPUT);
    tracing::info!("{}", res);
}

fn numbers(input: &str) -> IResult<&str, Vec<i32>> {
    fn parser(input: &str) -> IResult<&str, Option<i32>> {
        let parsed = alt((
            // digit
            peek(map(digit1, |str: &str| str.parse::<i32>().ok())),
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
            // nothing
            peek(map(take_while1(|c: char| is_alphanumeric(c as u8)), |_| {
                None
            })),
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

fn process(input: &str) -> u32 {
    advent_of_code_2023::initialize();
    //let res = INPUT
    //    .lines()
    //    .map(|line| {})
    //    .inspect(|el| tracing::info!(?el))
    //    .sum::<u32>();
    //res
    0
}

#[cfg(test)]
mod test {
    use pretty_assertions::{assert_eq, assert_ne};
    use rstest::rstest;

    use crate::{numbers, process};

    #[rstest]
    #[case("two1nine", vec![2, 1, 9])]
    #[case("eightwothree", vec![8, 2, 3])]
    #[case("abcone2threexyz", vec![1, 2, 3])]
    #[case("xtwone3four", vec![2, 1, 3, 4])]
    #[case("4nineeightseven2", vec![4, 9, 8, 7, 2])]
    fn test_numbers(#[case] input: &str, #[case] expected: Vec<i32>) {
        advent_of_code_2023::initialize();
        match numbers(input) {
            Ok((_, res)) => assert_eq!(res, expected),
            Err(err) => panic!("{:?}", err),
        }
    }

    #[test]
    #[ignore]
    fn test_input() {
        assert_eq!(
            process(
                "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
        "
            ),
            281
        );
    }
}
