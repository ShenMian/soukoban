use std::hash::{Hash, Hasher};

use nalgebra::Vector2;
use rustc_hash::FxHasher;

use crate::{
    FxHashSet, Map, Tiles,
    path_finding::{compute_area_anchor, compute_reachable_area},
    solver::{Strategy, context::Context},
};

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct State {
    pub player_position: Vector2<i32>,
    pub box_positions: FxHashSet<Vector2<i32>>,
}

impl State {
    /// Computes the lower bound on the minimum remaining cost to reach a final
    /// state.
    ///
    /// It sums, over all boxes, the minimum cost needed to bring that box to
    /// its nearest goal (ignoring other boxes).
    ///
    /// A value of 0 means every box is already on a goal, i.e. the state is a
    /// final state.
    pub fn lower_bound(&self, ctx: &Context) -> i32 {
        self.box_positions
            .iter()
            .map(|box_position| ctx.min_costs()[box_position])
            .sum()
    }

    /// Computes the hash of the canonicalized state.
    pub fn canonicalized_hash(&self, ctx: &Context) -> u64 {
        let mut hasher = FxHasher::default();
        match ctx.strategy() {
            Strategy::Quick | Strategy::PushOptimal => {
                let mut canonicalized_state = self.clone();
                canonicalized_state.canonicalize_player(ctx.map());
                canonicalized_state.hash(&mut hasher);
                hasher.finish()
            }
            Strategy::MoveOptimal => {
                self.hash(&mut hasher);
                hasher.finish()
            }
        }
    }

    /// Canonicalizes the position of the player.
    fn canonicalize_player(&mut self, map: &Map) {
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
