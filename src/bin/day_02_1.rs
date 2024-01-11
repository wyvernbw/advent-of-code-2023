use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt},
    multi::{fold_many0, many0},
    sequence::{terminated, tuple},
    IResult,
};

fn main() {
    advent_of_code_2023::initialize();
    let result = process(include_str!("../day_02_1_input.txt"));
    tracing::info!(?result);
}

fn process(input: &str) -> u32 {
    input.lines().flat_map(game_possible).sum()
}

fn game_possible(input: &str) -> Option<u32> {
    fn game(input: &str) -> IResult<&str, u32> {
        let (rem, (_game, _space, id, _colon)) =
            tuple((tag("Game"), tag(" "), digit1, tag(": ")))(input)?;
        Ok((rem, id.parse().expect("id should be a number")))
    }
    let (game, id) = game(input).ok()?;
    fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
        let (rem, mut games) = many0(terminated(game_parser, tag("; ")))(input)?;
        let (_, last_game) = game_parser(rem)?;
        games.push(last_game);
        Ok(("", games))
    }
    tracing::info!(games = ?parse_games(game));
    Some(0)
}

fn game_parser(input: &str) -> IResult<&str, Game> {
    fold_many0(
        tuple((
            map_res(digit1, str::parse::<u32>),
            tag(" "),
            alt((tag("red"), tag("green"), tag("blue"))),
            opt(tag(", ")),
        )),
        Game::default,
        |Game(r, g, b), (count, _, color, _)| match color {
            "red" => Game(r + count, g, b),
            "green" => Game(r, g + count, b),
            "blue" => Game(r, g, b + count),
            _ => Game(r, g, b),
        },
    )(input)
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Game(u32, u32, u32);

#[cfg(test)]
mod day_02_1_tests {
    use rstest::rstest;

    use crate::game_parser;
    use crate::game_possible;
    use crate::Game;
    use pretty_assertions::assert_eq;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        true
    )]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        false
    )]
    fn games(#[case] input: &str, #[case] expected: bool) {
        let result = game_possible(input);
        assert_eq!(result.is_some(), expected);
    }

    #[rstest]
    #[case("3 blue, 4 red", Game(4, 0, 3))]
    #[case("1 red, 2 green, 6 blue", Game(1, 2, 6))]
    fn test_game(#[case] input: &str, #[case] expected: Game) {
        let (_, game) = game_parser(input).unwrap();
        assert_eq!(game, expected)
    }
}
