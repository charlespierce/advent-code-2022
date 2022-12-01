use aoc_runner_derive::aoc;

fn parse_calories(input: &str) -> impl Iterator<Item = usize> + '_ {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<usize>().unwrap()).sum())
}

#[aoc(day1, part1)]
fn solve_part1(input: &str) -> usize {
    parse_calories(input).max().unwrap()
}

#[aoc(day1, part2)]
fn solve_part2(input: &str) -> usize {
    let (first, second, third) =
        parse_calories(input).fold((0, 0, 0), |(most, second, third), cals| {
            if cals > most {
                (cals, most, second)
            } else if cals > second {
                (most, cals, second)
            } else if cals > third {
                (most, second, cals)
            } else {
                (most, second, third)
            }
        });

    first + second + third
}
