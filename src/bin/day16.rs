use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
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

        rule targets() -> Vec<RoomID>
            = ("tunnels"/"tunnel") ws() ("leads"/"lead") ws()
              "to" ws() ("valves"/"valve") ws()
              v:(valve_id() ** ", ") { v.into_iter().collect() }

        rule room() -> (RoomID, Room)
            = "Valve" ws() id:valve_id() ws()
               flow_rate:flow_rate() ws()
               tunnels:targets() { (id, Room { flow_rate, tunnels }) }

        pub rule input() -> Cave
            = v:(room() ** "\n") { Cave::new(v) }
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
    tunnels: Vec<RoomID>,
}

#[derive(Debug)]
pub struct Cave {
    tunnels: BTreeMap<RoomID, Vec<RoomID>>,
    flow_rates: BTreeMap<RoomID, i64>,
}

impl Cave {
    fn new(rooms: Vec<(RoomID, Room)>) -> Cave {
        let flow_rates = rooms
            .iter()
            .filter_map(|(room_id, room)| {
                if room.flow_rate > 0 {
                    Some((*room_id, room.flow_rate))
                } else {
                    None
                }
            })
            .collect();
        let tunnels = rooms
            .into_iter()
            .map(|(room_id, room)| (room_id, room.tunnels))
            .collect();
        Cave {
            tunnels,
            flow_rates,
        }
    }

    fn get_flow_rate(&self, room_id: &RoomID) -> i64 {
        *self
            .flow_rates
            .get(room_id)
            .unwrap_or_else(|| panic!("No flow rate for {room_id}"))
    }

    fn get_tunnels(&self, room_id: &RoomID) -> &Vec<RoomID> {
        self.tunnels
            .get(room_id)
            .unwrap_or_else(|| panic!("No tunnels for {room_id}"))
    }

    fn unopened_valves(&self) -> Vec<RoomID> {
        let mut rooms: Vec<(i64, RoomID)> = self
            .flow_rates
            .iter()
            .map(|(room_id, flow_rate)| (*flow_rate, *room_id))
            .collect();
        rooms.sort();
        rooms.reverse();
        rooms.into_iter().map(|(_, room_id)| room_id).collect()
    }
}

#[derive(Debug, Copy, Clone)]
struct TurnState {
    remaining_turns: i64,
    pressure_released: i64,
    total_flow_rate: i64,
    best_score_at_end: i64,
}

impl TurnState {
    fn initial(remaining_turns: i64, cave: &Cave, player_count: usize) -> Self {
        TurnState::new(
            remaining_turns,
            0,
            0,
            cave,
            &cave.unopened_valves(),
            player_count,
        )
    }

    fn new(
        remaining_turns: i64,
        pressure_released: i64,
        total_flow_rate: i64,
        cave: &Cave,
        unopened_valves: &[RoomID],
        player_count: usize,
    ) -> Self {
        let mut best_score_at_end = pressure_released;
        let mut current_flow_rate = total_flow_rate;
        let mut current_remaining_turns = remaining_turns;

        let flow_rate_adjustments = unopened_valves.chunks(player_count).map(|chunk| {
            chunk
                .iter()
                .map(|room| cave.get_flow_rate(room))
                .sum::<i64>()
        });

        for adjustment in flow_rate_adjustments {
            if current_remaining_turns == 0 {
                break;
            }

            // Open
            current_remaining_turns -= 1;
            best_score_at_end += current_flow_rate;
            current_flow_rate += adjustment;
        }
        best_score_at_end += current_remaining_turns * current_flow_rate;

        TurnState {
            remaining_turns,
            pressure_released,
            total_flow_rate,
            best_score_at_end,
        }
    }

    fn score_at_end(&self) -> i64 {
        self.pressure_released + self.total_flow_rate * self.remaining_turns
    }

    fn worse_than(&self, other: &Self) -> bool {
        if self.best_score_at_end < other.best_score_at_end {
            return true;
        }

        if self.pressure_released <= other.pressure_released {
            self.remaining_turns <= other.remaining_turns
        } else {
            false
        }
    }

    fn advance(&self, cave: &Cave, unopened_valves: &[RoomID], player_count: usize) -> Self {
        if self.remaining_turns < 1 {
            panic!("Cannot advance past end");
        }
        TurnState::new(
            self.remaining_turns - 1,
            self.pressure_released + self.total_flow_rate,
            self.total_flow_rate,
            cave,
            unopened_valves,
            player_count,
        )
    }

