use aoc_runner_derive::aoc;
use std::collections::HashSet;
use std::iter::repeat;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn shift(self, dir: Direction) -> Self {
        match dir {
            Direction::Up => Point::new(self.x, self.y + 1),
            Direction::Down => Point::new(self.x, self.y - 1),
            Direction::Left => Point::new(self.x - 1, self.y),
            Direction::Right => Point::new(self.x + 1, self.y),
        }
    }

    fn move_closer(self, target: Self) -> Self {
        let (diff_x, diff_y) = match (target.x - self.x, target.y - self.y) {
            (2, 2) | (2, 1) | (1, 2) => (1, 1),
            (2, -1) | (2, -2) | (1, -2) => (1, -1),
            (-2, 2) | (-2, 1) | (-1, 2) => (-1, 1),
            (-2, -1) | (-2, -2) | (-1, -2) => (-1, -1),
            (2, 0) => (1, 0),
            (-2, 0) => (-1, 0),
            (0, 2) => (0, 1),
            (0, -2) => (0, -1),
            _ => (0, 0),
        };

        Point::new(self.x + diff_x, self.y + diff_y)
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Rope {
    knots: Vec<Point>,
    visited: HashSet<Point>,
}

impl Rope {
    fn new(count: usize) -> Self {
        let origin = Point::new(0, 0);
        let knots = vec![origin; count];
        Self {
            knots,
            visited: HashSet::from([origin]),
        }
    }

    fn move_head(&mut self, direction: Direction) {
        self.knots[0] = self.knots[0].shift(direction);

        for index in 1..self.knots.len() {
            self.knots[index] = self.knots[index].move_closer(self.knots[index - 1]);
        }
        self.visited.insert(self.knots[self.knots.len() - 1]);
    }
}

fn parse_moves(input: &str) -> impl Iterator<Item = Direction> + '_ {
    input.lines().flat_map(|line| {
        let (dir_str, count_str) = line.split_once(' ').unwrap();
        let count: usize = count_str.parse().unwrap();
        let dir = match dir_str {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid input"),
        };

        repeat(dir).take(count)
    })
}

#[aoc(day9, part1)]
fn solve_part1(input: &str) -> usize {
    let mut rope = Rope::new(2);

    for dir in parse_moves(input) {
        rope.move_head(dir);
    }

    rope.visited.len()
}

#[aoc(day9, part2)]
fn solve_part2(input: &str) -> usize {
    let mut rope = Rope::new(10);

    for dir in parse_moves(input) {
        rope.move_head(dir);
    }

    rope.visited.len()
}
