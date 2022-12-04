use std::fs;
fn main() {
    dbg!(part_one());
    dbg!(part_two());
}
fn part_one()-> i32 {
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

}fn part_two()-> i32 {
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
        if list1.contains(&start_two) || list1.contains(&end_two) {
            amount += 1;
            continue;
        }
        if list2.contains(&start_one) || list2.contains(&end_one) {
            amount += 1;
            continue;
        }
    }
    amount

}
