use aoc_runner_derive::aoc;
use std::ops::Add;

#[derive(Default)]
struct SnafuNumber {
    digits: Vec<i8>,
}

impl SnafuNumber {
    fn from_str(input: &str) -> Self {
        let digits = input
            .chars()
            .rev()
            .map(|chr| match chr {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => unreachable!(),
            })
            .collect();

        Self { digits }
    }

    fn into_str(self) -> String {
        self.digits
            .into_iter()
            .rev()
            .map(|digit| match digit {
                2 => '2',
                1 => '1',
                0 => '0',
                -1 => '-',
                -2 => '=',
                _ => unreachable!(),
            })
            .collect()
    }
}

impl Add for SnafuNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut digits = Vec::new();
        let mut carry = 0;

        let max_len = self.digits.len().max(rhs.digits.len());
        for i in 0..max_len {
            let left = self.digits.get(i).copied().unwrap_or(0);
            let right = rhs.digits.get(i).copied().unwrap_or(0);

            match left + right + carry {
                5 => {
                    carry = 1;
                    digits.push(0);
                }
                4 => {
                    carry = 1;
                    digits.push(-1);
                }
                3 => {
                    carry = 1;
                    digits.push(-2);
                }
                2 => {
                    carry = 0;
                    digits.push(2);
                }
                1 => {
                    carry = 0;
                    digits.push(1);
                }
                0 => {
                    carry = 0;
                    digits.push(0);
                }
                -1 => {
                    carry = 0;
                    digits.push(-1);
                }
                -2 => {
                    carry = 0;
                    digits.push(-2);
                }
                -3 => {
                    carry = -1;
                    digits.push(2);
                }
                -4 => {
                    carry = -1;
                    digits.push(1);
                }
                -5 => {
                    carry = -1;
                    digits.push(0);
                }
                _ => unreachable!(),
            }
        }

        if carry != 0 {
            digits.push(carry);
        }

        Self { digits }
    }
}

#[aoc(day25, part1)]
fn solve_part1(input: &str) -> String {
    input
        .lines()
        .map(SnafuNumber::from_str)
        .fold(SnafuNumber::default(), |acc, num| acc + num)
        .into_str()
}

#[aoc(day25, part2)]
fn solve_part2(_input: &str) -> usize {
    0
}
