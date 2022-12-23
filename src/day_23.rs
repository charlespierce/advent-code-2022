use aoc_runner_derive::aoc;
use std::array::IntoIter;
use std::collections::{HashMap, HashSet};
use std::iter::Cycle;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn neighbors(self, direction: Direction) -> [Self; 3] {
        match direction {
            Direction::North => [
                Point::new(self.x - 1, self.y - 1),
                Point::new(self.x, self.y - 1),
                Point::new(self.x + 1, self.y - 1),
            ],
            Direction::South => [
                Point::new(self.x - 1, self.y + 1),
                Point::new(self.x, self.y + 1),
                Point::new(self.x + 1, self.y + 1),
            ],
            Direction::East => [
                Point::new(self.x + 1, self.y - 1),
                Point::new(self.x + 1, self.y),
                Point::new(self.x + 1, self.y + 1),
            ],
            Direction::West => [
                Point::new(self.x - 1, self.y - 1),
                Point::new(self.x - 1, self.y),
                Point::new(self.x - 1, self.y + 1),
            ],
        }
    }

    fn proposed_move(self, direction: Direction) -> Self {
        match direction {
            Direction::North => Point::new(self.x, self.y - 1),
            Direction::South => Point::new(self.x, self.y + 1),
            Direction::East => Point::new(self.x + 1, self.y),
            Direction::West => Point::new(self.x - 1, self.y),
        }
    }

    fn max(self, other: Self) -> Self {
        Point {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    fn min(self, other: Self) -> Self {
        Point {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn directions() -> Cycle<IntoIter<Direction, 4>> {
    [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]
    .into_iter()
    .cycle()
}

struct Forest {
    elves: HashSet<Point>,
    directions: Cycle<IntoIter<Direction, 4>>,
}

impl Forest {
    fn is_alone(&self, elf: Point) -> bool {
        [
            Point::new(elf.x - 1, elf.y - 1),
            Point::new(elf.x, elf.y - 1),
            Point::new(elf.x + 1, elf.y - 1),
            Point::new(elf.x + 1, elf.y),
            Point::new(elf.x + 1, elf.y + 1),
            Point::new(elf.x, elf.y + 1),
            Point::new(elf.x - 1, elf.y + 1),
            Point::new(elf.x - 1, elf.y),
        ]
        .into_iter()
        .all(|point| !self.elves.contains(&point))
    }

    fn is_safe_direction(&self, elf: Point, direction: Direction) -> bool {
        elf.neighbors(direction)
            .into_iter()
            .all(|point| !self.elves.contains(&point))
    }

    fn propose_moves(&mut self) -> HashMap<Point, Vec<Point>> {
        let dirs = [
            self.directions.next().unwrap(),
            self.directions.next().unwrap(),
            self.directions.next().unwrap(),
            self.directions.next().unwrap(),
        ];
        // Rotate the directions for the next call
        self.directions.next();

        let mut result: HashMap<Point, Vec<Point>> = HashMap::new();

        for elf in self.elves.iter().copied() {
            if self.is_alone(elf) {
                continue;
            }

            for dir in dirs {
                if self.is_safe_direction(elf, dir) {
                    result.entry(elf.proposed_move(dir)).or_default().push(elf);
                    break;
                }
            }
        }

        result
    }

    fn step(&mut self) -> bool {
        let mut did_move = false;
        for (target, elves) in self.propose_moves() {
            if elves.len() == 1 {
                did_move = true;
                self.elves.remove(&elves[0]);
                self.elves.insert(target);
            }
        }
        did_move
    }

    fn free_space(&self) -> usize {
        let min_point = Point::new(isize::MAX, isize::MAX);
        let max_point = Point::new(isize::MIN, isize::MIN);

        let (min, max) = self
            .elves
            .iter()
            .fold((min_point, max_point), |(min, max), &point| {
                (min.min(point), max.max(point))
            });

        (max.x - min.x + 1) as usize * (max.y - min.y + 1) as usize - self.elves.len()
    }
}

fn parse_forest(input: &str) -> Forest {
    let elves: HashSet<_> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, chr)| {
                if chr == '#' {
                    Some(Point::new(x as isize, y as isize))
                } else {
                    None
                }
            })
        })
        .collect();

    Forest {
        elves,
        directions: directions(),
    }
}

#[aoc(day23, part1)]
fn solve_part1(input: &str) -> usize {
    let mut forest = parse_forest(input);

    for _ in 0..10 {
        forest.step();
    }

    forest.free_space()
}

#[aoc(day23, part2)]
fn solve_part2(input: &str) -> usize {
    let mut forest = parse_forest(input);
    let mut round = 1;

    while forest.step() {
        round += 1;
    }

    round
}
