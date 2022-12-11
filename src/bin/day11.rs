const EXAMPLE_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
#[derive(Debug, Clone)]
struct Monkey<'a> {
    items: Vec<i64>,
    divide_num: i64,
    throw_true: usize,
    throw_false: usize,
    operation: (&'a str, &'a str),
    inspects: i64,
}
use itertools::Itertools;
fn get_monkeys(input: &String) -> Vec<Monkey> {
    let chunks = input.split("\n\n");
    let mut monkeys: Vec<Monkey> = vec![];
    for chunk in chunks {
        let lines = chunk.lines().skip(1).collect_vec();
        let items = lines[0][18..]
            .split(", ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect_vec();
        let operation = lines[1][23..].split_once(' ').unwrap();
        let divide_num = lines[2][21..].parse::<i64>().unwrap();
        let throw_true = lines[3][29..].parse::<usize>().unwrap();
        let throw_false = lines[4][30..].parse::<usize>().unwrap();
        monkeys.push(Monkey {
            items,
            divide_num,
            throw_false,
            throw_true,
            operation,
            inspects: 0,
        });
    }
    monkeys
}
fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn solution(input: &String, part_one_bool: bool) -> i64 {
    let mut monkeys = get_monkeys(input);
    let range;
    let mut lcm = 1;
    range = match part_one_bool {
        true => 20,
        false => 10000,
    };
    for _ in 0..range {
        for i in 0..monkeys.len() {
            let items = std::mem::take(&mut monkeys[i].items);
            monkeys[i].inspects += items.len() as i64;
            for item in items {
                let mut worry = match monkeys[i].operation {
                    ("+", "old") => item + item,
                    ("*", "old") => item * item,
                    ("+", num) => item + num.parse::<i64>().unwrap(),
                    ("*", num) => item * num.parse::<i64>().unwrap(),
                    _ => panic!("failed operation"),
                };
                if part_one_bool {
                    worry /= 3;
                }
                let throw_to = match worry % monkeys[i].divide_num {
                    0 => monkeys[i].throw_true,
                    _ => monkeys[i].throw_false,
                };
                monkeys[throw_to].items.push(worry);
                lcm = lcm * monkeys[i].divide_num / gcd(lcm, monkeys[i].divide_num);
            }
        }
        if !part_one_bool {
            for m in &mut monkeys {
                m.items.iter_mut().for_each(|i| *i %= lcm);
            }
        }
    }
    monkeys.sort_by_key(|m| -m.inspects);
    monkeys[0].inspects * monkeys[1].inspects
}

fn main() {}
#[test]
fn test_part_one_example() {
    assert_eq!(10605, solution(&EXAMPLE_INPUT.to_owned(), true))
}
#[test]
fn test_part_two_example() {
    assert_eq!(2713310158, solution(&EXAMPLE_INPUT.to_owned(), false))
}
#[test]
fn test_part_one_input() {
    assert_eq!(
        120756,
        solution(&std::fs::read_to_string("inputs/day11.txt").unwrap(), true)
    )
}
#[test]
fn test_part_two_input() {
    assert_eq!(
        39109444654,
        solution(&std::fs::read_to_string("inputs/day11.txt").unwrap(), false)
    )
}
