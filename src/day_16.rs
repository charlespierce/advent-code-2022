use aoc_runner_derive::aoc;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u64},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::collections::{HashMap, HashSet};

struct Cavern {
    flow: HashMap<usize, usize>,
    distance_map: HashMap<usize, HashMap<usize, usize>>,
}

struct MoveOption {
    end: usize,
    cost: usize,
    flow: usize,
}

impl Cavern {
    fn distance(&self, from: usize, to: usize) -> usize {
        *self.distance_map.get(&from).unwrap().get(&to).unwrap()
    }

    fn options(
        &self,
        state: State,
        time_remaining: usize,
    ) -> impl Iterator<Item = MoveOption> + '_ {
        self.flow.iter().filter_map(move |(&id, &flow)| {
            if id == state.position || state.is_activated(id) {
                return None;
            }

            // Adding one to include activating the valve
            let cost = self.distance(state.position, id) + 1;

            if cost > time_remaining {
                None
            } else {
                Some(MoveOption {
                    end: id,
                    cost,
                    flow,
                })
            }
        })
    }

    fn maximum_flow(&self, state: State, time_remaining: usize) -> usize {
        self.options(state, time_remaining)
            .map(|option| {
                let new_time = time_remaining - option.cost;
                let pressure = option.flow * new_time;
                pressure + self.maximum_flow(state.move_and_activate(option.end), new_time)
            })
            .max()
            .unwrap_or(0)
    }

    fn flow_final_states(&self, state: State, time_remaining: usize) -> HashSet<(usize, usize)> {
        let mut states = HashSet::new();

        for option in self.options(state, time_remaining) {
            let new_time = time_remaining - option.cost;
            let pressure = option.flow * new_time;

            for (flow, activated) in
                self.flow_final_states(state.move_and_activate(option.end), new_time)
            {
                states.insert((pressure + flow, activated));
            }
        }

        if states.is_empty() {
            states.insert((0, state.activated));
        }

        states
    }
}

struct Valve {
    flow: usize,
    neighbors: Vec<usize>,
}

#[derive(Clone, Copy)]
struct State {
    position: usize,
    activated: usize,
}

impl State {
    fn new(start: usize) -> Self {
        State {
            position: start,
            activated: 0,
        }
    }

    fn is_activated(self, id: usize) -> bool {
        let mask = 1 << id;
        self.activated & mask > 0
    }

    fn move_and_activate(self, end: usize) -> Self {
        let mask = 1 << end;
        State {
            position: end,
            activated: self.activated | mask,
        }
    }
}
fn determine_ids(input: &str) -> (usize, HashMap<&str, usize>) {
    let mut id_map = HashMap::new();
    let mut start = None;

    for (id, str_id) in input.lines().map(|line| &line[6..8]).enumerate() {
        id_map.insert(str_id, id);
        if str_id == "AA" {
            start = Some(id);
        }
    }

    (start.unwrap(), id_map)
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (rest, value) = u64(input)?;
    Ok((rest, value as usize))
}

fn parse_str(input: &str) -> IResult<&str, &str> {
    let (rest, value) = alpha1(input)?;
    Ok((rest, value))
}

fn parse_valve(input: &str, id_map: &HashMap<&str, usize>) -> (usize, Valve) {
    let (_, (_, id, _, flow, _, neighbors)) = tuple((
        tag("Valve "),
        parse_str,
        tag(" has flow rate="),
        parse_usize,
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), parse_str),
    ))(input)
    .unwrap();

    let self_id = *id_map.get(id).unwrap();
    let neighbors = neighbors
        .into_iter()
        .map(|id| *id_map.get(id).unwrap())
        .collect();

    (self_id, Valve { flow, neighbors })
}

fn calculate_distances(valves: &HashMap<usize, Valve>) -> HashMap<usize, HashMap<usize, usize>> {
    let valve_count = valves.len();
    let mut distances: HashMap<usize, HashMap<usize, usize>> = HashMap::new();

    for (id, valve) in valves.iter() {
        for end_id in valve.neighbors.iter() {
            distances.entry(*id).or_default().insert(*end_id, 1);
            distances.entry(*end_id).or_default().insert(*id, 1);
        }
        distances.entry(*id).or_default().insert(*id, 0);
    }

    for k in 0..valve_count {
        for i in 0..valve_count {
            for j in 0..valve_count {
                let current = distances.get(&i).unwrap().get(&j).copied();
                let maybe_first_leg = distances.get(&i).unwrap().get(&k).copied();
                let maybe_second_leg = distances.get(&k).unwrap().get(&j).copied();

                if let (Some(first), Some(second)) = (maybe_first_leg, maybe_second_leg) {
                    match current {
                        None => {
                            distances.entry(i).or_default().insert(j, first + second);
                        }
                        Some(curr) => {
                            if curr > first + second {
                                distances.entry(i).or_default().insert(j, first + second);
                            }
                        }
                    }
                }
            }
        }
    }

    distances
}

fn parse_cavern(input: &str) -> (usize, Cavern) {
    let (start, id_map) = determine_ids(input);
    let valves: HashMap<_, _> = input
        .lines()
        .map(|line| parse_valve(line, &id_map))
        .collect();
    let distance_map = calculate_distances(&valves);

    let flow = valves
        .into_iter()
        .filter_map(|(id, valve)| {
            if valve.flow > 0 {
                Some((id, valve.flow))
            } else {
                None
            }
        })
        .collect();

    (start, Cavern { flow, distance_map })
}

#[aoc(day16, part1)]
fn solve_part1(input: &str) -> usize {
    let (start, cavern) = parse_cavern(input);
    cavern.maximum_flow(State::new(start), 30)
}

#[aoc(day16, part2)]
fn solve_part2(input: &str) -> usize {
    let (start, cavern) = parse_cavern(input);
    let final_states = cavern.flow_final_states(State::new(start), 26);

    let mut max_flow = 0;
    for &(flow_1, activated_1) in final_states.iter() {
        for &(flow_2, activated_2) in final_states.iter() {
            if activated_1 & activated_2 == 0 {
                max_flow = max_flow.max(flow_1 + flow_2);
            }
        }
    }

    max_flow
}
