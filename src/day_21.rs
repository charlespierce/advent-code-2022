use aoc_runner_derive::aoc;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, i64},
    sequence::tuple,
    IResult,
};
use std::collections::HashMap;

enum Monkey<'a> {
    Number(isize),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

struct MonkeyArray<'a> {
    monkeys: HashMap<&'a str, Monkey<'a>>,
}

impl<'a> MonkeyArray<'a> {
    fn calculate(&self, id: &str) -> isize {
        let monkey = self.monkeys.get(id).unwrap();

        match monkey {
            Monkey::Number(value) => *value,
            Monkey::Add(first, second) => self.calculate(first) + self.calculate(second),
            Monkey::Sub(first, second) => self.calculate(first) - self.calculate(second),
            Monkey::Mul(first, second) => self.calculate(first) * self.calculate(second),
            Monkey::Div(first, second) => self.calculate(first) / self.calculate(second),
        }
    }

    fn determine_unknown(&self, id: &str, expected: isize) -> isize {
        if id == "humn" {
            return expected;
        }

        let monkey = self.monkeys.get(id).unwrap();

        match monkey {
            Monkey::Add(first, second) => {
                let first_val = self.calculate_with_unknown(first);
                let second_val = self.calculate_with_unknown(second);

                match (first_val, second_val) {
                    (Some(num), None) => self.determine_unknown(second, expected - num),
                    (None, Some(num)) => self.determine_unknown(first, expected - num),
                    _ => unreachable!(),
                }
            }
            Monkey::Sub(first, second) => {
                let first_val = self.calculate_with_unknown(first);
                let second_val = self.calculate_with_unknown(second);

                match (first_val, second_val) {
                    (Some(num), None) => self.determine_unknown(second, num - expected),
                    (None, Some(num)) => self.determine_unknown(first, expected + num),
                    _ => unreachable!(),
                }
            }
            Monkey::Mul(first, second) => {
                let first_val = self.calculate_with_unknown(first);
                let second_val = self.calculate_with_unknown(second);

                match (first_val, second_val) {
                    (Some(num), None) => self.determine_unknown(second, expected / num),
                    (None, Some(num)) => self.determine_unknown(first, expected / num),
                    _ => unreachable!(),
                }
            }
            Monkey::Div(first, second) => {
                let first_val = self.calculate_with_unknown(first);
                let second_val = self.calculate_with_unknown(second);

                match (first_val, second_val) {
                    (Some(num), None) => self.determine_unknown(second, num / expected),
                    (None, Some(num)) => self.determine_unknown(first, expected * num),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    fn calculate_with_unknown(&self, id: &str) -> Option<isize> {
        if id == "humn" {
            return None;
        }

        let monkey = self.monkeys.get(id).unwrap();

        match monkey {
            Monkey::Number(value) => Some(*value),
            Monkey::Add(first, second) => {
                Some(self.calculate_with_unknown(first)? + self.calculate_with_unknown(second)?)
            }
            Monkey::Sub(first, second) => {
                Some(self.calculate_with_unknown(first)? - self.calculate_with_unknown(second)?)
            }
            Monkey::Mul(first, second) => {
                Some(self.calculate_with_unknown(first)? * self.calculate_with_unknown(second)?)
            }
            Monkey::Div(first, second) => {
                Some(self.calculate_with_unknown(first)? / self.calculate_with_unknown(second)?)
            }
        }
    }
}

fn parse_number(input: &str) -> IResult<&str, Monkey> {
    let (rest, value) = i64(input)?;
    Ok((rest, Monkey::Number(value as isize)))
}

fn parse_add(input: &str) -> IResult<&str, Monkey> {
    let (rest, (first, _, second)) = tuple((alpha1, tag(" + "), alpha1))(input)?;
    Ok((rest, Monkey::Add(first, second)))
}

fn parse_sub(input: &str) -> IResult<&str, Monkey> {
    let (rest, (first, _, second)) = tuple((alpha1, tag(" - "), alpha1))(input)?;
    Ok((rest, Monkey::Sub(first, second)))
}

fn parse_mul(input: &str) -> IResult<&str, Monkey> {
    let (rest, (first, _, second)) = tuple((alpha1, tag(" * "), alpha1))(input)?;
    Ok((rest, Monkey::Mul(first, second)))
}

fn parse_div(input: &str) -> IResult<&str, Monkey> {
    let (rest, (first, _, second)) = tuple((alpha1, tag(" / "), alpha1))(input)?;
    Ok((rest, Monkey::Div(first, second)))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    alt((parse_number, parse_add, parse_sub, parse_mul, parse_div))(input)
}

fn parse_line(input: &str) -> (&str, Monkey) {
    let (_, (id, _, monkey)) = tuple((alpha1, tag(": "), parse_monkey))(input).unwrap();

    (id, monkey)
}

fn parse_monkey_array(input: &str) -> MonkeyArray {
    MonkeyArray {
        monkeys: input.lines().map(parse_line).collect(),
    }
}

#[aoc(day21, part1)]
fn solve_part1(input: &str) -> isize {
    let array = parse_monkey_array(input);

    array.calculate("root")
}

#[aoc(day21, part2)]
fn solve_part2(input: &str) -> isize {
    let array = parse_monkey_array(input);

    match array.monkeys.get("root").unwrap() {
        Monkey::Add(first, second)
        | Monkey::Sub(first, second)
        | Monkey::Mul(first, second)
        | Monkey::Div(first, second) => {
            let first_val = array.calculate_with_unknown(first);
            let second_val = array.calculate_with_unknown(second);

            match (first_val, second_val) {
                (Some(num), None) => array.determine_unknown(second, num),
                (None, Some(num)) => array.determine_unknown(first, num),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}
