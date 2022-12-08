const EXAMPLE_INPUT: &str = "$ cd /
$ ls
dir e
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
const EXAMPLE_INPUT_2: &str = "$ cd /
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
    let mut dirs: std::collections::BTreeMap<String, i32> = std::collections::BTreeMap::new();
    let mut pwd = vec![];
    input
        .lines()
        .for_each(|line| match line.split(" ").collect::<Vec<&str>>()[..] {
            ["$", "cd", "/"] => {
                pwd = vec!["root"];
            }
            ["$", "cd", ".."] => {pwd.pop();},
            ["$", "ls"] => (),
            ["dir", _] => (),
            ["$", "cd", folder_name] => {
                pwd.push(folder_name);
                dirs.entry(pwd.join("/")).or_default();
            }
            [size, _file_name] => {
                pwd.iter().enumerate().for_each(|(i, _)| {
                    // println!("file path: {}, size: {},file name: {_file_name}",&pwd[0..pwd.len() - i].join("/"), size);
                    dirs.entry(pwd[0..pwd.len()-i].join("/"))
                        .and_modify(|e| *e += size.parse::<i32>().unwrap());
                })
            }
            _ => panic!("failed to parse line"),
        });
    dirs
        .into_values()
        .filter(|n| *n <= 100_000)
        .fold(0, |acc, v| acc + v)
}
fn main() {
    let input = std::fs::read_to_string("inputs/day7_2.txt").unwrap();
    part_one(EXAMPLE_INPUT.to_string());
    part_one(input);
}
#[test]
fn part_one_example() {
    assert_eq!(95437, part_one(EXAMPLE_INPUT_2.to_string()));
}
#[test]
// #[ignore = "reason"]
fn part_one_test() {
    assert_eq!(
        24933642,
        part_one(std::fs::read_to_string("inputs/day7_2.txt").unwrap())
    )
}
