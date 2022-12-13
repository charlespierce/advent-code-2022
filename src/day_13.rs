use aoc_runner_derive::aoc;
use std::cmp::Ordering;

#[derive(Clone)]
enum Signal {
    Int(u32),
    List(Vec<Signal>),
}

impl PartialEq for Signal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Signal::Int(left), Signal::Int(right)) => left.eq(right),
            (Signal::List(left), Signal::List(right)) => left.eq(right),
            (left @ Signal::List(_), Signal::Int(right)) => {
                left.eq(&Signal::List(vec![Signal::Int(*right)]))
            }
            (Signal::Int(left), right @ Signal::List(_)) => {
                Signal::List(vec![Signal::Int(*left)]).eq(right)
            }
        }
    }
}

impl Eq for Signal {}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Signal::Int(left), Signal::Int(right)) => left.cmp(right),
            (Signal::List(left), Signal::List(right)) => left.cmp(right),
            (left @ Signal::List(_), Signal::Int(right)) => {
                left.cmp(&Signal::List(vec![Signal::Int(*right)]))
            }
            (Signal::Int(left), right @ Signal::List(_)) => {
                Signal::List(vec![Signal::Int(*left)]).cmp(right)
            }
        }
    }
}

#[aoc(day13, part1)]
fn solve_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(index, pair)| {
            let mut lines = pair.lines();
            let left = parser::parse_signal(lines.next().unwrap());
            let right = parser::parse_signal(lines.next().unwrap());

            if left < right {
                Some(index + 1)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day13, part2)]
fn solve_part2(input: &str) -> usize {
    let first_divider = parser::parse_signal("[[2]]");
    let second_divider = parser::parse_signal("[[6]]");

    let mut signals = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parser::parse_signal)
        .collect::<Vec<_>>();
    signals.push(first_divider.clone());
    signals.push(second_divider.clone());
    signals.sort();

    signals
        .into_iter()
        .enumerate()
        .fold(1, |acc, (index, signal)| {
            if signal == first_divider || signal == second_divider {
                acc * (index + 1)
            } else {
                acc
            }
        })
}

mod parser {
    use super::Signal;

    use nom::branch::alt;
    use nom::character::complete::{char, u32};
    use nom::combinator::all_consuming;
    use nom::multi::separated_list0;
    use nom::sequence::delimited;
    use nom::IResult;

    fn integer(input: &str) -> IResult<&str, Signal> {
        let (rest, value) = u32(input)?;
        Ok((rest, Signal::Int(value)))
    }

    fn list(input: &str) -> IResult<&str, Signal> {
        let (rest, value) =
            delimited(char('['), separated_list0(char(','), signal), char(']'))(input)?;

        Ok((rest, Signal::List(value)))
    }

    fn signal(input: &str) -> IResult<&str, Signal> {
        alt((integer, list))(input)
    }

    pub(super) fn parse_signal(input: &str) -> Signal {
        all_consuming(signal)(input).unwrap().1
    }
}
