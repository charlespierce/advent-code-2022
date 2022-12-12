use crate::dijkstra::{Dijkstra, Value};
use aoc_runner_derive::aoc;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

impl Value for Point {
    type Id = Point;

    fn id(&self) -> Self::Id {
        *self
    }
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

struct Map {
    heights: Vec<Vec<u32>>,
}

impl Map {
    fn get(&self, point: Point) -> u32 {
        self.heights[point.row][point.col]
    }

    fn neighbors(&self, point: Point) -> Vec<Point> {
        let mut options = Vec::new();
        if point.row > 0 {
            options.push(Point::new(point.row - 1, point.col));
        }

        if point.row < self.heights.len() - 1 {
            options.push(Point::new(point.row + 1, point.col));
        }

        if point.col > 0 {
            options.push(Point::new(point.row, point.col - 1));
        }

        if point.col < self.heights[0].len() - 1 {
            options.push(Point::new(point.row, point.col + 1));
        }

        options
    }
}

fn parse_map(input: &str) -> (Map, Point, Point) {
    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);

    let heights = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, chr)| {
                    if chr == 'S' {
                        start = Point::new(row, col);
                        0
                    } else if chr == 'E' {
                        end = Point::new(row, col);
                        25
                    } else {
                        chr.to_digit(36).unwrap() - 10
                    }
                })
                .collect()
        })
        .collect();

    (Map { heights }, start, end)
}

#[aoc(day12, part1)]
fn solve_part1(input: &str) -> usize {
    let (map, start, end) = parse_map(input);

    let (_, cost) = Dijkstra::new(
        start,
        |point| *point == end,
        |point| {
            let current = map.get(*point);

            map.neighbors(*point)
                .into_iter()
                .filter_map(|neighbor_point| {
                    let height = map.get(neighbor_point);
                    if height <= current + 1 {
                        Some((neighbor_point, 1))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        },
    )
    .next()
    .unwrap();

    cost
}

#[aoc(day12, part2)]
fn solve_part2(input: &str) -> usize {
    let (map, _, start) = parse_map(input);

    let (_, cost) = Dijkstra::new(
        start,
        |point| map.get(*point) == 0,
        |point| {
            let current = map.get(*point);

            map.neighbors(*point)
                .into_iter()
                .filter_map(|neighbor_point| {
                    let height = map.get(neighbor_point);
                    if height >= current - 1 {
                        Some((neighbor_point, 1))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        },
    )
    .next()
    .unwrap();

    cost
}
