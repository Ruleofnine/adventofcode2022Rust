const EXAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";
use std::rc::Rc;

use take_until::TakeUntilExt;
#[derive(Debug)]
struct Grid {
    rows: Vec<Row>,
    columns: Vec<Column>,
    max_length: usize,
    total_visible: usize,
}
#[derive(Debug)]
struct Row {
    trees: Vec<Rc<Tree>>,
    max_height: usize,
}
impl CheckHieght for Row {
    fn check_height(&mut self, height: usize) {
        if height > self.max_height {
            self.max_height = height;
        }
    }
}
#[derive(Debug)]
struct Column {
    trees: Vec<Rc<Tree>>,
    max_height: usize,
}
impl CheckHieght for Column {
    fn check_height(&mut self, height: usize) {
        if height > self.max_height {
            self.max_height = height;
        }
    }
}
#[derive(Debug)]
struct Tree {
    height: u32,
    visible: bool,
}
pub trait CheckHieght {
    fn check_height(&mut self, height: usize);
}
impl Grid {
    pub fn check_outter(&mut self, indexes: (usize, usize)) -> bool {
        let outter_list = [0, self.max_length - 1];
        if outter_list.contains(&indexes.0) || outter_list.contains(&indexes.1) {
            self.total_visible += 1;
            true
        } else {
            false
        }
    }

    fn check_scenic_score(&mut self) -> usize {
        let mut max_score = 0;
        for (row_index, row) in self.rows.iter().enumerate() {
            for (column_index, tree) in row.trees.iter().enumerate() {
                let (left, right) = &self.rows[row_index].trees.split_at(column_index);
                let (up, down) = &self.columns[column_index].trees.split_at(row_index);
                let left_view = left
                    .iter()
                    .rev()
                    .take_until(|x| tree.height <= x.height)
                    .count();
                let right_view = right
                    .iter()
                    .skip(1)
                    .take_until(|x| tree.height <= x.height)
                    .count();
                let down_view = down
                    .iter()
                    .skip(1)
                    .take_until(|x| tree.height <= x.height)
                    .count();
                let up_view = up
                    .iter()
                    .rev()
                    .take_until(|x| tree.height <= x.height)
                    .count();
                max_score = max_score.max(left_view * up_view * down_view * right_view);
            }
        }
        max_score
    }

    fn check_visibility(&mut self) {
        for (row_index, row) in self.rows.iter().enumerate() {
            for (column_index, tree) in row.trees.iter().enumerate() {
                if tree.visible {
                    continue;
                }
                let (left, right) = &self.rows[row_index].trees.split_at(column_index);
                let (up, down) = &self.columns[column_index].trees.split_at(row_index);
                let left_greater =
                    left.iter().filter(|x| tree.height > x.height).count() == left.len();
                let right_greater = right
                    .iter()
                    .skip(1)
                    .filter(|x| tree.height > x.height)
                    .count()
                    == right.len() - 1;
                let down_greater = down
                    .iter()
                    .skip(1)
                    .filter(|x| tree.height > x.height)
                    .count()
                    == down.len() - 1;
                let up_greater = up.iter().filter(|x| tree.height > x.height).count() == up.len();
                let results = vec![left_greater, right_greater, down_greater, up_greater];
                if results.iter().any(|b| b == &true) {
                    self.total_visible += 1;
                }
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("inputs/day8.txt").unwrap();
    let mut grid = build_grid(input);
    dbg!(grid.check_visibility());
    dbg!(grid.total_visible);
    dbg!(grid.check_scenic_score());
}
fn build_grid(input:String)->Grid{
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let max_length = lines.len();
    let mut grid = Grid {
        rows: Vec::new(),
        columns: Vec::new(),
        max_length,
        total_visible: 0,
    };
    (0..max_length).into_iter().for_each(|_| {
        grid.columns.push(Column {
            trees: Vec::new(),
            max_height: 0,
        });
        grid.rows.push(Row {
            trees: Vec::new(),
            max_height: 0,
        })
    });
    lines.iter().enumerate().for_each(|(row_index, line)| {
        line.chars()
            .into_iter()
            .enumerate()
            .for_each(|(column_index, c)| {
                let visible = grid.check_outter((row_index, column_index));
                let height = c.to_digit(10).unwrap();
                grid.rows[row_index].check_height(height as usize);
                grid.columns[column_index].check_height(height as usize);
                let tree = Rc::new(Tree { height, visible });
                grid.rows[row_index].trees.push(Rc::clone(&tree));
                grid.columns[column_index].trees.push(Rc::clone(&tree));
            });
    });
    grid
}
#[test]
fn part_one_test(){
    let input = EXAMPLE_INPUT.to_owned();
    let mut grid = build_grid(input);
    grid.check_visibility();
    assert_eq!(21,grid.total_visible)
}
#[test]
fn part_two_test(){
    let input = EXAMPLE_INPUT.to_owned();
    let mut grid = build_grid(input);
    assert_eq!(grid.check_scenic_score(),8)
}
#[test]
fn actual_input(){
    let input = std::fs::read_to_string("inputs/day8.txt").unwrap();
    let mut grid = build_grid(input);
    assert_eq!(grid.check_scenic_score(),157320);
    grid.check_visibility();
    assert_eq!(1832,grid.total_visible);
}
