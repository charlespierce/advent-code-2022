use aoc_runner_derive::aoc;
use std::ops::RangeInclusive;

fn parse_assignment(input: &str) -> RangeInclusive<usize> {
    let (lower, upper) = input.split_once('-').unwrap();
    lower.parse().unwrap()..=upper.parse().unwrap()
}

fn parse_pair(input: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let (first, second) = input.split_once(',').unwrap();

    let first_elf = parse_assignment(first);
    let second_elf = parse_assignment(second);
    (first_elf, second_elf)
}

fn is_completely_overlapping(
    first: &RangeInclusive<usize>,
    second: &RangeInclusive<usize>,
) -> bool {
    (first.start() <= second.start() && first.end() >= second.end())
        || (first.start() >= second.start() && first.end() <= second.end())
}

fn has_any_overlap(first: &RangeInclusive<usize>, second: &RangeInclusive<usize>) -> bool {
    first.start() <= second.end() && first.end() >= second.start()
}

#[aoc(day4, part1)]
fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_pair)
        .filter(|(elf_1, elf_2)| is_completely_overlapping(elf_1, elf_2))
        .count()
}

#[aoc(day4, part2)]
fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_pair)
        .filter(|(elf_1, elf_2)| has_any_overlap(elf_1, elf_2))
        .count()
}
