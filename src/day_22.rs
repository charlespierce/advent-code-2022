use aoc_runner_derive::aoc;
use nom::{branch::alt, bytes::complete::tag, character::complete::u64, multi::many1, IResult};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn next(self, direction: Direction) -> Self {
        let (row, col) = match direction {
            Direction::Up => (self.row - 1, self.col),
            Direction::Down => (self.row + 1, self.col),
            Direction::Left => (self.row, self.col - 1),
            Direction::Right => (self.row, self.col + 1),
        };

        Point { row, col }
    }
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Point { row, col }
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Open,
    Solid,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn value(self) -> usize {
        match self {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        }
    }
}

#[derive(Clone, Copy)]
enum Move {
    Forward(usize),
    Left,
    Right,
}

struct Board {
    tiles: HashMap<Point, Tile>,
    max_row: usize,
    max_col: usize,
}

impl Board {
    fn next_point(&self, position: Point, direction: Direction) -> Point {
        let next = position.next(direction);

        if self.tiles.contains_key(&next) {
            return next;
        }

        let (wrap_row, wrap_col) = match direction {
            Direction::Up => (self.max_row, position.col),
            Direction::Down => (1, position.col),
            Direction::Left => (position.row, self.max_col),
            Direction::Right => (position.row, 1),
        };

        let mut wrap = Point::new(wrap_row, wrap_col);

        while !self.tiles.contains_key(&wrap) {
            wrap = wrap.next(direction);
        }

        wrap
    }

    fn next_point_cube(&self, position: Point, direction: Direction) -> (Point, Direction) {
        let next = position.next(direction);

        if self.tiles.contains_key(&next) {
            return (next, direction);
        }

        match (direction, position.row, position.col) {
            (Direction::Up, 1, col) if (51..=100).contains(&col) => {
                (Point::new(col + 100, 1), Direction::Right)
            }
            (Direction::Up, 1, col) if (101..=150).contains(&col) => {
                (Point::new(200, col - 100), Direction::Up)
            }
            (Direction::Up, 101, col) if (1..=50).contains(&col) => {
                (Point::new(col + 50, 51), Direction::Right)
            }
            (Direction::Down, 200, col) if (1..=50).contains(&col) => {
                (Point::new(1, col + 100), Direction::Down)
            }
            (Direction::Down, 150, col) if (51..=100).contains(&col) => {
                (Point::new(col + 100, 50), Direction::Left)
            }
            (Direction::Down, 50, col) if (101..=150).contains(&col) => {
                (Point::new(col - 50, 100), Direction::Left)
            }
            (Direction::Left, row, 51) if (1..=50).contains(&row) => {
                (Point::new(151 - row, 1), Direction::Right)
            }
            (Direction::Left, row, 51) if (51..=100).contains(&row) => {
                (Point::new(101, row - 50), Direction::Down)
            }
            (Direction::Left, row, 1) if (101..=150).contains(&row) => {
                (Point::new(151 - row, 51), Direction::Right)
            }
            (Direction::Left, row, 1) if (151..=200).contains(&row) => {
                (Point::new(1, row - 100), Direction::Down)
            }
            (Direction::Right, row, 150) if (1..=50).contains(&row) => {
                (Point::new(151 - row, 100), Direction::Left)
            }
            (Direction::Right, row, 100) if (51..=100).contains(&row) => {
                (Point::new(50, row + 50), Direction::Up)
            }
            (Direction::Right, row, 100) if (101..=150).contains(&row) => {
                (Point::new(151 - row, 150), Direction::Left)
            }
            (Direction::Right, row, 50) if (151..=200).contains(&row) => {
                (Point::new(150, row - 100), Direction::Up)
            }
            _ => unreachable!(),
        }
    }

    fn next_tile(&self, position: Point, direction: Direction) -> Point {
        let next = self.next_point(position, direction);

        match self.tiles.get(&next) {
            Some(Tile::Open) => next,
            Some(Tile::Solid) => position,
            _ => unreachable!("Next point already checks for existence"),
        }
    }

