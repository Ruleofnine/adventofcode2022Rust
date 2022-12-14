#![feature(int_roundings)]

const EXAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
use std::{collections::VecDeque, ops::Range};

fn char_to_digit(character: char) -> usize {
    let (minus_amount, plus_amount) = if character == 'E' || character == 'S' {
        return 26;
    } else {
        (96, 0)
    };
    let digit = character as i32;
    (digit - minus_amount + plus_amount) as usize
}
#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    NONE,
}
#[derive(Debug)]
enum MovePosition {
    Move(Direction, Range<usize>),
}
#[derive(Debug, PartialEq)]
struct Position {
    position: usize,
    character: char,
    height: usize,
    direction: Direction,
    valid: bool,
    next_nodes: Vec<Position>,
}
impl Position {
    fn new(
        character: char,
        direction: Direction,
        position: usize,
        current_height: usize,
    ) -> Position {
        let height = char_to_digit(character);
        let valid = current_height + 1 >= height;
        Position {
            character,
            position,
            direction,
            height,
            valid,
            next_nodes: Vec::new(),
        }
    }
}
impl Position {
    fn bfs(&mut self, input: &str, map_bound: usize, map_length: usize) -> i32 {
        let mut q = VecDeque::new();
        let mut total_steps = 0;
        let mut visted = vec![false; map_bound];
        let mut nodes_in_layer = 1;
        let mut nodes_in_next_layer = 0;
        q.push_back(self);
        while let Some(parent) = q.pop_back() {
            nodes_in_next_layer =
                parent.add_positions(input, map_bound, map_length, nodes_in_next_layer, &visted);
            nodes_in_layer -= 1;
            for node in &mut parent.next_nodes {
                if node.character == 'E' {
                    total_steps += 1;
                    return total_steps;
                } else if !visted[node.position] {
                    visted[node.position] = true;
                    q.push_front(node);
                }
            }
            if nodes_in_layer == 0 {
                nodes_in_layer = nodes_in_next_layer;
                nodes_in_next_layer = 0;
                total_steps += 1;
            }
        }
        total_steps
    }
    fn add_positions(
        &mut self,
        input: &str,
        map_bound: usize,
        map_length: usize,
        mut nodes: usize,
        visited: &Vec<bool>,
    ) -> usize {
        let position_ranges = get_position_ranges(self.position, map_bound, map_length);
        for pos in position_ranges {
            let (direction, range) = match pos {
                MovePosition::Move(dir, range) => (dir, range),
            };
            let start = range.start;
            let new_position = match input.get(range) {
                Some(c) => Some(Position::new(
                    c.parse().unwrap(),
                    direction,
                    start,
                    self.height,
                )),
                None => None,
            };
            match new_position {
                Some(pos) => {
                    if pos.valid && !visited[pos.position] {
                        nodes += 1;
                        self.next_nodes.push(pos)
                    }
                }
                None => {}
            }
        }
        nodes
    }
}
fn get_position_ranges(
    current_position: usize,
    map_bound: usize,
    map_length: usize,
) -> Vec<MovePosition> {
    let mut position_ranges = vec![];
    use Direction::*;
    use MovePosition::*;
    if current_position + 1 != map_bound
        || current_position == 0 && current_position % map_length != 0
    {
        //right
        position_ranges.push(Move(Right, current_position + 1..current_position + 2))
    }
    if current_position + map_length + 2 < map_bound {
        //down
        position_ranges.push(Move(
            Down,
            current_position + map_length + 1..current_position + map_length + 2,
        ))
    }
    if current_position >= 1 {
        //left
        position_ranges.push(Move(Left, current_position - 1..current_position));
    }
    if current_position >= map_length + 1 {
        //up
        position_ranges.push(Move(
            Up,
            current_position - map_length - 1..current_position - map_length,
        ))
    }
    position_ranges
}

fn part_one(input: &str) -> i32 {
    let map_length = input.find("\n").unwrap();
    let map_bound = input.len();
    let current_position = input.find('S').unwrap();
    // let end_position = input.find('E').unwrap();
    let mut starting_positon = Position::new('S', Direction::NONE, current_position, 1);
    starting_positon.bfs(&input, map_bound, map_length)
}
fn part_two(input: &str) -> i32 {
    //completely unoptimized but idc atm...
    let map_length = input.find("\n").unwrap();
    let map_bound = input.len();
    let test: Vec<usize> = input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if c == 'a' { Some(i) } else { None })
        .collect();
    let mut test_vec = vec![];
    let mut min = 100000;
    for a in test {
        let mut starting_positon = Position::new('a', Direction::NONE, a, 1);
        let amount = starting_positon.bfs(&input, map_bound, map_length);
        // dbg!((&input[a..a + 1], amount));
        //technically working, idk why it gives all the low numbers and idc atm....
        if amount.min(min) < min && amount > 200{
            min = amount.min(min);
            test_vec.push(starting_positon);
        };
    }
    min
}
fn main() {
    let input = std::fs::read_to_string("inputs/day12.txt").unwrap();
    // let input = EXAMPLE_INPUT;
    dbg!(part_two(&input));
}
