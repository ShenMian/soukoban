use std::{cmp::Ordering, collections::HashSet};

use crate::{
    deadlock::is_freeze_deadlock,
    direction::Direction,
    path_finding::{find_path, reachable_area},
    solver::{Solver, Strategy},
    state::State,
    Tiles,
};

/// A node in the search tree.
#[derive(Clone, Eq, Debug)]
pub struct Node {
    pub state: State,
    pub pushes: i32,
    pub moves: i32,
    priority: i32,
}

impl Node {
    /// Creates a new `Node`.
    pub fn new(state: State, pushes: i32, moves: i32, solver: &Solver) -> Self {
        let heuristic = state.heuristic(solver);
        let priority = match solver.strategy() {
            Strategy::Fast => heuristic,
            Strategy::OptimalPush => pushes + heuristic,
            Strategy::OptimalMove => moves + heuristic,
        };
        Self {
            state,
            pushes,
            moves,
            priority,
        }
    }

    /// Returns the successors of the node.
    pub fn successors(&self, solver: &Solver) -> Vec<Node> {
        let mut successors = Vec::new();
        let player_reachable_area = reachable_area(self.state.player_position, |position| {
            !solver.map()[position].intersects(Tiles::Wall)
                && !self.state.box_positions.contains(&position)
        });
        // Creates successor states by pushing boxes
        for box_position in &self.state.box_positions {
            for push_direction in Direction::iter() {
                let mut new_box_position = box_position + &push_direction.into();
                if solver.map()[new_box_position].intersects(Tiles::Wall)
                    || self.state.box_positions.contains(&new_box_position)
                    || !solver.lower_bounds().contains_key(&new_box_position)
                {
                    continue;
                }
                let new_player_position = box_position - &push_direction.into();
                if !player_reachable_area.contains(&new_player_position) {
                    continue;
                }
                let mut new_player_position = box_position;

                let mut new_pushes = self.pushes + 1;
                let mut new_moves = self.moves
                    + find_path(
                        self.state.player_position,
                        *new_player_position,
                        |position| {
                            !solver.map()[position].intersects(Tiles::Wall)
                                && (!self.state.box_positions.contains(&position)
                                    || position == *box_position)
                        },
                    )
                    .unwrap()
                    .len() as i32
                    - 1;

                // Skip no influence pushes
                // while solver
                //     .tunnels()
                //     .contains(&(new_box_position, push_direction))
                // {
                //     new_player_position = &new_box_position;
                //     new_box_position += push_direction.into();
                //     new_pushes += 1;
                //     new_moves += 1;
                // }

                let mut new_box_positions = self.state.box_positions.clone();
                new_box_positions.remove(&box_position);
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

                successors.push(Node::new(
                    State {
                        player_position: *new_player_position,
                        box_positions: new_box_positions,
                    },
                    new_pushes,
                    new_moves,
                    solver,
                ));
            }
        }
        successors
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