    fn next_tile_cube(&self, position: Point, direction: Direction) -> (Point, Direction) {
        let (next_pos, next_dir) = self.next_point_cube(position, direction);

        match self.tiles.get(&next_pos) {
            Some(Tile::Open) => (next_pos, next_dir),
            Some(Tile::Solid) => (position, direction),
            _ => unreachable!("Next point already checks for existence"),
        }
    }

    fn move_forward(&self, position: Point, direction: Direction, distance: usize) -> Point {
        let mut current = position;

        for _ in 0..distance {
            current = self.next_tile(current, direction);
        }

        current
    }

    fn move_forward_cube(
        &self,
        position: Point,
        direction: Direction,
        distance: usize,
    ) -> (Point, Direction) {
        let mut curr_pos = position;
        let mut curr_dir = direction;

        for _ in 0..distance {
            (curr_pos, curr_dir) = self.next_tile_cube(curr_pos, curr_dir);
        }

        (curr_pos, curr_dir)
    }

    fn find_start(&self) -> Point {
        let mut current = Point::new(1, 1);

        loop {
            if let Some(Tile::Open) = self.tiles.get(&current) {
                break;
            }

            current = current.next(Direction::Right);
        }

        current
    }

    fn trace_path<I>(&self, moves: I) -> (Point, Direction)
    where
        I: IntoIterator<Item = Move>,
    {
        let mut position = self.find_start();
        let mut direction = Direction::Right;

        for mv in moves {
            match mv {
                Move::Forward(distance) => {
                    position = self.move_forward(position, direction, distance);
                }
                Move::Left => {
                    direction = direction.turn_left();
                }
                Move::Right => {
                    direction = direction.turn_right();
                }
            }
        }

        (position, direction)
    }

    fn trace_path_cube<I>(&self, moves: I) -> (Point, Direction)
    where
        I: IntoIterator<Item = Move>,
    {
        let mut position = self.find_start();
        let mut direction = Direction::Right;

        for mv in moves {
            match mv {
                Move::Forward(distance) => {
                    (position, direction) = self.move_forward_cube(position, direction, distance);
                }
                Move::Left => {
                    direction = direction.turn_left();
                }
                Move::Right => {
                    direction = direction.turn_right();
                }
            }
        }

        (position, direction)
    }
}

fn parse_board(input: &str) -> Board {
    let mut tiles = HashMap::new();
    let mut max_row = 0;
    let mut max_col = 0;

    for (row, line) in input.lines().enumerate() {
        for (col, tile_chr) in line.chars().enumerate() {
            let tile = match tile_chr {
                '.' => Tile::Open,
                '#' => Tile::Solid,
                _ => continue,
            };

            let position = Point::new(row + 1, col + 1);
            tiles.insert(position, tile);

            max_row = max_row.max(position.row);
            max_col = max_col.max(position.col);
        }
    }

    Board {
        tiles,
        max_row,
        max_col,
    }
}

fn parse_forward(input: &str) -> IResult<&str, Move> {
    let (rest, value) = u64(input)?;
    Ok((rest, Move::Forward(value as usize)))
}

fn parse_left(input: &str) -> IResult<&str, Move> {
    let (rest, _) = tag("L")(input)?;
    Ok((rest, Move::Left))
}

fn parse_right(input: &str) -> IResult<&str, Move> {
    let (rest, _) = tag("R")(input)?;
    Ok((rest, Move::Right))
}

fn parse_moves(input: &str) -> Vec<Move> {
    many1(alt((parse_forward, parse_left, parse_right)))(input)
        .unwrap()
        .1
}

#[aoc(day22, part1)]
fn solve_part1(input: &str) -> usize {
    let (board_input, moves_input) = input.split_once("\n\n").unwrap();
    let board = parse_board(board_input);
    let moves = parse_moves(moves_input);

    let (position, direction) = board.trace_path(moves);

    (1000 * position.row) + (4 * position.col) + direction.value()
}

#[aoc(day22, part2)]
fn solve_part2(input: &str) -> usize {
    let (board_input, moves_input) = input.split_once("\n\n").unwrap();
    let board = parse_board(board_input);
    let moves = parse_moves(moves_input);

    let (position, direction) = board.trace_path_cube(moves);

    (1000 * position.row) + (4 * position.col) + direction.value()
}
