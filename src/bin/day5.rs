use itertools::Itertools;
use std::fs::read_to_string;
const EXAMPLE_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
fn move_crates(part: u8) -> String {
    let input: String = read_to_string("inputs/day5.txt").unwrap();
    let data: Vec<&str> = input.split("\n\n").collect();
    let lines: Vec<&str> = data[0].split("\n").collect();
    let row_count = lines.last().unwrap().split("  ").count();
    let mut crates: Vec<Vec<char>> = vec![vec![]; row_count];
    let character_range = 'A'..='Z';
    for i in lines {
        for (index, character) in i.chars().enumerate() {
            if character_range.contains(&character) {
                let row_num = index / 4;
                crates[row_num].insert(0,character);
            }
        }
    }
    for i in data[1].split("\n") {
        let (amount, from, to) = i
            .split(" ")
            .into_iter()
            .skip(1)
            .step_by(2)
            .map(|num| num.parse::<usize>().unwrap())
            .collect_tuple::<(usize, usize, usize)>()
            .unwrap();
        let mut to_work = Vec::<char>::new();
        for _ in 0..amount {
            to_work.push(crates[from - 1].pop().unwrap())
        }
        if part == 1 {
            for c in to_work.iter() {
                crates[to - 1].push(*c);
            }
        } else {
            for c in to_work.iter().rev() {
                crates[to - 1].push(*c);
            }
        }
    }
    let mut answer = Vec::<char>::new();
    for (_, c) in crates.iter().enumerate() {
        answer.push(*c.last().unwrap());
    }
    answer.iter().join("")
}
fn main() {
    dbg!(move_crates(0));
}
