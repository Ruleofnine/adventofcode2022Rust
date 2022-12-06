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

fn move_crates(part: u8,input:String) -> String {
    let data: Vec<&str> = input.split("\n\n").collect();
    let lines: Vec<&str> = data[0].split("\n").collect();
    let row_count = lines.last().unwrap().split("  ").count();
    let mut crates: Vec<Vec<char>> = vec![vec![]; row_count];
    for i in lines {
        for (index, character) in i.chars().enumerate().skip(1).step_by(4) {
            if character.is_alphabetic() {
                let row_num = index / 4;
                crates[row_num].insert(0, character)
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
    crates.iter().map(|c| c.last().unwrap()).collect::<String>()
}
fn main() {
    let input: String = read_to_string("inputs/day5.txt").unwrap();
    //part one
    dbg!(move_crates(1,input.clone()));
    //part two
    dbg!(move_crates(2,input));
}
#[test]
fn test_part_one(){
    assert_eq!("CMZ",move_crates(1,EXAMPLE_INPUT.to_owned()))
}
#[test]
fn test_part_two(){
    assert_eq!("MCD",move_crates(2,EXAMPLE_INPUT.to_owned()))

}
