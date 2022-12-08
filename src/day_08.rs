use aoc_runner_derive::aoc;

struct Tree {
    height: i32,
    visible: bool,
}

impl Tree {
    fn new(height: i32) -> Tree {
        Tree {
            height,
            visible: false,
        }
    }
}

struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl Forest {
    fn rows(&self) -> usize {
        self.trees.len()
    }

    fn columns(&self) -> usize {
        self.trees[0].len()
    }

    fn scan_row<S, F>(&mut self, row: usize, start: S, scanner: F)
    where
        F: FnMut(S, &mut Tree) -> S,
    {
        if let Some(tree_row) = self.trees.get_mut(row) {
            tree_row.iter_mut().fold(start, scanner);
        }
    }

    fn scan_row_reverse<S, F>(&mut self, row: usize, start: S, scanner: F)
    where
        F: FnMut(S, &mut Tree) -> S,
    {
        if let Some(tree_row) = self.trees.get_mut(row) {
            tree_row.iter_mut().rev().fold(start, scanner);
        }
    }

    fn scan_column<S, F>(&mut self, column: usize, start: S, mut scanner: F)
    where
        F: FnMut(S, &mut Tree) -> S,
    {
        if column < self.columns() {
            let mut accumulator = start;
            for tree_row in self.trees.iter_mut() {
                accumulator = scanner(accumulator, &mut tree_row[column]);
            }
        }
    }

    fn scan_column_reverse<S, F>(&mut self, column: usize, start: S, mut scanner: F)
    where
        F: FnMut(S, &mut Tree) -> S,
    {
        if column < self.columns() {
            let mut accumulator = start;
            for tree_row in self.trees.iter_mut().rev() {
                accumulator = scanner(accumulator, &mut tree_row[column]);
            }
        }
    }

    fn count_visible(&self) -> usize {
        self.trees
            .iter()
            .map(|row| row.iter().filter(|tree| tree.visible).count())
            .sum()
    }

    fn scenic_score(&self, row: usize, column: usize) -> usize {
        if row == 0 || column == 0 || row == self.rows() - 1 || column == self.columns() - 1 {
            return 0;
        }

        let house_height = self.trees[row][column].height;

        let mut left_score = 0;
        for tree in self.trees[row][..column].iter().rev() {
            left_score += 1;
            if tree.height >= house_height {
                break;
            }
        }

        let mut right_score = 0;
        for tree in self.trees[row][column + 1..].iter() {
            right_score += 1;
            if tree.height >= house_height {
                break;
            }
        }

        let mut up_score = 0;
        for row_index in (0..row).rev() {
            let tree = &self.trees[row_index][column];
            up_score += 1;
            if tree.height >= house_height {
                break;
            }
        }

        let mut down_score = 0;
        for row_index in row + 1..self.rows() {
            let tree = &self.trees[row_index][column];
            down_score += 1;
            if tree.height >= house_height {
                break;
            }
        }

        left_score * right_score * up_score * down_score
    }
}

fn parse_forest(input: &str) -> Forest {
    Forest {
        trees: input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|chr| Tree::new(chr.to_digit(10).unwrap() as i32))
                    .collect()
            })
            .collect(),
    }
}

fn check_visible(tallest: i32, tree: &mut Tree) -> i32 {
    if tree.height > tallest {
        tree.visible = true;
        tree.height
    } else {
        tallest
    }
}

#[aoc(day8, part1)]
fn solve_part1(input: &str) -> usize {
    let mut forest = parse_forest(input);

    for row in 0..forest.rows() {
        forest.scan_row(row, -1, check_visible);
        forest.scan_row_reverse(row, -1, check_visible);
    }

    for column in 0..forest.columns() {
        forest.scan_column(column, -1, check_visible);
        forest.scan_column_reverse(column, -1, check_visible);
    }

    forest.count_visible()
}

#[aoc(day8, part2)]
fn solve_part2(input: &str) -> usize {
    let forest = parse_forest(input);

    let mut highest_score = 0;

    for row in 1..forest.rows() - 1 {
        for column in 1..forest.columns() - 1 {
            let score = forest.scenic_score(row, column);
            highest_score = highest_score.max(score);
        }
    }

    highest_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenic_score() {
        let forest = parse_forest(
            "30373
25512
65332
33549
35390",
        );

        assert_eq!(forest.scenic_score(1, 2), 4);
        assert_eq!(forest.scenic_score(3, 2), 8);
    }
}
