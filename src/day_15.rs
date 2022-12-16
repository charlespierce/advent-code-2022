use aoc_runner_derive::aoc;
use nom::{bytes::complete::tag, character::complete::i64, sequence::tuple, IResult};
use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn distance(self, other: Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

struct Sensor {
    position: Point,
    closest_beacon: Point,
}

impl Sensor {
    fn distance_to_beacon(&self) -> isize {
        self.position.distance(self.closest_beacon)
    }

    fn precluded_x_range(&self, y: isize) -> Option<RangeInclusive<isize>> {
        let precluded_distance = self.distance_to_beacon();
        let distance_to_y = (self.position.y - y).abs();

        if precluded_distance > distance_to_y {
            let x_extent = precluded_distance - distance_to_y;
            Some((self.position.x - x_extent)..=(self.position.x + x_extent))
        } else {
            None
        }
    }
}

struct RangeCollection {
    ranges: Vec<RangeInclusive<isize>>,
}

impl RangeCollection {
    fn new() -> Self {
        RangeCollection { ranges: Vec::new() }
    }

    fn add_range(&mut self, range: RangeInclusive<isize>) {
        let mut current = Some(range);
        let existing = std::mem::take(&mut self.ranges);

        for range in existing {
            match current.take() {
                None => self.ranges.push(range),
                Some(curr) => {
                    if range.end() < curr.start() {
                        self.ranges.push(range);
                        current = Some(curr);
                    } else if *range.start() == curr.end() + 1 {
                        current = Some(*curr.start()..=*range.end());
                    } else if *range.start() > curr.end() + 1 {
                        self.ranges.push(curr);
                        self.ranges.push(range);
                    } else {
                        let start = curr.start().min(range.start());
                        let end = curr.end().max(range.end());
                        current = Some(*start..=*end);
                    }
                }
            }
        }

        if let Some(curr) = current {
            self.ranges.push(curr);
        }
    }

    fn size(&self) -> isize {
        self.ranges
            .iter()
            .map(|range| *range.end() - *range.start() + 1)
            .sum()
    }

    fn gap(&self, start: isize, end: isize) -> Option<isize> {
        // This is a bit hacky by assuming the values will be dense with at most a single gap
        for range in &self.ranges {
            if *range.end() < start || *range.start() > end {
                continue;
            }

            if *range.start() > start {
                return Some(*range.start() - 1);
            }

            if *range.end() < end {
                return Some(*range.end() + 1);
            }

            break;
        }

        None
    }
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    let (rest, value) = i64(input)?;
    Ok((rest, value as isize))
}

fn parse_sensor(input: &str) -> Sensor {
    let (_, (_, x, _, y, _, beacon_x, _, beacon_y)) = tuple((
        tag("Sensor at x="),
        parse_isize,
        tag(", y="),
        parse_isize,
        tag(": closest beacon is at x="),
        parse_isize,
        tag(", y="),
        parse_isize,
    ))(input)
    .unwrap();

    Sensor {
        position: Point { x, y },
        closest_beacon: Point {
            x: beacon_x,
            y: beacon_y,
        },
    }
}

fn parse_sensors(input: &str) -> Vec<Sensor> {
    input.lines().map(parse_sensor).collect()
}

#[aoc(day15, part1)]
fn solve_part1(input: &str) -> usize {
    const TARGET_Y: isize = 2_000_000;
    let sensors = parse_sensors(input);
    let mut precluded_ranges = RangeCollection::new();
    let mut beacons = HashSet::new();

    for sensor in sensors {
        if sensor.closest_beacon.y == TARGET_Y {
            beacons.insert(sensor.closest_beacon.x);
        }

        if let Some(range) = sensor.precluded_x_range(TARGET_Y) {
            precluded_ranges.add_range(range);
        }
    }

    precluded_ranges.size() as usize - beacons.len()
}

#[aoc(day15, part2)]
fn solve_part2(input: &str) -> isize {
    let sensors = parse_sensors(input);

    for y in 0..=4_000_000 {
        let mut ranges = RangeCollection::new();

        for sensor in &sensors {
            if let Some(range) = sensor.precluded_x_range(y) {
                ranges.add_range(range);
            }
        }

        if let Some(x) = ranges.gap(0, 4_000_000) {
            return x * 4_000_000 + y;
        }
    }

    0
}
