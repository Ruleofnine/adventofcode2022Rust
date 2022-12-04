#![feature(iter_array_chunks)]
// Did myself then watched youtube video for extra guidance https://www.youtube.com/watch?v=yBJJYkC5cdk
use std::fs;

fn main() {
    dbg!(part_one());
    dbg!(part_two());
    dbg!(part_two_try2());
//
}
//alternative interesting method using pattern binding found in the youtube comments
fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("Unacceptable character {c} !"),
    }
}
fn char_to_digit(character: char) -> u32 {
    let (minus_amount, plus_amount) = if character.is_uppercase() {
        (64, 26)
    } else {
        (96, 0)
    };
    let digit = character as u32;
    digit - minus_amount + plus_amount
}
fn part_one() -> u32 {
    let input = fs::read_to_string("inputs/day3.txt").unwrap();
    let sections: Vec<&str> = input.split("\n").collect();
    let mut priority_sum = 0;
    for section in sections {
        let len = section.len() / 2;
        let (sub1, sub2) = section.split_at(len);
        for character in sub1.chars() {
            if sub2.contains(character) {
                priority_sum += char_to_digit(character);
                break;
            }
        }
    }
    priority_sum
}
//My original implementation
fn part_two() -> u32 {
    let input = fs::read_to_string("inputs/day3.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut priority_sum = 0;
    for (index, line) in lines.iter().enumerate().skip(2).step_by(3) {
        for character in line.chars() {
            if lines[index - 1].contains(character) && lines[index - 2].contains(character) {
                priority_sum += char_to_digit(character);
                break;
            }
        }
    }
    priority_sum
}
//Using nightly feature and example from youtube video
fn part_two_try2() -> u32{
    let input = fs::read_to_string("inputs/day3.txt").unwrap();
    let sum = input
        .lines()
        .array_chunks()
        .map(|[a, b, c]| {
            char_to_digit(
                a.chars()
                    .find(|a_char| b.contains(*a_char) && c.contains(*a_char))
                    .unwrap(),
            )
        })
        .sum();
    sum
}
#[test]
fn test_char_to_digit() {
    let char_test = 'B';
    let digit = char_test as u8;
    let minus_amount = if char_test.is_uppercase() { 64 } else { 96 };
    assert_eq!(digit - minus_amount, 2)
}
#[test]
fn test_example1() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    let sections: Vec<&str> = input.split("\n").collect();
    let mut priority_sum = 0;
    for section in sections {
        let len = section.len() / 2;
        let (sub1, sub2) = section.split_at(len);
        for character in sub1.chars() {
            if sub2.contains(character) {
                priority_sum += char_to_digit(character);
                break;
            }
        }
    }
    assert_eq!(157, priority_sum);
}
#[test]
fn test_example2() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for (index, line) in lines.iter().enumerate().skip(2).step_by(3) {
        for character in line.chars() {
            if lines[index - 1].contains(character) && lines[index - 2].contains(character) {
                dbg!(character);
                sum += char_to_digit(character);
                break;
            }
        }
    }
    assert_eq!(70, sum);
}
