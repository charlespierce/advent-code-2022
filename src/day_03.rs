use aoc_runner_derive::aoc;
use std::collections::HashSet;

fn shared_item(rucksack: &str) -> u8 {
    let midpoint = rucksack.len() / 2;
    let (first, second) = rucksack.split_at(midpoint);

    let first_set: HashSet<_> = first.as_bytes().iter().collect();
    let second_set: HashSet<_> = second.as_bytes().iter().collect();

    **first_set.intersection(&second_set).next().unwrap()
}

fn badge_item(first_elf: &str, second_elf: &str, third_elf: &str) -> u8 {
    let first_set: HashSet<_> = first_elf.as_bytes().iter().collect();
    let second_set: HashSet<_> = second_elf.as_bytes().iter().collect();
    let third_set: HashSet<_> = third_elf.as_bytes().iter().collect();

    let first_inter: HashSet<_> = first_set.intersection(&second_set).copied().collect();

    **first_inter.intersection(&third_set).next().unwrap()
}

fn priority(chr: u8) -> usize {
    if chr > 96 {
        (chr - 96) as usize
    } else {
        (chr - 38) as usize
    }
}

#[aoc(day3, part1)]
fn solve_part1(input: &str) -> usize {
    input.lines().map(shared_item).map(priority).sum()
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .fold((0, Vec::new()), |(acc, mut elves), elf| {
            elves.push(elf);

            if elves.len() == 3 {
                (
                    acc + priority(badge_item(elves[0], elves[1], elves[2])),
                    Vec::new(),
                )
            } else {
                (acc, elves)
            }
        })
        .0
}
