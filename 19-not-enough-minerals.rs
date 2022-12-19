use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead, BufReader};
use std::vec::Vec;

fn read_lines() -> Vec<String> {
    BufReader::new(stdin())
        .lines()
        .flatten()
        .collect::<Vec<String>>()
}

#[derive(Debug, Clone, Copy)]
struct Elements {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl Elements {
    fn new(ore: i32, clay: i32, obsidian: i32, geode: i32) -> Elements {
        Elements {
            ore: ore,
            clay: clay,
            obsidian: obsidian,
            geode: geode,
        }
    }

    fn add(&self, other: &Elements) -> Elements {
        Elements {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }

    fn sub(&self, other: &Elements) -> Elements {
        Elements {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }

    fn max(&self, other: &Elements) -> Elements {
        Elements {
            ore: self.ore.max(other.ore),
            clay: self.clay.max(other.clay),
            obsidian: self.obsidian.max(other.obsidian),
            geode: self.geode.max(other.geode),
        }
    }

    fn min(&self, other: &Elements) -> Elements {
        Elements {
            ore: self.ore.min(other.ore),
            clay: self.clay.min(other.clay),
            obsidian: self.obsidian.min(other.obsidian),
            geode: self.geode.min(other.geode),
        }
    }

    fn mul(&self, other: i32) -> Elements {
        Elements {
            ore: self.ore * other,
            clay: self.clay * other,
            obsidian: self.obsidian * other,
            geode: self.geode * other,
        }
    }

    fn ge(&self, other: &Elements) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }

    fn valid(&self) -> bool {
        self.ore >= 0 && self.clay >= 0 && self.obsidian >= 0 && self.geode >= 0
    }

    fn to_tuple(&self) -> (i32, i32, i32, i32) {
        (self.ore, self.clay, self.obsidian, self.geode)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Phase {
    BuildRobot,
    CollectResources,
}

#[derive(Debug, Clone)]
struct Blueprint {
    index: i32,
    ore: Elements,
    clay: Elements,
    obsidian: Elements,
    geode: Elements,
    search_space: i64,
    visited: HashMap<(i32, (i32, i32, i32, i32), (i32, i32, i32, i32)), i32>,
}

impl Blueprint {
    fn parse(s: &str) -> Blueprint {
        let tokens = s.split(' ').collect::<Vec<_>>();
        let index = tokens[1].split(':').nth(0).unwrap().parse::<i32>().unwrap();
        let ore = tokens[6].parse::<i32>().unwrap();
        let clay = tokens[12].parse::<i32>().unwrap();
        let obsidian = (
            tokens[18].parse::<i32>().unwrap(),
            tokens[21].parse::<i32>().unwrap(),
        );
        let geode = (
            tokens[27].parse::<i32>().unwrap(),
            tokens[30].parse::<i32>().unwrap(),
        );
        Blueprint {
            index: index,
            ore: Elements::new(ore, 0, 0, 0),
            clay: Elements::new(clay, 0, 0, 0),
            obsidian: Elements::new(obsidian.0, obsidian.1, 0, 0),
            geode: Elements::new(geode.0, 0, geode.1, 0),
            search_space: 0,
            visited: HashMap::new(),
        }
    }

    fn find_max_geode(&mut self, t: i32) -> i32 {
        self.visited.clear();
        self.search_space = 0;

        let limits = self
            .ore
            .max(&self.clay)
            .max(&self.obsidian)
            .max(&self.geode);

        self.dfs(
            1,
            t,
            Phase::CollectResources,
            &Elements::new(0, 0, 0, 0),
            &Elements::new(1, 0, 0, 0),
            &Elements::new(0, 0, 0, 0),
            &limits,
        )
    }

    fn dfs(
        &mut self,
        t: i32,
        target_t: i32,
        phase: Phase,
        resources: &Elements,
        robots: &Elements,
        pending_robots: &Elements,
        limits: &Elements,
    ) -> i32 {
        // if minute_phase == MinutePhase::CollectResources { resources, robots, pending_robots);
        //     println!("\tvisited.len() = {}", visited.len());
        // }
        assert!(t <= target_t);
        if !resources.valid() {
            panic!()
        }

        match phase {
            Phase::CollectResources => {
                let new_resources = resources.add(robots);
                let new_robots = robots.add(pending_robots);

                // // geode resources are calculated in advance.
                // let geode = pending_robots.geode * (target_t - t);

                if t == target_t {
                    // if new_resources.geode == 2 { new_resources, new_robots);
                    //     println!("\tvisited.len() = {}", visited.len());
                    // }
                    self.search_space += 1;
                    if self.search_space % 100000 == 0 {
                        println!(
                            "Blueprint({})::dfs()\n\tsearch space = {}\n\tvisited.len() = {}",
                            self.index,
                            self.search_space,
                            self.visited.len()
                        );
                        println!(
                            "\tresources = {:?}\n\trobots = {:?}",
                            new_resources, new_robots
                        );
                    }
                    new_resources.geode
                } else {
                    // Clamp, except geode.
                    let mut current_limits = limits.mul(target_t - t);
                    current_limits.geode = i32::MAX;
                    let new_resources = new_resources.min(&current_limits);

                    let key = (t, new_resources.to_tuple(), new_robots.to_tuple());
                    let existing = self.visited.get(&key);
                    match existing {
                        None => {
                            let best = self.dfs(
                                t + 1,
                                target_t,
                                Phase::BuildRobot,
                                &new_resources,
                                &new_robots,
                                &Elements::new(0, 0, 0, 0),
                                limits,
                            );

                            self.visited.insert(key, best);

                            best
                        }
                        Some(&value) => value,
                    }
                }
            }
            Phase::BuildRobot => {
                let build_ore = if resources.ge(&self.ore) {
                    self.dfs(
                        t,
                        target_t,
                        Phase::CollectResources,
                        &resources.sub(&self.ore),
                        robots,
                        &Elements::new(1, 0, 0, 0),
                        limits,
                    )
                } else {
                    -1
                };
                let build_clay = if resources.ge(&self.clay) {
                    self.dfs(
                        t,
                        target_t,
                        Phase::CollectResources,
                        &resources.sub(&self.clay),
                        robots,
                        &Elements::new(0, 1, 0, 0),
                        limits,
                    )
                } else {
                    -1
                };
                let build_obsidian = if resources.ge(&self.obsidian) {
                    self.dfs(
                        t,
                        target_t,
                        Phase::CollectResources,
                        &resources.sub(&self.obsidian),
                        robots,
                        &Elements::new(0, 0, 1, 0),
                        limits,
                    )
                } else {
                    -1
                };
                let build_geode = if resources.ge(&self.geode) {
                    self.dfs(
                        t,
                        target_t,
                        Phase::CollectResources,
                        &resources.sub(&self.geode),
                        robots,
                        &Elements::new(0, 0, 0, 1),
                        limits,
                    )
                } else {
                    -1
                };

                let build_nothing = if build_ore == -1
                    || build_clay == -1
                    || build_obsidian == -1
                    || build_geode == -1
                {
                    self.dfs(
                        t,
                        target_t,
                        Phase::CollectResources,
                        resources,
                        robots,
                        pending_robots,
                        limits,
                    )
                } else {
                    -1
                };

                *[
                    build_ore,
                    build_clay,
                    build_obsidian,
                    build_geode,
                    build_nothing,
                ]
                .iter()
                .max()
                .unwrap()
            }
        }
    }
}

fn main() {
    let blueprints = read_lines()
        .iter()
        .map(|line| Blueprint::parse(line))
        .collect::<Vec<_>>();

    // println!("{:?}", blueprints);

    let ans1 = blueprints
        .iter()
        .map(|blueprint| {
            let mut blueprint = blueprint.clone();
            let best = blueprint.find_max_geode(24);
            println!(
                "Blueprint({}):\n\tsearch space = {}\n\tvisited.len() = {}",
                blueprint.index,
                blueprint.search_space,
                blueprint.visited.len()
            );
            println!("best {} = {}", blueprint.index, best);
            blueprint.index * best
        })
        .sum::<i32>();

    println!("{}", ans1);

    let ans2 = blueprints[..3.min(blueprints.len())]
        .iter()
        .map(|blueprint| {
            let mut blueprint = blueprint.clone();
            let best = blueprint.find_max_geode(32);
            println!(
                "Blueprint({}):\n\tsearch space = {}\n\tvisited.len() = {}",
                blueprint.index,
                blueprint.search_space,
                blueprint.visited.len()
            );
            println!("best {} = {}", blueprint.index, best);
            best
        })
        .product::<i32>();

    println!("{}", ans2);
}
