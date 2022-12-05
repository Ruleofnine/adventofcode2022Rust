use std::fs;
fn main() {
    dbg!(part_one());
    dbg!(part_two());
}
fn part_one() -> i32 {
    let input = fs::read_to_string("inputs/day4.txt").unwrap();
    let mut amount = 0;
    for i in input
        .replace(",", "\n")
        .lines()
        .map(|a| a.split_once("-").unwrap())
        .collect::<Vec<(&str, &str)>>()
        .chunks(2)
    {
        let start_one = i[0].0.parse::<i32>().unwrap();
        let end_one = i[0].1.parse::<i32>().unwrap();
        let start_two = i[1].0.parse::<i32>().unwrap();
        let end_two = i[1].1.parse::<i32>().unwrap();
        let list1 = start_one..=end_one;
        let list2 = start_two..=end_two;
        if list1.contains(&start_two) && list1.contains(&end_two) {
            amount += 1;
            continue;
        }
        if list2.contains(&start_one) && list2.contains(&end_one) {
            amount += 1;
            continue;
        }
    }
    amount
}

fn part_two() -> i32 {
    let input: Vec<Vec<i32>> = fs::read_to_string("inputs/day4.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split(&[',', '-'])
                .map(|a| a.parse().unwrap())
                .collect()
        })
        .collect();
    let mut amount = 0;
    for i in input {
        let list1 = i[0]..=i[1];
        let list2 = i[2]..=i[3];
        if list1.contains(list2.start()) || list1.contains(list2.end()) {
            amount += 1;
            continue;
        }
        if list2.contains(list1.start()) || list2.contains(list1.end()) {
            amount += 1;
        }
    }
    amount
}
// #[test]
// fn part_two_test() {
//     let input: Vec<Vec<i32>> = "2-4,6-8
// 2-3,4-5
// 5-7,7-9
// 2-8,3-7
// 6-6,4-6
// 2-6,4-8
// "
//     .lines()
//     .map(|line| {
//         let (a,b,c,d) = line.split(&[',', '-'])
//             .map(|a| a.parse().unwrap())
//             .collect()
//     })
//     .collect();
//     dbg!(input);
// }