    fn opening_valve(
        &self,
        flow_rate: i64,
        cave: &Cave,
        unopened_valves: &[RoomID],
        player_count: usize,
    ) -> Self {
        if self.remaining_turns < 1 {
            panic!("Cannot advance past end");
        }
        TurnState::new(
            self.remaining_turns - 1,
            self.pressure_released + self.total_flow_rate,
            self.total_flow_rate + flow_rate,
            cave,
            unopened_valves,
            player_count,
        )
    }
}

trait CaveState: Sized + Debug + PartialEq + Eq + PartialOrd + Ord + Hash + Clone {
    fn next_states(&self, cave: &Cave, turn_state: &TurnState) -> Vec<(TurnState, Self)>;
    fn player_count(&self) -> usize;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct CaveStateSimple {
    current_room: RoomID,
    unopened_valves: Vec<RoomID>,
}

impl CaveStateSimple {
    fn moving_to_room(&self, room: RoomID) -> Self {
        CaveStateSimple {
            current_room: room,
            unopened_valves: self.unopened_valves.clone(),
        }
    }

    fn opening_valve(&self) -> Self {
        let mut valves = self.unopened_valves.clone();
        valves.retain(|r| *r != self.current_room);

        CaveStateSimple {
            current_room: self.current_room,
            unopened_valves: valves,
        }
    }
}

impl CaveState for CaveStateSimple {
    fn next_states(&self, cave: &Cave, turn_state: &TurnState) -> Vec<(TurnState, Self)> {
        let mut result = vec![];

        if turn_state.remaining_turns <= 1 {
            return result;
        }

        if self.unopened_valves.contains(&self.current_room) {
            let next_cave_state = self.opening_valve();
            let next_turn_state = turn_state.opening_valve(
                cave.get_flow_rate(&self.current_room),
                cave,
                &next_cave_state.unopened_valves,
                1,
            );
            result.push((next_turn_state, next_cave_state))
        }

        for next_room in cave.get_tunnels(&self.current_room).iter() {
            let next_cave_state = self.moving_to_room(*next_room);
            let next_turn_state = turn_state.advance(cave, &next_cave_state.unopened_valves, 1);
            result.push((next_turn_state, next_cave_state))
        }

        result
    }

