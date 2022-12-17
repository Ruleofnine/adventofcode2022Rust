const EXAMPLE_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, newline, none_of},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    ToUsize, *,
};
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Range;
fn parse_tuple(input: &str) -> IResult<&str, BTreeSet<(u32, u32)>> {
    let (input, pairs) = separated_list1(
        newline,
        separated_list1(
            tag(" -> "),
            separated_pair(
                nom::character::complete::u32,
                nom::character::complete::char(','),
                nom::character::complete::u32,
            ),
        ),
    )(input)?;
    let parsed = pairs
        .into_iter()
        .map(|l| {
            l.into_iter()
                .tuple_windows()
                .flat_map(|((ax, ay), (bx, by))| {
                    let x_min = ax.min(bx);
                    let x_max = ax.max(bx);
                    let x_range = x_min..=x_max;
                    let y_min = ay.min(by);
                    let y_max = ay.max(by);
                    let y_range = y_min..=y_max;
                    x_range.cartesian_product(y_range)
                })
        })
        .flatten()
        .collect::<BTreeSet<(u32, u32)>>();
    Ok((input, parsed))
}
fn part_one(input: &str) -> usize {
    let (_, mut cave) = parse_tuple(input).unwrap();
    let start_count = cave.len();
    let lowest_rock = cave.iter().fold(0, |state, i| state.max(i.1)) + 2;
    let mut current_sand = (500, 0);
    loop {
        if current_sand.1 > lowest_rock {
            break;
        }
        let down = (current_sand.0, current_sand.1 + 1);
        let left = (current_sand.0 - 1, current_sand.1 + 1);
        let right = (current_sand.0 + 1, current_sand.1 + 1);
        match (cave.get(&down), cave.get(&left), cave.get(&right)) {
            (Some(_), Some(_), Some(_)) => {
                cave.insert(current_sand);
                current_sand = (500, 0)
            }
            (None, _, _) => current_sand = down,
            (_, None, _) => current_sand = left,
            (_, _, None) => current_sand = right,
        }
    }
    cave.len() - start_count
}
fn part_two(input: &str) -> usize {
    let (_, mut cave) = parse_tuple(input).unwrap();
    let start_count = cave.len();
    let lowest_rock = cave.iter().fold(0, |state, i| state.max(i.1)) + 2;
    let mut current_sand = (500, 0);
    while let None = cave.get(&(500, 0)) {
        let down = (current_sand.0, current_sand.1 + 1);
        let left = (current_sand.0 - 1, current_sand.1 + 1);
        let right = (current_sand.0 + 1, current_sand.1 + 1);
        match (
            cave.get(&down).or_else(|| {
                if down.1 == lowest_rock {
                    Some(&(0, 0))
                } else {
                    None
                }
            }),
            cave.get(&left).or_else(|| {
                if left.1 == lowest_rock {
                    Some(&(0, 0))
                } else {
                    None
                }
            }),
            cave.get(&right).or_else(|| {
                if right.1 == lowest_rock {
                    Some(&(0, 0))
                } else {
                    None
                }
            }),
        ) {
            (Some(_), Some(_), Some(_)) => {
                cave.insert(current_sand);
                current_sand = (500, 0)
            }
            (None, _, _) => current_sand = down,
            (_, None, _) => current_sand = left,
            (_, _, None) => current_sand = right,
        }
    }
    cave.len() - start_count
}

fn main() {
    dbg!(part_two(EXAMPLE_INPUT));
}
#[test]
fn part_one_test() {
    assert_eq!(24, part_one(EXAMPLE_INPUT));
    assert_eq!(
        638,
        part_one(read_to_string("inputs/day14.txt").unwrap().as_str())
    );
}#[test]
fn part_two_test() {
    assert_eq!(93, part_two(EXAMPLE_INPUT));
    assert_eq!(
        31722,
        part_two(read_to_string("inputs/day14.txt").unwrap().as_str())
    );
}
