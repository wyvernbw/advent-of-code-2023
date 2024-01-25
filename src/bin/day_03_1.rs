#![feature(const_option)]
#![feature(if_let_guard)]

use std::ops::Range;

use advent_of_code_2023::initialize;

const INPUT: &str = include_str!("../day_03_input.txt");

fn main() {
    initialize();
    tracing::info!(answer = process(INPUT));
}

fn process(input: &str) -> u32 {
    let engine = make_engine(input);
    todo!()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tile {
    Number(u32),
    Empty,
    Symbol,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct EngineTile {
    tile: Tile,
    range: Range<u32>,
    row: u32,
}

impl EngineTile {
    fn new(tile: Tile, range: Range<u32>, row: u32) -> Self {
        Self { tile, range, row }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Engine(Vec<Vec<EngineTile>>);

impl Engine {
    fn perimeter_iter<'a>(&'a self, pivot: &'a EngineTile) -> PerimeterIterator {
        let pivot_row = pivot.row as usize;
        PerimeterIterator::new(pivot_row, self, pivot)
    }
}

struct PerimeterIterator<'a> {
    engine: &'a Engine,
    pivot: &'a EngineTile,
    row: usize,
    col: usize,
}

impl<'a> PerimeterIterator<'a> {
    fn new(pivot_row: usize, engine: &'a Engine, pivot: &'a EngineTile) -> Self {
        Self {
            engine,
            pivot,
            row: pivot_row.saturating_sub(1),
            col: 0,
        }
    }
}

impl<'a> Iterator for PerimeterIterator<'a> {
    type Item = &'a EngineTile;

    fn next(&mut self) -> Option<Self::Item> {
        fn in_perimeter(row: usize, col_range: &Range<u32>, pivot: &EngineTile) -> bool {
            let in_vertical_range = ((pivot.row.saturating_sub(1))..(pivot.row.saturating_add(2)))
                .contains(&(row as u32));
            let in_horizontal_range = col_range.clone().any(|col| {
                ((pivot.range.start.saturating_sub(1))..(pivot.range.end.saturating_add(2)))
                    .contains(&col)
            });
            in_vertical_range && in_horizontal_range
        }
        if self.row > self.pivot.row as usize + 1 {
            return None;
        }
        let Engine(engine) = self.engine;
        let next = engine.get(self.row).and_then(|row| {
            row.iter()
                .skip(self.col)
                .find(|tile| in_perimeter(self.row, &tile.range, self.pivot))
        });
        match next {
            Some(next) => {
                self.col += 1;
                Some(next)
            }
            None => {
                self.row += 1;
                self.col = 0;
                self.next()
            }
        }
    }
}

fn make_engine(input: &str) -> Engine {
    const fn to_tile(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            num if let Some(num) = num.to_digit(10) => Tile::Number(num),
            _ => Tile::Symbol,
        }
    }
    fn append_tile(acc: Vec<EngineTile>, (idx, tile): (u32, Tile)) -> Vec<EngineTile> {
        match acc.last() {
            Some(last) => match (last.tile, tile) {
                // same as last and is number
                (Tile::Number(last_value), Tile::Number(current_value)) => [
                    &acc[..acc.len() - 1],
                    &[EngineTile::new(
                        Tile::Number(last_value * 10 + current_value),
                        last.range.start..last.range.end + 1,
                        last.row,
                    )],
                ]
                .concat(),
                // same as last but is something else
                (last_tile, current_tile) if last_tile == current_tile => [
                    &acc[..acc.len() - 1],
                    &[EngineTile::new(
                        tile,
                        last.range.start..last.range.end + 1,
                        last.row,
                    )],
                ]
                .concat(),
                // different from last
                _ => [&acc[..], &[EngineTile::new(tile, idx..idx + 1, last.row)]].concat(),
            },
            _ => [acc, vec![EngineTile::new(tile, 0..1, 0)]].concat(),
        }
    }
    fn to_engine_row((idx, line): (usize, &str)) -> Vec<EngineTile> {
        line.trim()
            .chars()
            .enumerate()
            .map(|(idx, c)| (idx as u32, to_tile(c)))
            .fold(Vec::<EngineTile>::default(), append_tile)
    }
    let engine = input
        .lines()
        .enumerate()
        .map(to_engine_row)
        .collect::<Vec<_>>();
    Engine(engine)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "
		467..114..
		...*......
		..35..633.
		......#...
		617*......
		.....+.58.
		..592.....
		......755.
		...$.*....
		.664.598..
	";

    #[test]
    fn test_make_engine() {
        initialize();
        let engine = make_engine(TEST);
        tracing::debug!(?engine);
        dbg!(engine);
    }

    #[test]
    fn test_perimeter_iter() {
        initialize();
        let engine = make_engine(TEST);
        let pivot = &engine.0[4][0];
        let perimeter = engine.perimeter_iter(pivot).collect::<Vec<_>>();
        tracing::debug!(?perimeter);
        dbg!(perimeter);
    }
}
