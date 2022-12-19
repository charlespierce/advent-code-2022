use aoc_runner_derive::aoc;
use nom::{bytes::complete::tag, character::complete::u64, sequence::tuple, IResult};

#[derive(Clone, Copy, Debug)]
enum RobotKind {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, Copy)]
struct Cost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

#[derive(Clone, Copy, Debug)]
struct Resources {
    ore: usize,
    ore_robots: usize,
    clay: usize,
    clay_robots: usize,
    obsidian: usize,
    obsidian_robots: usize,
    geodes: usize,
    geode_robots: usize,
}

impl Resources {
    fn new() -> Self {
        Resources {
            ore: 0,
            ore_robots: 1,
            clay: 0,
            clay_robots: 0,
            obsidian: 0,
            obsidian_robots: 0,
            geodes: 0,
            geode_robots: 0,
        }
    }

    fn collect(self) -> Self {
        Resources {
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geodes: self.geodes + self.geode_robots,
            ..self
        }
    }

    fn can_build(self, cost: Cost) -> bool {
        self.ore >= cost.ore && self.clay >= cost.clay && self.obsidian >= cost.obsidian
    }

    fn build(self, robot: RobotKind, cost: Cost) -> Self {
        let (ore_robots, clay_robots, obsidian_robots, geode_robots) = match robot {
            RobotKind::Ore => (
                self.ore_robots + 1,
                self.clay_robots,
                self.obsidian_robots,
                self.geode_robots,
            ),
            RobotKind::Clay => (
                self.ore_robots,
                self.clay_robots + 1,
                self.obsidian_robots,
                self.geode_robots,
            ),
            RobotKind::Obsidian => (
                self.ore_robots,
                self.clay_robots,
                self.obsidian_robots + 1,
                self.geode_robots,
            ),
            RobotKind::Geode => (
                self.ore_robots,
                self.clay_robots,
                self.obsidian_robots,
                self.geode_robots + 1,
            ),
        };

        Self {
            ore: self.ore - cost.ore,
            ore_robots,
            clay: self.clay - cost.clay,
            clay_robots,
            obsidian: self.obsidian - cost.obsidian,
            obsidian_robots,
            geodes: self.geodes,
            geode_robots,
        }
    }
}

struct Factory {
    ore_robot: Cost,
    clay_robot: Cost,
    obsidian_robot: Cost,
    geode_robot: Cost,
    max_ore: usize,
    max_clay: usize,
    max_obsidian: usize,
}

impl Factory {
    fn new(ore_robot: Cost, clay_robot: Cost, obsidian_robot: Cost, geode_robot: Cost) -> Self {
        let max_ore = ore_robot
            .ore
            .max(clay_robot.ore)
            .max(obsidian_robot.ore)
            .max(geode_robot.ore);
        let max_clay = obsidian_robot.clay;
        let max_obsidian = geode_robot.obsidian;

        Self {
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
            max_ore,
            max_clay,
            max_obsidian,
        }
    }
    fn cost(&self, kind: RobotKind) -> Cost {
        match kind {
            RobotKind::Ore => self.ore_robot,
            RobotKind::Clay => self.clay_robot,
            RobotKind::Obsidian => self.obsidian_robot,
            RobotKind::Geode => self.geode_robot,
        }
    }

    fn reasonable_to_build(&self, kind: RobotKind, resources: Resources) -> bool {
        let cost = self.cost(kind);

        let over_resourced = match kind {
            RobotKind::Ore => resources.ore_robots > self.max_ore,
            RobotKind::Clay => resources.clay_robots > self.max_clay,
            RobotKind::Obsidian => resources.obsidian_robots > self.max_obsidian,
            RobotKind::Geode => false,
        };

        !over_resourced
            && (cost.ore == 0 || resources.ore_robots > 0)
            && (cost.clay == 0 || resources.clay_robots > 0)
            && (cost.obsidian == 0 || resources.obsidian_robots > 0)
    }

    fn next_build_options(&self, resources: Resources) -> Vec<RobotKind> {
        [
            RobotKind::Ore,
            RobotKind::Clay,
            RobotKind::Obsidian,
            RobotKind::Geode,
        ]
        .into_iter()
        .filter(|kind| self.reasonable_to_build(*kind, resources))
        .collect()
    }

    fn maximize(&self, time: usize) -> usize {
        let resources = Resources::new();
        self.next_build_options(resources)
            .into_iter()
            .map(|kind| self.maximum_geodes(resources, kind, time))
            .max()
            .unwrap()
    }

    fn maximum_geodes(
        &self,
        resources: Resources,
        active: RobotKind,
        time_remaining: usize,
    ) -> usize {
        if time_remaining == 0 {
            return resources.geodes;
        }

        let active_cost = self.cost(active);
        let can_build = resources.can_build(active_cost);
        let resources = resources.collect();

        if can_build {
            let resources = resources.build(active, active_cost);
            self.next_build_options(resources)
                .into_iter()
                .map(|new_active| self.maximum_geodes(resources, new_active, time_remaining - 1))
                .max()
                .unwrap()
        } else {
            self.maximum_geodes(resources, active, time_remaining - 1)
        }
    }
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    let (rest, value) = u64(input)?;
    Ok((rest, value as usize))
}

fn parse_factory(input: &str) -> (usize, Factory) {
    let (
        _,
        (
            _,
            id,
            _,
            ore_ore,
            _,
            clay_ore,
            _,
            obsidian_ore,
            _,
            obsidian_clay,
            _,
            geode_ore,
            _,
            geode_obsidian,
            _,
        ),
    ) = tuple((
        tag("Blueprint "),
        parse_usize,
        tag(": Each ore robot costs "),
        parse_usize,
        tag(" ore. Each clay robot costs "),
        parse_usize,
        tag(" ore. Each obsidian robot costs "),
        parse_usize,
        tag(" ore and "),
        parse_usize,
        tag(" clay. Each geode robot costs "),
        parse_usize,
        tag(" ore and "),
        parse_usize,
        tag(" obsidian."),
    ))(input)
    .unwrap();

    let factory = Factory::new(
        Cost {
            ore: ore_ore,
            clay: 0,
            obsidian: 0,
        },
        Cost {
            ore: clay_ore,
            clay: 0,
            obsidian: 0,
        },
        Cost {
            ore: obsidian_ore,
            clay: obsidian_clay,
            obsidian: 0,
        },
        Cost {
            ore: geode_ore,
            clay: 0,
            obsidian: geode_obsidian,
        },
    );

    (id, factory)
}

#[aoc(day19, part1)]
fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_factory)
        .map(|(id, factory)| {
            let geodes = factory.maximize(24);
            id * geodes
        })
        .sum()
}

#[aoc(day19, part2)]
fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .take(3)
        .map(parse_factory)
        .map(|(_, factory)| factory.maximize(32))
        .product()
}
