use itertools::Itertools;
use std::fs::read_to_string;
const EXAMPLE_INPUT_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
const EXAMPLE_INPUT_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
const EXAMPLE_INPUT_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
const EXAMPLE_INPUT_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
const EXAMPLE_INPUT_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
fn get_marker(input: &str, win_size: usize) -> usize {
    let message: Vec<char> = input.as_bytes().iter().map(|a| *a as char).collect();
    let break_point: Vec<_> = message
        .windows(win_size)
        .enumerate()
        .filter_map(|(i, a)| {
            if a.into_iter().unique().count() == win_size {
                Some(i + win_size)
            } else {
                None
            }
        })
        .collect();
    break_point[0]
}
fn main() {
    let message = read_to_string("inputs/day6.txt").unwrap();
    dbg!(get_marker(message.as_str(), 4));
    dbg!(get_marker(message.as_str(), 14));
}
#[test]
fn example_part_one() {
    assert_eq!(get_marker(EXAMPLE_INPUT_1, 4), 7);
    assert_eq!(get_marker(EXAMPLE_INPUT_2, 4), 5);
    assert_eq!(get_marker(EXAMPLE_INPUT_3, 4), 6);
    assert_eq!(get_marker(EXAMPLE_INPUT_4, 4), 10);
    assert_eq!(get_marker(EXAMPLE_INPUT_5, 4), 11);
}
#[test]
fn example_part_two() {
    assert_eq!(get_marker(EXAMPLE_INPUT_1, 14), 19);
    assert_eq!(get_marker(EXAMPLE_INPUT_2, 14), 23);
    assert_eq!(get_marker(EXAMPLE_INPUT_3, 14), 23);
    assert_eq!(get_marker(EXAMPLE_INPUT_4, 14), 29);
    assert_eq!(get_marker(EXAMPLE_INPUT_5, 14), 26);
}
