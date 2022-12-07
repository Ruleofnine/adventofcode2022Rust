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
fn part_one(input: String) -> i32 {
    let mut dirs = std::collections::HashMap::new();
    let mut pwd = vec![];
    input.lines().for_each(|line| {
        if let Some(dir) = line.strip_prefix("$ cd ") {
            match dir {
                ".." => {
                    pwd.pop();
                }
                r"/" => pwd = vec!["/"],
                _ => {
                    pwd.push(dir);
                    dirs.entry(dir).or_insert(0);
                }
            };
        } else {
            if let Ok(size) = line.split_once(" ").unwrap().0.parse::<i32>() {
                for f in &pwd {
                    dirs.entry(f).and_modify(|e| *e += size);
                }
            }
        }
    });
    // dirs.into_values().fold(0, |acc, v| {
        // dbg!(acc,v);
        // acc + match v {
            // i if i < 100_000 => i,
            // _ => 0,
        // }}
    let test:Vec<i32>= dirs.into_values().collect();
    test.iter().filter(|n| **n<=100_000).sum()
}
fn main() {
    let _input = std::fs::read_to_string("inputs/day7_2.txt").unwrap();
    println!("sum: {}", part_one(_input));
    println!("sum: {}", part_one(EXAMPLE_INPUT.to_string()));
}
#[test]
fn part_one_test() {
    assert_eq!(95437, part_one(EXAMPLE_INPUT.to_string()));
    assert_eq!(24933642, part_one(std::fs::read_to_string("inputs/day7_2.txt").unwrap()))
}
