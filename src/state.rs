use std::{
    collections::HashSet,
    hash::{DefaultHasher, Hash, Hasher},
};

use nalgebra::Vector2;

use crate::{
    Map, Tiles,
    path_finding::{compute_area_anchor, compute_reachable_area},
    solver::Strategy,
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct State {
    pub player_position: Vector2<i32>,
    pub box_positions: HashSet<Vector2<i32>>,
}

impl State {
    /// Computes the hash of the state according to the given strategy.
    pub fn compute_hash(&self, strategy: Strategy, map: &Map) -> u64 {
        let mut hasher = DefaultHasher::new();
        match strategy {
            Strategy::Fast | Strategy::OptimalPush => {
                let mut normalized_state = self.clone();
                normalized_state.normalize_player_position(map);
                normalized_state.hash(&mut hasher);
                hasher.finish()
            }
            Strategy::OptimalMove => {
                self.hash(&mut hasher);
                hasher.finish()
            }
        }
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
