use std::{convert::identity, error::Error};

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
    input
        .lines()
        .flat_map(max_game)
        .map(|game| game.game_power())
        .sum()
}

fn max_game(input: &str) -> Result<Game, Box<dyn Error + '_>> {
    fn game(input: &str) -> IResult<&str, u32> {
        let (rem, (_game, _space, id, _colon)) =
            tuple((tag("Game"), tag(" "), digit1, tag(": ")))(input)?;
        Ok((rem, id.parse().expect("id should be a number")))
    }
    let (game, id) = game(input)?;
    fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
        let (rem, mut games) = many0(terminated(game_parser, tag("; ")))(input)?;
        let (_, last_game) = game_parser(rem)?;
        games.push(last_game);
        Ok(("", games))
    }
    let (_, games) = parse_games(game)?;
    Ok(games
        .iter()
        .fold(Game::default(), |acc, el| acc.max_components(el)))
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

impl Game {
    fn max_components(&self, other: &Self) -> Self {
        Game(
            self.0.max(other.0),
            self.1.max(other.1),
            self.2.max(other.2),
        )
    }
    fn game_power(&self) -> u32 {
        self.0 * self.1 * self.2
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.1.partial_cmp(&other.1) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.2.partial_cmp(&other.2)
    }
}

#[cfg(test)]
mod day_02_1_tests {
    use rstest::rstest;

    use crate::game_parser;
    use crate::Game;
    use pretty_assertions::assert_eq;

    #[rstest]
    #[case("3 blue, 4 red", Game(4, 0, 3))]
    #[case("1 red, 2 green, 6 blue", Game(1, 2, 6))]
    fn test_game(#[case] input: &str, #[case] expected: Game) {
        let (_, game) = game_parser(input).unwrap();
        assert_eq!(game, expected)
    }
}
