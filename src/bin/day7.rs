const EXAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
use std::collections::BTreeMap;
fn day7(input: String) -> (i32, i32) {
    let mut dirs: BTreeMap<String, i32> = BTreeMap::from([("/".to_string(), 0)]);
    let mut pwd = vec!["/"];
    input
        .lines()
        .for_each(|line| match line.split(" ").collect::<Vec<&str>>()[..] {
            ["$", "cd", "/"] => pwd = vec!["/"],
            ["$", "cd", ".."] => _ = pwd.pop(),
            ["$", "ls"] => {}
            ["dir", _] => {}
            ["$", "cd", folder_name] => {
                pwd.push(folder_name);
                dirs.entry(pwd.join("/")).or_default();
            }
            [size, _file_name] => pwd.iter().enumerate().for_each(|(i, _)| {
                dirs.entry(pwd[..pwd.len() - i].join("/"))
                    .and_modify(|e| *e += size.parse::<i32>().unwrap());
            }),
            _ => panic!("failed to parse line"),
        });
    let available = 70_000_000 - dirs.get("/").unwrap();
    let smallest = dirs
        .clone()
        .into_values()
        .filter(|e| e + available > 30_000_000)
        .min()
        .unwrap();
    (
        dirs.into_values()
            .filter(|n| *n <= 100_000)
            .fold(0, |acc, v| acc + v),
        smallest,
    )
}
fn main() {
    let (part_one, part_two) = day7(std::fs::read_to_string("inputs/day7.txt").unwrap());
    println!("Part One: {}\nPart Two: {}",part_one,part_two)
}
#[test]
fn example() {
    let (part_one, part_two) = day7(EXAMPLE_INPUT.to_string());
    assert_eq!(95437, part_one);
    assert_eq!(24933642, part_two);
}
#[test]
fn part_one_test() {
    let (part_one, part_two) = day7(std::fs::read_to_string("inputs/day7.txt").unwrap());
    assert_eq!(1444896, part_one);
    assert_eq!(404395, part_two);
}
