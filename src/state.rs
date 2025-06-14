use std::{
    collections::HashSet,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::point::Point;
use crate::{
    path_finding::{normalized_area, reachable_area},
    solver::Solver,
    Map, Tiles,
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct State {
    pub player_position: Point,
    pub box_positions: HashSet<Point>,
}

impl State {
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

    /// Normalizes the state.
    pub fn normalize(&mut self, map: &Map) {
        self.player_position = normalized_area(&reachable_area(self.player_position, |position| {
            !(map[position].intersects(Tiles::Wall) || self.box_positions.contains(&position))
        }))
        .unwrap();
    }

    /// Returns the hash of the normalized state.
    pub fn normalized_hash(&self, map: &Map) -> u64 {
        let mut normalized_state = self.clone();
        normalized_state.normalize(map);
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
