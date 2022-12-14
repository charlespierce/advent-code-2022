use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn moves(self) -> Vec<Point> {
        vec![
            Point::new(self.x, self.y + 1),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x + 1, self.y + 1),
        ]
    }
}

struct Rock {
    blocked: HashSet<Point>,
    max_y: isize,
}

enum SandResult {
    Stopped,
    FellIntoAbyss,
    Clogged,
}

impl Rock {
    fn sand_unit(&mut self) -> SandResult {
        let mut position = Point::new(500, 0);

        'outer: while position.y <= self.max_y {
            for next in position.moves() {
                if !self.blocked.contains(&next) {
                    position = next;
                    continue 'outer;
                }
            }

            self.blocked.insert(position);
            return SandResult::Stopped;
        }

        SandResult::FellIntoAbyss
    }

    fn sand_unit_floor(&mut self) -> SandResult {
        let mut position = Point::new(500, 0);

        'outer: loop {
            if position.y == self.max_y + 1 {
                self.blocked.insert(position);
                return SandResult::Stopped;
            }

            for next in position.moves() {
                if !self.blocked.contains(&next) {
                    position = next;
                    continue 'outer;
                }
            }

            if position.y == 0 {
                return SandResult::Clogged;
            } else {
                self.blocked.insert(position);
                return SandResult::Stopped;
            }
        }
    }
}

fn parse_path(input: &str) -> (isize, Vec<Point>) {
    let points = input
        .split(" -> ")
        .map(|point_str| {
            let (x_str, y_str) = point_str.split_once(',').unwrap();
            Point {
                x: x_str.parse().unwrap(),
                y: y_str.parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let mut path = Vec::new();
    let mut max_y = 0;

    for index in 1..points.len() {
        let start = points[index - 1];
        let end = points[index];

        if start.x == end.x {
            // Only change in y
            let y_range = if start.y < end.y {
                start.y..=end.y
            } else {
                end.y..=start.y
            };

            max_y = max_y.max(*y_range.end());
            for y in y_range {
                path.push(Point::new(start.x, y));
            }
        } else {
            max_y = max_y.max(start.y);
            // Only change in x
            let x_range = if start.x < end.x {
                start.x..=end.x
            } else {
                end.x..=start.x
            };

            for x in x_range {
                path.push(Point::new(x, start.y));
            }
        }
    }

    (max_y, path)
}

fn parse_rock(input: &str) -> Rock {
    let mut blocked = HashSet::new();
    let mut max_y = 0;

    for (y, path) in input.lines().map(parse_path) {
        max_y = max_y.max(y);
        blocked.extend(path);
    }

    Rock { blocked, max_y }
}

#[aoc(day14, part1)]
fn solve_part1(input: &str) -> usize {
    let mut rock = parse_rock(input);
    let mut count = 0;

    while matches!(rock.sand_unit(), SandResult::Stopped) {
        count += 1;
    }

    count
}

#[aoc(day14, part2)]
fn solve_part2(input: &str) -> usize {
    let mut rock = parse_rock(input);
    let mut count = 0;

    while matches!(rock.sand_unit_floor(), SandResult::Stopped) {
        count += 1;
    }

    // Need to include the final bit of sand that got stuck
    count + 1
}
