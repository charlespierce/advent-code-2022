use aoc_runner_derive::aoc;
use nom::{bytes::complete::tag, character::complete::i64, sequence::tuple, IResult};
use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Point { x, y, z }
    }

    fn adjacents(self) -> impl Iterator<Item = Point> {
        [
            Point::new(self.x - 1, self.y, self.z),
            Point::new(self.x + 1, self.y, self.z),
            Point::new(self.x, self.y - 1, self.z),
            Point::new(self.x, self.y + 1, self.z),
            Point::new(self.x, self.y, self.z - 1),
            Point::new(self.x, self.y, self.z + 1),
        ]
        .into_iter()
    }
}

struct LavaDroplet {
    min: Point,
    max: Point,
    cubes: HashSet<Point>,
}

impl LavaDroplet {
    fn surface_area(&self) -> usize {
        let mut area = 0;
        for point in self.cubes.iter() {
            for neighbor in point.adjacents() {
                if !self.cubes.contains(&neighbor) {
                    area += 1;
                }
            }
        }

        area
    }

    fn outside_blocks(&self) -> HashSet<Point> {
        let mut outside = HashSet::new();
        let mut queue = VecDeque::new();
        let (min_x, min_y, min_z) = (self.min.x - 1, self.min.y - 1, self.min.z - 1);
        let (max_x, max_y, max_z) = (self.max.x + 1, self.max.y + 1, self.max.z + 1);
        queue.push_back(Point::new(min_x, min_y, min_z));

        while let Some(point) = queue.pop_front() {
            if outside.contains(&point) {
                continue;
            }

            outside.insert(point);

            for neighbor in point.adjacents() {
                if !self.cubes.contains(&neighbor)
                    && neighbor.x >= min_x
                    && neighbor.y >= min_y
                    && neighbor.z >= min_z
                    && neighbor.x <= max_x
                    && neighbor.y <= max_y
                    && neighbor.z <= max_z
                {
                    queue.push_back(neighbor);
                }
            }
        }

        outside
    }

    fn exterior_surface_area(&self) -> usize {
        let mut area = 0;
        let outside = self.outside_blocks();

        for point in self.cubes.iter() {
            for neighbor in point.adjacents() {
                if outside.contains(&neighbor) {
                    area += 1;
                }
            }
        }

        area
    }
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    let (rest, value) = i64(input)?;
    Ok((rest, value as isize))
}

fn parse_point(input: &str) -> Point {
    let (_, (x, _, y, _, z)) =
        tuple((parse_isize, tag(","), parse_isize, tag(","), parse_isize))(input).unwrap();

    Point { x, y, z }
}

fn parse_droplet(input: &str) -> LavaDroplet {
    let mut min = Point::new(100, 100, 100);
    let mut max = Point::new(-100, -100, -100);
    let mut cubes = HashSet::new();

    for cube in input.lines().map(parse_point) {
        min.x = min.x.min(cube.x);
        min.y = min.y.min(cube.y);
        min.z = min.z.min(cube.z);
        max.x = max.x.max(cube.x);
        max.y = max.y.max(cube.y);
        max.z = max.z.max(cube.z);

        cubes.insert(cube);
    }

    LavaDroplet { min, max, cubes }
}

#[aoc(day18, part1)]
fn solve_part1(input: &str) -> usize {
    let droplet = parse_droplet(input);
    droplet.surface_area()
}

#[aoc(day18, part2)]
fn solve_part2(input: &str) -> usize {
    let droplet = parse_droplet(input);
    droplet.exterior_surface_area()
}
