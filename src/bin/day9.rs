const EXAMPLE_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
const EXAMPLE_INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
use std::cell::{Ref, RefCell};
use std::rc::Rc;
fn check_tail(head: &(i32, i32), tail: &mut (i32, i32)) {
    let x_dif = head.0 - tail.0;
    let y_dif = head.1 - tail.1;
    match (x_dif, y_dif) {
        (0, 0) => {}
        (1, 0) => {}
        (2, 0) => tail.0 += 1,
        (0, 1) => {}
        (0, 2) => tail.1 += 1,
        (1, 1) => {}
        (1, 2) => {
            tail.0 += 1;
            tail.1 += 1
        }
        (-1, 1) => {}
        (-2, 1) => {
            tail.0 -= 1;
            tail.1 += 1
        }
        (-2, 0) => tail.0 -= 1,
        (2, 1) => {
            tail.0 += 1;
            tail.1 += 1
        }
        (1, -1) => {}
        (0, -1) => {}
        (-1, -1) => {}
        (-2, -1) => {
            tail.0 -= 1;
            tail.1 -= 1
        }
        (2, -1) => {
            tail.0 += 1;
            tail.1 -= 1;
        }
        (-1, 0) => {}
        (-1, 2) => {
            tail.0 -= 1;
            tail.1 += 1
        }
        (0, -2) => tail.1 -= 1,
        (1, -2) => {
            tail.0 += 1;
            tail.1 -= 1
        }
        (-1, -2) => {
            tail.0 -= 1;
            tail.1 -= 1
        }
        _ => panic!("illegal move?"),
    }
}

use indexmap::IndexMap;
use std::collections::HashMap;
fn part_one(input: String) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut points: HashMap<(i32, i32), ()> = HashMap::from([(head, ())]);
    input.lines().for_each(|l| {
        let (direction, amount) = l.split_once(' ').unwrap();
        for _ in 0..amount.parse::<i8>().unwrap() {
            match direction {
                "R" => head.0 += 1,
                "L" => head.0 -= 1,
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                _ => panic!("unexpected direction"),
            }

            check_tail(&head, &mut tail);
            points.entry(tail).or_insert(());
        }
    });
    points.len()
}
#[derive(Debug)]
struct Snake {
    head: (i32, i32),
    nodes: Vec<(i32, i32)>,
    tail_visisted: IndexMap<(i32, i32), ()>,
    tail_length: usize,
}
impl Snake {
    fn check_nodes(&mut self) {
        self.nodes
            .iter_mut()
            .enumerate()
            .scan(&self.head, |state, (index, mut tail)| {
                let x_dif = state.0 - tail.0;
                let y_dif = state.1 - tail.1;
                match (x_dif, y_dif) {
                    (0, 0) => {}
                    (1, 0) => {}
                    (2, 0) => tail.0 += 1,
                    (0, 1) => {}
                    (0, 2) => tail.1 += 1,
                    (1, 1) => {}
                    (1, 2) => {
                        tail.0 += 1;
                        tail.1 += 1
                    }
                    (-1, 1) => {}
                    (-2, 1) => {
                        tail.0 -= 1;
                        tail.1 += 1
                    }
                    (-2, 0) => tail.0 -= 1,
                    (2, 1) => {
                        tail.0 += 1;
                        tail.1 += 1
                    }
                    (1, -1) => {}
                    (0, -1) => {}
                    (-1, -1) => {}
                    (-2, -1) => {
                        tail.0 -= 1;
                        tail.1 -= 1
                    }
                    (2, -1) => {
                        tail.0 += 1;
                        tail.1 -= 1;
                    }
                    (-1, 0) => {}
                    (-1, 2) => {
                        tail.0 -= 1;
                        tail.1 += 1
                    }
                    (0, -2) => tail.1 -= 1,
                    (1, -2) => {
                        tail.0 += 1;
                        tail.1 -= 1
                    }
                    (-1, -2) => {
                        tail.0 -= 1;
                        tail.1 -= 1
                    }
                    (2, -2) => {
                        tail.0 += 1;
                        tail.1 -= 1
                    }
                    (2, 2) => {
                        tail.0 += 1;
                        tail.1 += 1
                    }
                    (-2, 2) => {
                        tail.0 -= 1;
                        tail.1 += 1
                    }
                    (-2, -2) => {
                        tail.0 -= 1;
                        tail.1 -= 1
                    }

                    _ => panic!("illegal move? {}, {}", x_dif, y_dif),
                };
                if index + 1 == self.tail_length {
                    println!("END{index} ({},{})", &tail.0, &tail.1);
                    self.tail_visisted.entry(*tail).or_insert(());
                } else {
                    println!("node{index} ({},{})", &tail.0, &tail.1);
                }
                *state = tail;
                Some(*state)
            })
            .count();
    }
}
fn part_two(input: String) -> usize {
    let mut snake = Snake {
        head: (0, 0),
        nodes: vec![(0, 0); 9],
        tail_visisted: IndexMap::from([((0, 0), ())]),
        tail_length: 9,
    };
    // let mut points: HashMap<(i32, i32), ()> = HashMap::from([((0, 0), ())]);
    input.lines().for_each(|l| {
        let (direction, amount) = l.split_once(' ').unwrap();
        println!("------------");
        println!("Direction: {direction},Amount:{amount}");
        println!("------------");
        for _ in 0..amount.parse::<i8>().unwrap() {
            match direction {
                "R" => snake.head.0 += 1,
                "L" => snake.head.0 -= 1,
                "U" => snake.head.1 += 1,
                "D" => snake.head.1 -= 1,
                _ => panic!("unexpected direction"),
            }
            println!("HEAD: ({},{})", snake.head.0, snake.head.1);
            snake.check_nodes();
            println!("------------");
        }
    });
    dbg!(snake.tail_visisted.len());
    snake.tail_visisted.len()
}
// }
// });
// points.len()
// dbg!(dir);
// dbg!(head_point, tail_point);
// if head_point.0 > tail_point.0 + 1 {
//     tail_point.0 += 1
// } else if head_point.1 > tail_point.1 + 1 {
//     tail_point.1 += 1
// }
// check_tail(&mut head_point, &mut tail_point);
// points.entry(tail_point).or_insert(true);
// }

fn main() {
    part_two(std::fs::read_to_string("inputs/day9.txt").unwrap());
}
// #[test]
// fn example_part_one() {
//     assert_eq!(13, part_one(EXAMPLE_INPUT.to_owned()))
// }
#[test]
fn part_one_test() {
    assert_eq!(
        6256,
        part_one(std::fs::read_to_string("inputs/day9.txt").unwrap())
    )
}
#[test]
fn part_two_example() {
    assert_eq!(36, part_two(EXAMPLE_INPUT2.to_owned()));
}
#[test]
fn part_two_input() {
    assert_eq!(
        36,
        part_two(std::fs::read_to_string("inputs/day9.txt").unwrap())
    );
}
