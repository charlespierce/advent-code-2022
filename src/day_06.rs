use aoc_runner_derive::aoc;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

struct Communicator {
    window_size: usize,
    char_count: usize,
    buffer: VecDeque<char>,
    chars: HashMap<char, usize>,
}

impl Communicator {
    fn new(window_size: usize) -> Self {
        Communicator {
            window_size,
            char_count: 0,
            buffer: VecDeque::with_capacity(4),
            chars: HashMap::with_capacity(4),
        }
    }

    fn remove_char(&mut self) {
        let removed = self.buffer.pop_front().unwrap();
        if let Entry::Occupied(mut occupied) = self.chars.entry(removed) {
            *occupied.get_mut() -= 1;
            if *occupied.get() == 0 {
                occupied.remove();
            }
        }
    }

    fn process_char(&mut self, chr: char) {
        self.char_count += 1;

        if self.buffer.len() == self.window_size {
            self.remove_char();
        }
        self.buffer.push_back(chr);
        *self.chars.entry(chr).or_default() += 1;
    }

    fn has_start_marker(&self) -> bool {
        self.chars.len() == self.window_size
    }

    fn chars_seen(&self) -> usize {
        self.char_count
    }
}

#[aoc(day6, part1)]
fn solve_part1(input: &str) -> usize {
    let mut comm = Communicator::new(4);

    for chr in input.chars() {
        comm.process_char(chr);

        if comm.has_start_marker() {
            return comm.chars_seen();
        }
    }

    0
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> usize {
    let mut comm = Communicator::new(14);

    for chr in input.chars() {
        comm.process_char(chr);

        if comm.has_start_marker() {
            return comm.chars_seen();
        }
    }

    0
}
