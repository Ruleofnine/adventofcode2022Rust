const EXAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
// Sabqponm
// abcryxxl
// accszExk
// acctuvwj
// abdefghi

use std::collections::VecDeque;
fn calculate_height(character: char) -> i32 {
    let (minus_amount, plus_amount) = if character.is_uppercase() {
        return 26;
    } else {
        (96, 0)
    };
    let digit = character as i32;
    digit - minus_amount + plus_amount
}

fn bfs(
    map: &Vec<Vec<char>>,
    start: (usize, usize),
    row_length: usize,
    column_length: usize,
    searching_for: char,
) -> i32 {
    let mut rq: VecDeque<usize> = VecDeque::new();
    let mut cq: VecDeque<usize> = VecDeque::new();
    let dr: Vec<i32> = vec![-1, 1, 0, 0];
    let dc: Vec<i32> = vec![0, 0, 1, -1];
    let mut visited = vec![vec![false; column_length]; row_length];
    rq.push_back(start.0);
    cq.push_back(start.1);
    visited[start.0][start.1] = true;
    let mut move_count = 0;
    let mut nodes_left_in_layer = 1;
    let mut nodes_in_next_layer = 0;
    while !rq.is_empty() {
        let r = rq.pop_back().unwrap();
        let c = cq.pop_back().unwrap();
        if map[r][c] == searching_for {
            return move_count;
        }
        let current_height = calculate_height(map[r][c]);
        nodes_left_in_layer -= 1;
        for i in 0..4 {
            let rr = r as i32 + dr[i];
            let cc = c as i32 + dc[i];
            if rr < 0 || cc < 0 || rr >= row_length as i32 || cc >= column_length as i32 {
                continue;
            }
            let position = map[rr as usize][cc as usize];
            let new_height = calculate_height(position);
            if new_height - current_height > 1 || visited[rr as usize][cc as usize] {
                continue;
            }
            rq.push_front(rr as usize);
            cq.push_front(cc as usize);
            visited[rr as usize][cc as usize] = true;
            nodes_in_next_layer += 1;
        }
        if nodes_left_in_layer == 0 {
            nodes_left_in_layer = nodes_in_next_layer;
            nodes_in_next_layer = 0;
            move_count += 1;
        }
    }
    1000
}
fn main() {
    let input = std::fs::read_to_string("inputs/day12.txt").unwrap();
    // let input = EXAMPLE_INPUT;
    dbg!(part_two(&input));
}
fn part_two(input: &str) -> i32 {
    //unoptimzied, idc atm.
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let column_length = map[0].len();
    let row_length = map.len();
    let mut a_s = vec![];
    for (ri, row) in map.iter().enumerate() {
        for (ci, c) in row.iter().enumerate() {
            if c == &'a' {
                a_s.push((ri, ci));
            }
        }
    }
    let mut amount = 100000;
    for a in a_s {
        amount = bfs(&map, a, row_length, column_length, 'E').min(amount);
        dbg!(amount);
    }
    amount
}
fn part_one(input: &str) -> i32 {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let column_length = map[0].len();
    let row_length = map.len();
    let mut start_found = false;
    let mut end_found = false;
    let mut start_position = (1, 1);
    for (ri, row) in map.iter().enumerate() {
        if start_found{
            break;
        }
        for (ci, c) in row.iter().enumerate() {
            if !start_found && c == &'S' {
                start_found = true;
                start_position = (ri, ci)
            }
        }
    }
    bfs(&map, start_position, row_length, column_length, 'E')
}
#[test]
fn part_one_test(){
    assert_eq!(339,part_one(& std::fs::read_to_string("inputs/day12.txt").unwrap()));
    assert_eq!(31,part_one(&EXAMPLE_INPUT));
}
#[test]
fn part_two_test(){
    assert_eq!(332,part_two(& std::fs::read_to_string("inputs/day12.txt").unwrap()));
    assert_eq!(29,part_two(&EXAMPLE_INPUT));
}
