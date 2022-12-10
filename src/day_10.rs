use aoc_runner_derive::aoc;

enum Instruction {
    Noop,
    Addx(isize),
}

struct Communicator<F> {
    register: isize,
    cycle: isize,
    cycle_callback: F,
}

impl<F> Communicator<F>
where
    F: FnMut(isize, isize),
{
    fn new(mut cycle_callback: F) -> Self {
        cycle_callback(1, 1);

        Self {
            register: 1,
            cycle: 1,
            cycle_callback,
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => {
                self.cycle += 1;
            }
            Instruction::Addx(delta) => {
                self.cycle += 1;
                (self.cycle_callback)(self.cycle, self.register);
                self.cycle += 1;
                self.register += delta;
            }
        }

        (self.cycle_callback)(self.cycle, self.register);
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| match line {
            "noop" => Instruction::Noop,
            _ => {
                let delta = line.split(' ').nth(1).unwrap().parse().unwrap();
                Instruction::Addx(delta)
            }
        })
        .collect()
}

#[aoc(day10, part1)]
fn solve_part1(input: &str) -> isize {
    let mut signal_strength = 0;
    let mut comm = Communicator::new(|cycle, register| {
        if cycle % 40 == 20 {
            signal_strength += cycle * register;
        }
    });

    for instruction in parse_instructions(input) {
        comm.execute_instruction(instruction);
    }

    signal_strength
}

#[aoc(day10, part2)]
fn solve_part2(input: &str) -> String {
    let mut screen = String::with_capacity(247);
    screen.push('\n'); // So the output is aligned
    let mut comm = Communicator::new(|cycle, register| {
        let x_position = (cycle - 1) % 40;
        if (register - x_position).abs() <= 1 {
            screen.push('#');
        } else {
            screen.push(' ');
        }

        if x_position == 39 {
            screen.push('\n');
        }
    });

    for instruction in parse_instructions(input) {
        comm.execute_instruction(instruction);
    }

    screen
}
