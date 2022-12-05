use aoc_runner_derive::aoc;

#[derive(Copy, Clone)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

struct Stacks {
    inner: Vec<Vec<char>>,
}

impl Stacks {
    fn from_layout(input: &str) -> Self {
        // with_capacity because I looked at the input and saw how many there are :)
        let mut stacks = Stacks {
            inner: Vec::with_capacity(9),
        };

        for line in input.lines().rev().skip(1) {
            for (index, chr) in line.chars().skip(1).step_by(4).enumerate() {
                if index >= stacks.inner.len() {
                    stacks.inner.push(Vec::new());
                }
                // Space means there wasn't anything there
                if chr != ' ' {
                    stacks.inner[index].push(chr);
                }
            }
        }

        stacks
    }

    fn apply_move(&mut self, mv: Move) {
        let index = self.inner[mv.from].len() - mv.count;
        let containers = self.inner[mv.from].split_off(index);
        self.inner[mv.to].extend(containers.into_iter().rev());
    }

    fn apply_move_9001(&mut self, mv: Move) {
        let index = self.inner[mv.from].len() - mv.count;
        let containers = self.inner[mv.from].split_off(index);
        self.inner[mv.to].extend(containers);
    }

    fn top_values(&self) -> String {
        self.inner
            .iter()
            .map(|stack| *stack.last().unwrap())
            .collect()
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let mut values = line.split(' ').skip(1).step_by(2);
            let count = values.next().unwrap().parse::<usize>().unwrap();
            let from = values.next().unwrap().parse::<usize>().unwrap() - 1;
            let to = values.next().unwrap().parse::<usize>().unwrap() - 1;

            Move { count, from, to }
        })
        .collect()
}

#[aoc(day5, part1)]
fn solve_part1(input: &str) -> String {
    let (layout, moves) = input.split_once("\n\n").unwrap();
    let mut stacks = Stacks::from_layout(layout);

    for mv in parse_moves(moves) {
        stacks.apply_move(mv);
    }

    stacks.top_values()
}

#[aoc(day5, part2)]
fn solve_part2(input: &str) -> String {
    let (layout, moves) = input.split_once("\n\n").unwrap();
    let mut stacks = Stacks::from_layout(layout);

    for mv in parse_moves(moves) {
        stacks.apply_move_9001(mv);
    }

    stacks.top_values()
}
