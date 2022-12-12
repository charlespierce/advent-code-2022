use aoc_runner_derive::aoc;
use std::collections::VecDeque;

trait Trickster {
    fn inspect_next(&mut self) -> Option<Throw>;

    fn inspect_next_part2(&mut self) -> Option<Throw>;

    fn catch(&mut self, item: usize);

    fn num_inspected(&self) -> usize;
}

struct Monkey<O, T> {
    items: VecDeque<usize>,
    operation: O,
    test: T,
    inspected: usize,
}

impl<O, T> Monkey<O, T>
where
    O: Fn(usize) -> usize,
    T: Fn(usize) -> usize,
{
    fn new<I>(items: I, operation: O, test: T) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        Self {
            items: VecDeque::from_iter(items),
            operation,
            test,
            inspected: 0,
        }
    }
}

impl<O, T> Trickster for Monkey<O, T>
where
    O: Fn(usize) -> usize,
    T: Fn(usize) -> usize,
{
    fn inspect_next(&mut self) -> Option<Throw> {
        let starting_worry = self.items.pop_front()?;
        let after_inspecting = (self.operation)(starting_worry) / 3;
        self.inspected += 1;

        let target = (self.test)(after_inspecting);
        Some(Throw::new(target, after_inspecting))
    }

    fn inspect_next_part2(&mut self) -> Option<Throw> {
        let starting_worry = self.items.pop_front()?;
        // Note: This is the product of all of the divisors that we use in the operations. Modding
        // should not change the ultimate mod answer, nor any of the transformations, so this
        // will let us keep the parity without having to track the gigantic numbers.
        let after_inspecting = (self.operation)(starting_worry) % 9_699_690;
        self.inspected += 1;

        let target = (self.test)(after_inspecting);
        Some(Throw::new(target, after_inspecting))
    }

    fn catch(&mut self, item: usize) {
        self.items.push_back(item);
    }

    fn num_inspected(&self) -> usize {
        self.inspected
    }
}

struct Throw {
    target: usize,
    item: usize,
}

impl Throw {
    fn new(target: usize, item: usize) -> Self {
        Self { target, item }
    }
}

struct KeepAway {
    monkeys: Vec<Box<dyn Trickster>>,
}

impl KeepAway {
    fn new() -> Self {
        // Manually created rather than trying to parse the input (7 monkeys)

        let monkeys: Vec<Box<dyn Trickster>> = vec![
            Box::new(Monkey::new(
                [56, 56, 92, 65, 71, 61, 79],
                |old| old * 7,
                |item| {
                    if item % 3 == 0 {
                        3
                    } else {
                        7
                    }
                },
            )),
            Box::new(Monkey::new(
                [61, 85],
                |old| old + 5,
                |item| {
                    if item % 11 == 0 {
                        6
                    } else {
                        4
                    }
                },
            )),
            Box::new(Monkey::new(
                [54, 96, 82, 78, 69],
                |old| old * old,
                |item| {
                    if item % 7 == 0 {
                        0
                    } else {
                        7
                    }
                },
            )),
            Box::new(Monkey::new(
                [57, 59, 65, 95],
                |old| old + 4,
                |item| {
                    if item % 2 == 0 {
                        5
                    } else {
                        1
                    }
                },
            )),
            Box::new(Monkey::new(
                [62, 67, 80],
                |old| old * 17,
                |item| {
                    if item % 19 == 0 {
                        2
                    } else {
                        6
                    }
                },
            )),
            Box::new(Monkey::new(
                [91],
                |old| old + 7,
                |item| {
                    if item % 5 == 0 {
                        1
                    } else {
                        4
                    }
                },
            )),
            Box::new(Monkey::new(
                [79, 83, 64, 52, 77, 56, 63, 92],
                |old| old + 6,
                |item| {
                    if item % 17 == 0 {
                        2
                    } else {
                        0
                    }
                },
            )),
            Box::new(Monkey::new(
                [50, 97, 76, 96, 80, 56],
                |old| old + 3,
                |item| {
                    if item % 13 == 0 {
                        3
                    } else {
                        5
                    }
                },
            )),
        ];

        Self { monkeys }
    }

    fn take_turn(&mut self, index: usize) {
        while let Some(throw) = self.monkeys[index].inspect_next() {
            self.monkeys[throw.target].catch(throw.item);
        }
    }

    fn take_round(&mut self) {
        for index in 0..self.monkeys.len() {
            self.take_turn(index);
        }
    }

    fn take_turn_part2(&mut self, index: usize) {
        while let Some(throw) = self.monkeys[index].inspect_next_part2() {
            self.monkeys[throw.target].catch(throw.item);
        }
    }

    fn take_round_part2(&mut self) {
        for index in 0..self.monkeys.len() {
            self.take_turn_part2(index);
        }
    }

    fn most_active(&self) -> (usize, usize) {
        self.monkeys.iter().fold((0, 0), |(most, second), monkey| {
            let new = monkey.num_inspected();
            if new > most {
                (new, most)
            } else if new > second {
                (most, new)
            } else {
                (most, second)
            }
        })
    }
}

#[aoc(day11, part1)]
fn solve_part1(_input: &str) -> usize {
    let mut game = KeepAway::new();

    for _ in 0..20 {
        game.take_round();
    }

    let (most, second) = game.most_active();
    most * second
}

#[aoc(day11, part2)]
fn solve_part2(_input: &str) -> usize {
    let mut game = KeepAway::new();

    for _ in 0..10_000 {
        game.take_round_part2();
    }

    let (most, second) = game.most_active();
    most * second
}
