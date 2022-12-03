use std::fs;
fn main() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    let lines: Vec<&str> = input.split("\n").collect();
    let team_amount = lines.len()/3;
    // let team1 = lines.iter().take(3);

}
fn part_one(){
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
    dbg!(priority_sum);

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
    // for section in sections {
        // let len = section.len() / 2;
        // let (sub1, sub2) = section.split_at(len);
        // for character in sub1.chars() {
            // if sub2.contains(character) {
                // priority_sum += char_to_digit(character);
                // break;
            // }
        // }
    // }
    // assert_eq!(157, priority_sum);
}
fn char_to_digit(character: char) -> i32 {
    let (minus_amount, plus_amount) = if character.is_uppercase() {
        (64, 26)
    } else {
        (96, 0)
    };
    let digit = character as u8;
    digit as i32 - minus_amount + plus_amount
}