    fn player_count(&self) -> usize {
        1
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct CaveStateElephant {
    player_room: RoomID,
    elephant_room: RoomID,
    unopened_valves: Vec<RoomID>,
}

impl CaveStateElephant {
    fn moving_player(&self, room: RoomID) -> Self {
        CaveStateElephant {
            player_room: self.elephant_room.min(room),
            elephant_room: self.elephant_room.max(room),
            unopened_valves: self.unopened_valves.clone(),
        }
    }

    fn moving_elephant(&self, room: RoomID) -> Self {
        CaveStateElephant {
            player_room: self.player_room.min(room),
            elephant_room: self.player_room.max(room),
            unopened_valves: self.unopened_valves.clone(),
        }
    }

    fn moving_both(&self, player_room: RoomID, elephant_room: RoomID) -> Self {
        CaveStateElephant {
            player_room: player_room.min(elephant_room),
            elephant_room: player_room.max(elephant_room),
            unopened_valves: self.unopened_valves.clone(),
        }
    }

    fn opening_valve(&self, room: RoomID) -> Self {
        let mut valves = self.unopened_valves.clone();
        valves.retain(|r| *r != room);

        CaveStateElephant {
            player_room: self.player_room,
            elephant_room: self.elephant_room,
            unopened_valves: valves,
        }
    }
}

impl CaveState for CaveStateElephant {
    fn next_states(&self, cave: &Cave, turn_state: &TurnState) -> Vec<(TurnState, Self)> {
        let mut result = vec![];

        if turn_state.remaining_turns <= 1 {
            return result;
        }

        // Both open
        if self.player_room != self.elephant_room
            && self.unopened_valves.contains(&self.player_room)
            && self.unopened_valves.contains(&self.elephant_room)
        {
            let next_cave_state = self
                .opening_valve(self.player_room)
                .opening_valve(self.elephant_room);
            let next_turn_state = turn_state.opening_valve(
                cave.get_flow_rate(&self.player_room) + cave.get_flow_rate(&self.elephant_room),
                cave,
                &next_cave_state.unopened_valves,
                2,
            );
            result.push((next_turn_state, next_cave_state))
        }

        // Player opens, elephant moves
        if self.unopened_valves.contains(&self.player_room) {
            for next_room in cave.get_tunnels(&self.elephant_room).iter() {
                let next_cave_state = self
                    .moving_elephant(*next_room)
                    .opening_valve(self.player_room);
                let next_turn_state = turn_state.opening_valve(
                    cave.get_flow_rate(&self.player_room),
                    cave,
                    &next_cave_state.unopened_valves,
                    2,
                );
                result.push((next_turn_state, next_cave_state))
            }
        }

        // Elephant opens, player moves
        if self.unopened_valves.contains(&self.elephant_room) {
            for next_room in cave.get_tunnels(&self.player_room).iter() {
                let next_cave_state = self
                    .moving_player(*next_room)
                    .opening_valve(self.elephant_room);
                let next_turn_state = turn_state.opening_valve(
                    cave.get_flow_rate(&self.elephant_room),
                    cave,
                    &next_cave_state.unopened_valves,
                    2,
                );
                result.push((next_turn_state, next_cave_state))
            }
        }

        // Both move
        for next_player_room in cave.get_tunnels(&self.player_room).iter() {
            for next_elephant_room in cave.get_tunnels(&self.elephant_room).iter() {
                let next_cave_state = self.moving_both(*next_player_room, *next_elephant_room);
                let next_turn_state = turn_state.advance(cave, &next_cave_state.unopened_valves, 2);
                result.push((next_turn_state, next_cave_state))
            }
        }

        result
    }

    fn player_count(&self) -> usize {
        2
    }
}

fn parse_input(input: &str) -> Cave {
    parse_peg(input, input_parser::input)
}

fn remove_matching<T, F: Fn(&T) -> bool>(from: &mut Vec<T>, predicate: F) {
    let mut i = 0;
    while i < from.len() {
        if predicate(&from[i]) {
            from.remove(i);
        } else {
            i += 1;
        }
    }
}

const DEBUG: bool = false;
const DEBUG_OCCASIONAL: bool = true;
const DEBUG_SKIPS: bool = false;

fn solve<T: CaveState>(
    input: &str,
    make_initial: fn(Vec<RoomID>) -> T,
    initial_remaining_time: i64,
) -> i64 {
    let cave = parse_input(input);
    let mut best_states: BTreeMap<T, Vec<TurnState>> = BTreeMap::new();
    let mut pending: BTreeSet<T> = BTreeSet::new();

    let initial_state = make_initial(cave.unopened_valves());
    best_states.insert(
        initial_state.clone(),
        vec![TurnState::initial(
            initial_remaining_time,
            &cave,
            initial_state.player_count(),
        )],
    );
    pending.insert(initial_state);

    let mut max_score: i64 = 0;
    let mut evaluated = 0;

    while let Some(cave_state) = pending.pop_first() {
        evaluated += 1;
        let debug = DEBUG || (DEBUG_OCCASIONAL && evaluated % 50000 == 0);

        let turn_states = best_states.get(&cave_state).unwrap_or(&vec![]).clone();
        if debug {
            println!(
                "Evaluating {cave_state:?} ({evaluated} evaluated, {} pending)",
                pending.len()
            );
        }

        for turn_state in turn_states {
            if debug {
                println!(" - Using {turn_state:?} ({})", turn_state.score_at_end());
            }

            for (next_turn_state, next_cave_state) in cave_state.next_states(&cave, &turn_state) {
                let mut existing_states = best_states
                    .get(&next_cave_state)
                    .cloned()
                    .unwrap_or_default();

                if let Some(better_state) = existing_states
                    .iter()
                    .find(|existing_state| next_turn_state.worse_than(existing_state))
                {
                    if debug && DEBUG_SKIPS {
                        println!("   - Next: {next_cave_state:?}");
                        println!("           {next_turn_state:?}");
                        println!("    [skip] {better_state:?}");
                    }
                    continue;
                }

                if next_turn_state.best_score_at_end < max_score {
                    continue;
                }
                max_score = max_score.max(next_turn_state.score_at_end());

                if debug {
                    println!("   - Next: {next_cave_state:?}");
                    println!("           {next_turn_state:?}");
                }

                remove_matching(&mut existing_states, |existing_state| {
                    existing_state.worse_than(&next_turn_state)
                });
                existing_states.push(next_turn_state);
                best_states.insert(next_cave_state.clone(), existing_states);
                pending.insert(next_cave_state);
            }
        }
    }
    println!("Evaluated {evaluated} states");

    max_score
}

fn part1(input: &str) -> i64 {
    solve(
        input,
        |valves| CaveStateSimple {
            current_room: RoomID::INITIAL,
            unopened_valves: valves,
        },
        30,
    )
}

fn part2(input: &str) -> i64 {
    solve(
        input,
        |valves| CaveStateElephant {
            player_room: RoomID::INITIAL,
            elephant_room: RoomID::INITIAL,
            unopened_valves: valves,
        },
        26,
    )
}

fn main() {
    aoc_main!(part1, part2);
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_INPUT), 1707);
    }
}
