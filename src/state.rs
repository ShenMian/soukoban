use std::{
    collections::HashSet,
    hash::{DefaultHasher, Hash, Hasher},
};

use nalgebra::Vector2;

use crate::{
    deadlock::is_freeze_deadlock,
    direction::Direction,
    path_finding::{normalized_area, reachable_area},
    solver::Solver,
    Map, Tiles,
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct State {
    pub player_position: Vector2<i32>,
    pub box_positions: HashSet<Vector2<i32>>,
}

impl State {
    /// Returns the successors of the state.
    pub fn successors(&self, solver: &Solver) -> Vec<State> {
        let mut successors = Vec::new();
        let player_reachable_area = reachable_area(self.player_position, |position| {
            !solver.map()[position].intersects(Tiles::Wall)
                && !self.box_positions.contains(&position)
        });
        // Creates successor states by pushing boxes
        for box_position in &self.box_positions {
            for direction in Direction::iter() {
                let mut new_box_position = box_position + &direction.into();
                if solver.map()[new_box_position].intersects(Tiles::Wall)
                    || self.box_positions.contains(&new_box_position)
                    || !solver.lower_bounds().contains_key(&new_box_position)
                {
                    continue;
                }
                let new_player_position = box_position - &direction.into();
                if !player_reachable_area.contains(&new_player_position) {
                    continue;
                }

                // Skip no influence pushes
                while solver.tunnels().contains(&(new_box_position, direction)) {
                    new_box_position += &direction.into();
                }

                let mut new_box_positions = self.box_positions.clone();
                new_box_positions.remove(box_position);
                new_box_positions.insert(new_box_position);

                // Skip freeze deadlocks
                if !solver.map()[new_box_position].intersects(Tiles::Goal)
                    && is_freeze_deadlock(
                        solver.map(),
                        new_box_position,
                        &new_box_positions,
                        &mut HashSet::new(),
                    )
                {
                    continue;
                }

                successors.push(State {
                    player_position: new_player_position,
                    box_positions: new_box_positions,
                });
            }
        }
        successors
    }

    /// Returns true if the state is solved.
    pub fn is_solved(&self, solver: &Solver) -> bool {
        self.box_positions == *solver.map().goal_positions()
    }

    /// Returns the heuristic value of the state.
    pub fn heuristic(&self, solver: &Solver) -> i32 {
        self.box_positions
            .iter()
            .map(|box_position| solver.lower_bounds()[box_position])
            .sum()
    }

    /// Returns the hash of the normalized state.
    pub fn normalized_hash(&self, map: &Map) -> u64 {
        let mut normalized_state = self.clone();
        normalized_state.player_position =
            normalized_area(&reachable_area(self.player_position, |position| {
                !(map[position].intersects(Tiles::Wall) || self.box_positions.contains(&position))
            }))
            .unwrap();
        let mut hasher = DefaultHasher::new();
        normalized_state.hash(&mut hasher);
        hasher.finish()
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.player_position.hash(state);
        for box_position in &self.box_positions {
            box_position.hash(state);
        }
    }
}

impl From<Map> for State {
    fn from(map: Map) -> Self {
        Self {
            player_position: map.player_position(),
            box_positions: map.box_positions().clone(),
        }
    }
}
