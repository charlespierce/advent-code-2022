use aoc_runner_derive::aoc;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::iter::Cycle;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(Clone, Copy)]
enum Jet {
    Left,
    Right,
}

impl Jet {
    fn from_char(chr: char) -> Self {
        match chr {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("Invalid jet direction"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum RockKind {
    Horizontal,
    Plus,
    L,
    Vertical,
    Box,
}

#[derive(Clone, Copy, Debug)]
struct Rock {
    kind: RockKind,
    origin: Point,
}

impl Rock {
    fn new(kind: RockKind, origin: Point) -> Self {
        Rock { kind, origin }
    }

    fn move_left(self) -> Option<Self> {
        if self.origin.x > 0 {
            let origin = Point::new(self.origin.x - 1, self.origin.y);
            Some(Rock::new(self.kind, origin))
        } else {
            None
        }
    }

    fn move_right(self) -> Option<Self> {
        let max_x = match self.kind {
            RockKind::Horizontal => 3,
            RockKind::Plus => 4,
            RockKind::L => 4,
            RockKind::Vertical => 6,
            RockKind::Box => 5,
        };

        if self.origin.x < max_x {
            let origin = Point::new(self.origin.x + 1, self.origin.y);
            Some(Rock::new(self.kind, origin))
        } else {
            None
        }
    }

    fn move_down(self) -> Option<Self> {
        if self.origin.y > 1 {
            let origin = Point::new(self.origin.x, self.origin.y - 1);
            Some(Rock::new(self.kind, origin))
        } else {
            None
        }
    }

    fn points(self) -> impl Iterator<Item = Point> {
        let x = self.origin.x;
        let y = self.origin.y;
        match self.kind {
            RockKind::Horizontal => vec![
                Point::new(x, y),
                Point::new(x + 1, y),
                Point::new(x + 2, y),
                Point::new(x + 3, y),
            ],
            RockKind::Plus => vec![
                Point::new(x + 1, y),
                Point::new(x, y + 1),
                Point::new(x + 1, y + 1),
                Point::new(x + 1, y + 2),
                Point::new(x + 2, y + 1),
            ],
            RockKind::L => vec![
                Point::new(x, y),
                Point::new(x + 1, y),
                Point::new(x + 2, y),
                Point::new(x + 2, y + 1),
                Point::new(x + 2, y + 2),
            ],
            RockKind::Vertical => vec![
                Point::new(x, y),
                Point::new(x, y + 1),
                Point::new(x, y + 2),
                Point::new(x, y + 3),
            ],
            RockKind::Box => vec![
                Point::new(x, y),
                Point::new(x, y + 1),
                Point::new(x + 1, y),
                Point::new(x + 1, y + 1),
            ],
        }
        .into_iter()
    }
}

struct Cave<J> {
    jets: J,
    blocked: HashSet<Point>,
    max_y: usize,
}

impl<I> Cave<Cycle<I>>
where
    I: Iterator<Item = Jet> + Clone,
{
    fn new(jets: I) -> Self {
        Cave {
            jets: jets.cycle(),
            blocked: HashSet::new(),
            max_y: 0,
        }
    }

    fn is_covered(&self, rock: Rock) -> bool {
        rock.points().any(|point| self.blocked.contains(&point))
    }

    fn hash_top(&self) -> String {
        let mut hash = String::with_capacity(280);
        for y in (self.max_y - 39)..=self.max_y {
            for x in 0..7 {
                if self.blocked.contains(&Point::new(x, y)) {
                    hash.push('#');
                } else {
                    hash.push('.');
                }
            }
        }

        hash
    }

    fn moved_rock(&self, maybe_rock: Option<Rock>) -> Option<Rock> {
        maybe_rock.and_then(|rock| (!self.is_covered(rock)).then_some(rock))
    }

    fn drop_rock(&mut self, kind: RockKind) {
        let start = Point::new(2, self.max_y + 4);
        let mut rock = Rock::new(kind, start);

        loop {
            // Do horizontal movement
            let updated_rock = match self.jets.next().unwrap() {
                Jet::Left => rock.move_left(),
                Jet::Right => rock.move_right(),
            };

            if let Some(moved) = self.moved_rock(updated_rock) {
                rock = moved;
            }

            // Do vertical movement
            match self.moved_rock(rock.move_down()) {
                Some(moved) => {
                    rock = moved;
                }
                None => {
                    // We've found a spot to stop
                    for point in rock.points() {
                        self.blocked.insert(point);
                        self.max_y = self.max_y.max(point.y);
                    }
                    break;
                }
            }
        }
    }
}

fn parse_jets(input: &str) -> Vec<Jet> {
    input.chars().map(Jet::from_char).collect()
}

fn rocks() -> impl Iterator<Item = RockKind> {
    [
        RockKind::Horizontal,
        RockKind::Plus,
        RockKind::L,
        RockKind::Vertical,
        RockKind::Box,
    ]
    .into_iter()
    .cycle()
}

#[aoc(day17, part1)]
fn solve_part1(input: &str) -> usize {
    let jets = parse_jets(input);
    let mut cave = Cave::new(jets.into_iter());

    for rock in rocks().take(2022) {
        cave.drop_rock(rock);
    }

    cave.max_y
}

#[aoc(day17, part2)]
fn solve_part2(input: &str) -> usize {
    let jets = parse_jets(input);
    let sweep = jets.len() * 5;
    let mut cyclic_part = 0;
    let mut last: Option<usize> = None;
    let mut cave = Cave::new(jets.into_iter());
    let mut heights: HashMap<String, (usize, usize)> = HashMap::new();

    for (index, rock) in rocks().enumerate() {
        cave.drop_rock(rock);
        if (index + 1) % sweep == 0 && last.is_none() {
            let hash = cave.hash_top();

            match heights.entry(hash) {
                Entry::Vacant(vacant) => {
                    vacant.insert((cave.max_y, index + 1));
                }
                Entry::Occupied(occupied) => {
                    let (old_height, old_index) = *occupied.get();

                    let period = index + 1 - old_index;
                    let height_diff = cave.max_y - old_height;
                    let remaining = 1_000_000_000_000 - index - 1;
                    cyclic_part = (remaining / period) * height_diff;
                    last = Some((remaining % period) + index);
                }
            }
        }

        if Some(index) == last {
            break;
        }
    }

    cyclic_part + cave.max_y
}
