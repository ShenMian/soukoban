use std::{
    collections::HashSet,
    hash::{DefaultHasher, Hash, Hasher},
};

use nalgebra::Vector2;

use crate::{
    path_finding::{compute_area_anchor, compute_reachable_area},
    solver::Solver,
    Map, Tiles,
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct State {
    pub player_position: Vector2<i32>,
    pub box_positions: HashSet<Vector2<i32>>,
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

    /// Returns the hash of the normalized state.
    pub fn normalized_hash(&self, map: &Map) -> u64 {
        let mut normalized_state = self.clone();
        normalized_state.normalize_player_position(map);
        let mut hasher = DefaultHasher::new();
        normalized_state.hash(&mut hasher);
        hasher.finish()
    }

    /// Normalizes the position of the player.
    fn normalize_player_position(&mut self, map: &Map) {
        let player_reachable_area = compute_reachable_area(self.player_position, |position| {
            !(map[position].intersects(Tiles::Wall) || self.box_positions.contains(&position))
        });
        self.player_position = compute_area_anchor(&player_reachable_area).unwrap();
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.player_position.hash(state);
        let mut sorted_boxes: Vec<_> = self.box_positions.iter().collect();
        sorted_boxes.sort_by_key(|position| (position.x, position.y));
        for box_position in sorted_boxes {
            box_position.hash(state);
        }
    }
}
