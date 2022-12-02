use std::fs;
#[derive(Debug,Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}
#[derive(PartialEq, Debug)]
enum GameState {
    Win,
    Lose,
    Tie,
}
impl RPS {
    fn choice(choice: &str) -> RPS {
        match choice {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => Self::Scissors,
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => Self::Scissors,
            _ => panic!("invalid game"),
        }
    }
    fn vs(&self, other: &Self) -> GameState {
        use GameState::*;
        use RPS::*;
        match (self, other) {
            (Rock, Scissors) => Win,
            (Rock, Paper) => Lose,
            (Rock, Rock) => Tie,
            (Paper, Rock) => Win,
            (Paper, Paper) => Tie,
            (Paper, Scissors) => Lose,
            (Scissors, Rock) => Lose,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Tie,
        }
    }
        fn loses(&self) -> RPS {
        use RPS::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
        fn wins(&self) -> RPS {
        use RPS::*;
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }
}
fn tally_score(choice: &RPS, state: GameState) -> i32 {
    let choice_score = match choice {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };
    let game_score = match state {
        GameState::Lose => 0,
        GameState::Tie => 3,
        GameState::Win => 6,
    };
    choice_score + game_score
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vs_tie() {
        let yours = RPS::choice("X");
        let theirs = RPS::choice("A");
        yours.vs(&theirs);

        assert_eq!(yours.vs(&theirs), GameState::Tie);
    }
    #[test]
    fn vs_lose() {
        let yours = RPS::choice("X");
        let theirs = RPS::choice("B");
        yours.vs(&theirs);

        assert_eq!(yours.vs(&theirs), GameState::Lose);
    }
    #[test]
    fn vs_win() {
        let yours = RPS::choice("X");
        let theirs = RPS::choice("C");
        yours.vs(&theirs);

        assert_eq!(yours.vs(&theirs), GameState::Win);
    }
    #[test]
    fn game_win() {
        let yours = RPS::choice("X");
        let theirs = RPS::choice("C");
        let gamestate = yours.vs(&theirs);
        let score = tally_score(&yours, gamestate);
        assert_eq!(score, 7);
    }
    #[test]
    fn example_test_part1() {
            let input = "A Y
B X
C Z";
    let games:Vec<Vec<RPS>> = input.split("\n").map(|a|a.split(" ").map(|letter|RPS::choice(letter)).collect()).collect();
    let mut score = 0;
    for game in games{
        let mut game_iter = game.iter();
        let opponent_move = game_iter.next().unwrap();
        let your_move = game_iter.next().unwrap();
        let game_state= your_move.vs(opponent_move);
        let game_score = tally_score(your_move,game_state);
        score+=game_score
    }
        assert_eq!(score,15)
    }
    #[test]
    fn example_test_part2() {
            let input = "A Y
B X
C Z";
    let games:Vec<Vec<&str>> = input.split("\n").map(|a|a.split(" ").collect()).collect();
    let mut score = 0;
    for game in games{
        let mut game_iter = game.iter();
        let opponent_move = RPS::choice(game_iter.next().unwrap());
        let your_outcome = *game_iter.next().unwrap();
        let needed_state = match your_outcome {
                "X" => GameState::Lose,
                "Y" => GameState::Tie,
                "Z" => GameState::Win,
                _ => panic!("not a vaild gamestate")
            };
        let needed_move = match needed_state {
                GameState::Lose => opponent_move.wins(),
                GameState::Tie => opponent_move.clone(),
                GameState::Win => opponent_move.loses(),

            };
            let game_state = needed_move.vs(&opponent_move);
            let game_score = tally_score(&needed_move,game_state);
             score+=game_score
    }
        assert_eq!(score,12)
    }
}
fn main() {
    let input = fs::read_to_string("inputs/day2.txt").unwrap();
        let games:Vec<Vec<&str>> = input.split("\n").map(|a|a.split(" ").collect()).collect();
    let mut score = 0;
    for game in games{
        if game.len() != 2{
            continue
        }
        let mut game_iter = game.iter();
        let opponent_move = RPS::choice(game_iter.next().unwrap());
        let your_outcome = *game_iter.next().unwrap();
        let needed_state = match your_outcome {
                "X" => GameState::Lose,
                "Y" => GameState::Tie,
                "Z" => GameState::Win,
                _ => panic!("not a vaild gamestate")
            };
        let needed_move = match needed_state {
                GameState::Lose => opponent_move.wins(),
                GameState::Tie => opponent_move.clone(),
                GameState::Win => opponent_move.loses(),

            };
            let game_state = needed_move.vs(&opponent_move);
            let game_score = tally_score(&needed_move,game_state);
             score+=game_score
    }
    dbg!(score);
}
