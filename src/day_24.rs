use crate::dijkstra::{Dijkstra, Value};
use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn new(row: isize, col: isize) -> Self {
        Point { row, col }
    }

    fn shift(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Point::new(self.row - 1, self.col),
            Direction::Down => Point::new(self.row + 1, self.col),
            Direction::Left => Point::new(self.row, self.col - 1),
            Direction::Right => Point::new(self.row, self.col + 1),
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    position: Point,
    time: usize,
}

impl Value for State {
    type Id = State;

    fn id(&self) -> Self::Id {
        *self
    }
}

#[derive(Clone, Copy)]
struct Blizzard {
    start: Point,
    direction: Direction,
}

struct Snowfield {
    blizzards: Vec<Blizzard>,
    start_col: isize,
    end_col: isize,
    max_row: isize,
    max_col: isize,
}

struct SnowfieldSnapshot {
    blizzards: HashSet<Point>,
    start_col: isize,
    end_col: isize,
    max_row: isize,
    max_col: isize,
}

impl Snowfield {
    fn shift_position(&self, start: Point, direction: Direction, time: usize) -> Point {
        let updated = match direction {
            Direction::Up => Point::new(start.row - time as isize, start.col),
            Direction::Down => Point::new(start.row + time as isize, start.col),
            Direction::Left => Point::new(start.row, start.col - time as isize),
            Direction::Right => Point::new(start.row, start.col + time as isize),
        };

        let wrapped_row = (updated.row - 1).rem_euclid(self.max_row) + 1;
        let wrapped_col = (updated.col - 1).rem_euclid(self.max_col) + 1;

        Point::new(wrapped_row, wrapped_col)
    }

    fn snapshot(&self, time: usize) -> SnowfieldSnapshot {
        let blizzards = self
            .blizzards
            .iter()
            .map(|blizz| self.shift_position(blizz.start, blizz.direction, time))
            .collect();

        SnowfieldSnapshot {
            blizzards,
            start_col: self.start_col,
            end_col: self.end_col,
            max_row: self.max_row,
            max_col: self.max_col,
        }
    }
}

impl SnowfieldSnapshot {
    fn is_open(&self, point: Point) -> bool {
        (point.row >= 1
            && point.row <= self.max_row
            && point.col >= 1
            && point.col <= self.max_col
            && !self.blizzards.contains(&point))
            || (point.row == 0 && point.col == self.start_col)
            || (point.row == self.max_row + 1 && point.col == self.end_col)
    }
}

fn parse_snowfield(input: &str) -> Snowfield {
    let mut lines = input.lines();

    let start_col = lines
        .next()
        .unwrap()
        .chars()
        .position(|chr| chr == '.')
        .unwrap() as isize;
    let end_col = lines
        .next_back()
        .unwrap()
        .chars()
        .position(|chr| chr == '.')
        .unwrap() as isize;

    let mut max_row = 0;
    let mut max_col = 0;
    let mut blizzards = Vec::new();

    for (row_index, line) in lines.enumerate() {
        let row = row_index as isize + 1;
        max_row = max_row.max(row);

        for (col, chr) in line.chars().enumerate() {
            let start = Point::new(row, col as isize);
            let direction = match chr {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                '#' => {
                    max_col = max_col.max(col as isize - 1);
                    continue;
                }
                _ => continue,
            };

            blizzards.push(Blizzard { start, direction });
        }
    }
    Snowfield {
        blizzards,
        start_col,
        end_col,
        max_row,
        max_col,
    }
}

#[aoc(day24, part1)]
fn solve_part1(input: &str) -> usize {
    let snowfield = parse_snowfield(input);
    let start_pos = Point::new(0, snowfield.start_col);
    let start = State {
        position: start_pos,
        time: 0,
    };

    let (end, _) = Dijkstra::new(
        start,
        |state| state.position == Point::new(snowfield.max_row + 1, snowfield.end_col),
        |state| {
            let next_time = state.time + 1;
            let snapshot = snowfield.snapshot(next_time);

            [
                state.position.shift(Direction::Down),
                state.position.shift(Direction::Right),
                state.position,
                state.position.shift(Direction::Up),
                state.position.shift(Direction::Left),
            ]
            .into_iter()
            .filter_map(move |position| {
                if snapshot.is_open(position) {
                    Some((
                        State {
                            position,
                            time: next_time,
                        },
                        1,
                    ))
                } else {
                    None
                }
            })
        },
    )
    .next()
    .unwrap();

    end.time
}

#[aoc(day24, part2)]
fn solve_part2(input: &str) -> usize {
    let snowfield = parse_snowfield(input);
    let start_pos = Point::new(0, snowfield.start_col);
    let start = State {
        position: start_pos,
        time: 0,
    };

    let (first_end, _) = Dijkstra::new(
        start,
        |state| state.position == Point::new(snowfield.max_row + 1, snowfield.end_col),
        |state| {
            let next_time = state.time + 1;
            let snapshot = snowfield.snapshot(next_time);

            [
                state.position.shift(Direction::Down),
                state.position.shift(Direction::Right),
                state.position,
                state.position.shift(Direction::Up),
                state.position.shift(Direction::Left),
            ]
            .into_iter()
            .filter_map(move |position| {
                if snapshot.is_open(position) {
                    Some((
                        State {
                            position,
                            time: next_time,
                        },
                        1,
                    ))
                } else {
                    None
                }
            })
        },
    )
    .next()
    .unwrap();

    let (second_start, _) = Dijkstra::new(
        first_end,
        |state| state.position == Point::new(0, snowfield.start_col),
        |state| {
            let next_time = state.time + 1;
            let snapshot = snowfield.snapshot(next_time);

            [
                state.position.shift(Direction::Down),
                state.position.shift(Direction::Right),
                state.position,
                state.position.shift(Direction::Up),
                state.position.shift(Direction::Left),
            ]
            .into_iter()
            .filter_map(move |position| {
                if snapshot.is_open(position) {
                    Some((
                        State {
                            position,
                            time: next_time,
                        },
                        1,
                    ))
                } else {
                    None
                }
            })
        },
    )
    .next()
    .unwrap();

    let (final_end, _) = Dijkstra::new(
        second_start,
        |state| state.position == Point::new(snowfield.max_row + 1, snowfield.end_col),
        |state| {
            let next_time = state.time + 1;
            let snapshot = snowfield.snapshot(next_time);

            [
                state.position.shift(Direction::Down),
                state.position.shift(Direction::Right),
                state.position,
                state.position.shift(Direction::Up),
                state.position.shift(Direction::Left),
            ]
            .into_iter()
            .filter_map(move |position| {
                if snapshot.is_open(position) {
                    Some((
                        State {
                            position,
                            time: next_time,
                        },
                        1,
                    ))
                } else {
                    None
                }
            })
        },
    )
    .next()
    .unwrap();

    final_end.time
}
