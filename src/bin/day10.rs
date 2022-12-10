const EXAMPLE_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
const EXAMPLE_INPUT_SMALL: &str = "noop
addx 3
addx -5
";
#[derive(Debug)]
struct System {
    cycles: i32,
    register: i32,
    signal_strength: i32,
    display: Vec<Vec<String>>,
    row_num: usize,
}
impl System {
    fn addx(&mut self, num: i32) {
        println!("Begin executing addx {}",num);
        self.increment_cycle();
        self.increment_cycle();
        self.increment_register(num);
    }
    fn increment_cycle(&mut self) {
        self.cycles += 1;
        println!("Start Cycle {}",self.cycles);
        let pixels = [self.register - 1, self.register, self.register + 1];
        println!("Cycles :{}, Register: {}",self.cycles,self.register);
        println!("Pixels ({}|{}|{}), {}",pixels[0],pixels[1],pixels[2],pixels.contains(&((self.cycles-1)%40)));
        let mut sprite = ["."; 40];
        sprite.iter_mut().enumerate().for_each(|(i, c)| {
            let index = i as i32;
            if pixels.contains(&index) {
                *c = "#"
            }
        });
        let symbol;
        if pixels.contains(&((self.cycles-1)%40)) {
            self.display[self.row_num].push("#".to_owned());
            symbol = "#";
        } else {
            self.display[self.row_num].push(".".to_owned());
            symbol = "."
        }
        println!("During cyle :{} CRT draws pixel \"{}\" in position {}",self.cycles,symbol,self.display[self.row_num].len()-1);
        println!("Sprite Position: {}",sprite.join(""));
        println!("Current CRT Row: {}",self.display[self.row_num].join(""));
        println!("----------");
        if (self.cycles % 40) == 0  && self.row_num != self.display.len()-1{
            dbg!(self.cycles, self.row_num);
            self.row_num += 1;
        }
        self.check_signal();
    }
    fn check_signal(&mut self) {
        if self.cycles == 20 || (self.cycles - 20) % 40 == 0 {
            self.signal_strength = self.signal_strength + (self.register * self.cycles)
        }
    }
    fn increment_register(&mut self, num: i32) {
        self.register += num;
        println!("End of cyle {} Finish executing addx {} (Register is now {})",self.cycles,num,self.register)
    }
}
fn main() {}
fn part_one(input: String) -> i32 {
    let mut system = System {
        cycles: 0,
        register: 1,
        signal_strength: 0,
        display: vec![Vec::new(); 6],
        row_num: 0,
    };
    input.lines().for_each(|l| {
        match l.split(' ').collect::<Vec<&str>>()[..] {
            ["addx", num] => system.addx(num.parse::<i32>().unwrap()),
            ["noop"] => system.increment_cycle(),
            _ => panic!("failed to parse"),
        };
    });
    for row in system.display{
        println!("{}",row.join(""));
    }
    system.signal_strength
}
#[test]
fn example_part_one() {
    assert_eq!(part_one(EXAMPLE_INPUT.to_owned()), 13140)
}
#[test]
fn part_one_test() {
    assert_eq!(part_one(std::fs::read_to_string("inputs/day10.txt").unwrap()),10760)
}
