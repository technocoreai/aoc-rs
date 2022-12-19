use enum_map::{enum_map, Enum, EnumMap};
use rayon::prelude::*;
use std::collections::BTreeMap;
use utils::{aoc_main, parse_peg};

peg::parser! {
    grammar input_parser() for str {
        rule integer() -> u64
            = n:$("-"? ['0'..='9']+) { ? n.parse::<u64>().or(Err("integer")) }

        rule resource() -> Resource
            = "ore" { Resource::Ore }
            / "clay" { Resource::Clay }
            / "obsidian" { Resource::Obsidian }
            / "geode" { Resource::Geode }

        rule cost() -> (Resource, u64)
            = cost:integer() " " resource:resource() { (resource, cost) }

        rule resources() -> ResourceAmounts
            = v:(cost() ** " and ") { v.into_iter().collect() }

        rule robot_costs() -> (Resource, ResourceAmounts)
            = "Each " kind:resource() " robot costs " costs:resources() "." { (kind, costs) }

        rule blueprint() -> Blueprint
            = "Blueprint " id:integer() ": " robot_costs:(robot_costs() ** " ") {
            Blueprint {
                id,
                robot_costs: robot_costs.into_iter().collect()
            }
        }

        pub rule input() -> Vec<Blueprint> =
            v:(blueprint() ** "\n") { v }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Enum, Hash)]
pub enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

type ResourceAmounts = EnumMap<Resource, u64>;

#[derive(Debug)]
pub struct Blueprint {
    id: u64,
    robot_costs: EnumMap<Resource, ResourceAmounts>,
}

fn stockpile_duration_single(current_amount: u64, robots: u64, cost: u64) -> Option<u64> {
    if robots == 0 {
        return None;
    }
    if current_amount >= cost {
        return Some(0);
    }

    let remainder = cost - current_amount;
    let turns = remainder / robots;
    if robots * turns == remainder {
        Some(turns)
    } else {
        Some(turns + 1)
    }
}

fn stockpile_duration(
    current_resources: &ResourceAmounts,
    current_robots: &ResourceAmounts,
    costs: &ResourceAmounts,
) -> Option<u64> {
    let mut result = 0;
    for (resource, cost) in costs {
        if *cost == 0 {
            continue;
        }
        let duration_for_resource =
            stockpile_duration_single(current_resources[resource], current_robots[resource], *cost);
        result = result.max(duration_for_resource?);
    }
    Some(result)
}

fn simulate(
    max_geode_robots: &mut BTreeMap<u64, u64>,
    blueprint: &Blueprint,
    max_turns: u64,
    current_turn: u64,
    current_resources: ResourceAmounts,
    current_robots: ResourceAmounts,
) -> (u64, u64) {
    let remaining_turns = max_turns - current_turn;

    if current_turn <= 5 {
        let indent = current_turn as usize;
        println!(
            "{:>indent$}[{}/{remaining_turns}]: resources: {current_resources:?}, robots: {current_robots:?}",
            " ", blueprint.id,
        );
    }

    if let Some(better_score) = max_geode_robots.get(&current_turn) {
        if *better_score > current_robots[Resource::Geode] {
            return (0, 0);
        }
    }
    max_geode_robots.insert(current_turn, current_robots[Resource::Geode]);

    let mut result =
        current_resources[Resource::Geode] + current_robots[Resource::Geode] * remaining_turns;
    let mut states = 1;

    if current_turn < max_turns {
        for (resource, costs) in &blueprint.robot_costs {
            if let Some(stockpile_turns) =
                stockpile_duration(&current_resources, &current_robots, costs)
            {
                let step_time = stockpile_turns + 1;

                if step_time > remaining_turns {
                    continue;
                }

                let mut updated_robots = current_robots;
                updated_robots[resource] += 1;

                let mut updated_resources = current_resources;
                for (resource, robots) in current_robots {
                    updated_resources[resource] += robots * step_time;
                    updated_resources[resource] -= costs[resource];
                }

                let (build_result, build_states) = simulate(
                    max_geode_robots,
                    blueprint,
                    max_turns,
                    current_turn + step_time,
                    updated_resources,
                    updated_robots,
                );
                result = result.max(build_result);
                states += build_states;
            }
        }
    }
    (result, states)
}

fn max_geode_count(blueprint: &Blueprint, max_turns: u64) -> u64 {
    let (geode_count, states) = simulate(
        &mut BTreeMap::new(),
        blueprint,
        max_turns,
        0,
        enum_map! {_ => 0},
        enum_map! {Resource::Ore => 1, _ => 0},
    );
    println!(
        "Done with {}: {geode_count}, seen {states} states",
        blueprint.id
    );
    geode_count
}

fn part1(input: &str) -> u64 {
    parse_peg(input, input_parser::input)
        .par_iter()
        .map(|blueprint| max_geode_count(blueprint, 24) * blueprint.id)
        .sum()
}

fn part2(input: &str) -> u64 {
    parse_peg(input, input_parser::input)
        .par_iter()
        .take(3)
        .map(|blueprint| max_geode_count(blueprint, 32))
        .product()
}

fn main() {
    aoc_main!(part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 33);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 56 * 62);
    }
}
