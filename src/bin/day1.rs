use std::fs::{File, self};
use std::io::prelude::*;
use std::path::Path;
fn max_calories() -> u32 {
    let data = fs::read_to_string("inputs/day1.txt").unwrap();
    data.split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap()
}
fn top3_calories(){
        let data = fs::read_to_string("inputs/day1.txt").unwrap();
        let mut test:Vec<u32> = data
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum::<u32>()).collect();
    test.sort_by(|a,b|b.cmp(a));
    let top3:u32 = test.iter().take(3).sum();
    dbg!(top3);
}
fn main() {
    dbg!(max_calories());
    dbg!(top3_calories());
}
#[test]
fn day_1_test_max() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    let test = input
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap();
    assert_eq!(24000, test)
}
#[test]
fn day_1_test_top3() {
    let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    let mut test:Vec<u32> = input
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<u32>().unwrap()).sum::<u32>()).collect();
    test.sort_by(|a,b|b.cmp(a));
    dbg!(&test);
    let top3:u32 = test.iter().take(3).sum();
    dbg!(test);
    assert_eq!(top3,45000);
}

