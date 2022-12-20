use aoc_runner_derive::aoc;
use indexmap::IndexSet;

struct Encrypted {
    original: Vec<isize>,
    current: IndexSet<(usize, isize)>,
}

impl Encrypted {
    fn calc_end_index(&self, start_index: usize, delta: isize) -> usize {
        let count = self.original.len() as isize - 1;

        let value = ((start_index as isize) + delta) % count;

        if value < 0 {
            (value + count) as usize
        } else {
            value as usize
        }
    }

    fn calc_index(&self, index: usize) -> usize {
        let count = self.original.len();

        index % count
    }

    fn decrypt_pass(&mut self) {
        for (index, &value) in self.original.iter().enumerate() {
            let start_index = self.current.get_index_of(&(index, value)).unwrap();
            let end_index = self.calc_end_index(start_index, value);

            self.current.move_index(start_index, end_index);
        }
    }

    fn coordinates(&self) -> (isize, isize, isize) {
        let entry = self.current.iter().find(|(_, value)| *value == 0).unwrap();
        let start = self.current.get_index_of(entry).unwrap();
        let first_index = self.calc_index(start + 1000);
        let second_index = self.calc_index(start + 2000);
        let third_index = self.calc_index(start + 3000);

        (
            self.current.get_index(first_index).unwrap().1,
            self.current.get_index(second_index).unwrap().1,
            self.current.get_index(third_index).unwrap().1,
        )
    }
}

impl FromIterator<isize> for Encrypted {
    fn from_iter<T: IntoIterator<Item = isize>>(iter: T) -> Self {
        let mut encrypted = Encrypted {
            original: Vec::new(),
            current: IndexSet::new(),
        };

        for (index, value) in iter.into_iter().enumerate() {
            encrypted.original.push(value);
            encrypted.current.insert((index, value));
        }

        encrypted
    }
}

fn parse_encrypted(input: &str, key: isize) -> Encrypted {
    input
        .lines()
        .map(|line| key * line.parse::<isize>().unwrap())
        .collect()
}

#[aoc(day20, part1)]
fn solve_part1(input: &str) -> isize {
    let mut encrypted = parse_encrypted(input, 1);
    encrypted.decrypt_pass();

    let (first, second, third) = encrypted.coordinates();
    first + second + third
}

#[aoc(day20, part2)]
fn solve_part2(input: &str) -> isize {
    let mut encrypted = parse_encrypted(input, 811589153);

    for _ in 0..10 {
        encrypted.decrypt_pass();
    }

    let (first, second, third) = encrypted.coordinates();
    first + second + third
}
