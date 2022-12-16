use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use utils::{aoc_main, parse_peg};

peg::parser! {
    grammar input_parser() for str {
        rule valve_id() -> RoomID
            = c1:['A'..='Z'] c2:['A'..='Z'] { RoomID(c1, c2) }

        rule ws() = " ";

        rule integer() -> i64
            = n:$("-"? ['0'..='9']+) { ? n.parse::<i64>().or(Err("integer")) }

        rule flow_rate() -> i64
            = "has flow rate=" v:integer() ";" { v }

        rule targets() -> HashSet<RoomID>
            = ("tunnels"/"tunnel") ws() ("leads"/"lead") ws()
              "to" ws() ("valves"/"valve") ws()
              v:(valve_id() ** ", ") { v.into_iter().collect() }

        rule room() -> (RoomID, Room)
            = "Valve" ws() id:valve_id() ws()
               flow_rate:flow_rate() ws()
               tunnels:targets() { (id, Room { flow_rate, tunnels }) }

        pub rule input() -> HashMap<RoomID, Room>
            = v:(room() ** "\n") { v.into_iter().collect() }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Ord, PartialOrd)]
pub struct RoomID(char, char);

impl RoomID {
    const INITIAL: RoomID = RoomID('A', 'A');
}

impl Debug for RoomID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for RoomID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

#[derive(Debug)]
pub struct Room {
    flow_rate: i64,
    tunnels: HashSet<RoomID>,
}

type Cave = HashMap<RoomID, Room>;

#[derive(Debug, Copy, Clone)]
struct TurnState {
    remaining_turns: i64,
    pressure_released: i64,
    total_flow_rate: i64,
}

impl TurnState {
    fn score_at_end(&self) -> i64 {
        self.pressure_released + self.total_flow_rate * self.remaining_turns
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct CaveState {
    current_room: RoomID,
    pending_valves: Vec<RoomID>,
}

fn parse_input(input: &str) -> Cave {
    parse_peg(input, input_parser::input)
}

fn pop(states: &mut HashSet<CaveState>) -> Option<CaveState> {
    let result = states.iter().next().cloned();
    for item in &result {
        states.remove(item);
    }
    result
}

fn next_states(
    cave: &Cave,
    turn_state: &TurnState,
    cave_state: &CaveState,
) -> Vec<(TurnState, CaveState)> {
    let mut result = vec![];

    if turn_state.remaining_turns <= 1 {
        return result;
    }

    let current_room = cave.get(&cave_state.current_room).unwrap();

    if cave_state.pending_valves.contains(&cave_state.current_room) {
        let next_turn_state = TurnState {
            remaining_turns: turn_state.remaining_turns - 1,
            total_flow_rate: turn_state.total_flow_rate + current_room.flow_rate,
            pressure_released: turn_state.pressure_released + turn_state.total_flow_rate,
        };
        let next_cave_state = CaveState {
            current_room: cave_state.current_room,
            pending_valves: cave_state
                .pending_valves
                .iter()
                .copied()
                .filter(|v| *v != cave_state.current_room)
                .collect(),
        };
        result.push((next_turn_state, next_cave_state))
    }

    for next_room in current_room.tunnels.iter() {
        let next_turn_state = TurnState {
            remaining_turns: turn_state.remaining_turns - 1,
            pressure_released: turn_state.pressure_released + turn_state.total_flow_rate,
            total_flow_rate: turn_state.total_flow_rate,
        };
        let next_cave_state = CaveState {
            current_room: *next_room,
            pending_valves: cave_state.pending_valves.clone(),
        };
        result.push((next_turn_state, next_cave_state))
    }

    result
}

fn solve_part1(input: &str, debug: bool) -> i64 {
    let cave = parse_input(input);
    let mut best_states: HashMap<CaveState, Vec<TurnState>> = HashMap::new();
    let mut pending: HashSet<CaveState> = HashSet::new();
    println!("{:?}", cave);
    println!();

    let initial_state = CaveState {
        current_room: RoomID::INITIAL,
        pending_valves: cave
            .iter()
            .filter_map(
                |(id, room)| {
                    if room.flow_rate > 0 {
                        Some(*id)
                    } else {
                        None
                    }
                },
            )
            .collect(),
    };
    best_states.insert(
        initial_state.clone(),
        vec![TurnState {
            remaining_turns: 30,
            pressure_released: 0,
            total_flow_rate: 0,
        }],
    );
    pending.insert(initial_state);

    let mut max_score: i64 = 0;

    while let Some(cave_state) = pop(&mut pending) {
        let turn_states = best_states.get(&cave_state).unwrap_or(&vec![]).clone();
        if debug {
            println!("Evaluating {cave_state:?}");
        }

        for turn_state in turn_states {
            if debug {
                println!(" - Using {turn_state:?} ({})", turn_state.score_at_end());
            }
            max_score = max_score.max(turn_state.score_at_end());

            for (next_turn_state, next_cave_state) in next_states(&cave, &turn_state, &cave_state) {
                if debug {
                    println!("   - Next: {next_cave_state:?}");
                    println!("           {next_turn_state:?}");
                }

                let mut existing_states = best_states
                    .get(&next_cave_state)
                    .cloned()
                    .unwrap_or_default();

                if let Some(better_state) = existing_states.iter().find(|existing_state| {
                    if next_turn_state.pressure_released <= existing_state.pressure_released {
                        next_turn_state.remaining_turns <= existing_state.remaining_turns
                    } else {
                        false
                    }
                }) {
                    if debug {
                        println!("    [skip] {:?}", better_state);
                    }
                    continue;
                }

                existing_states.push(next_turn_state);
                best_states.insert(next_cave_state.clone(), existing_states);
                pending.insert(next_cave_state);
            }
        }
    }

    max_score
}

fn part1(input: &str) -> i64 {
    solve_part1(input, false)
}

fn part2(input: &str) -> u32 {
    unimplemented!();
}

fn main() {
    aoc_main!(part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 1651);
    }

    //#[test]
    //fn test_part2() {
    //    assert_eq!(part2(EXAMPLE_INPUT), 0);
    //}
}
