use std::{cmp::Ordering, collections::HashSet};

use crate::{
    Tiles,
    deadlock::is_freeze_deadlock,
    direction::Direction,
    path_finding::{compute_reachable_area, find_path},
    solver::{Solver, Strategy},
    state::State,
};

/// A node in the search tree.
#[derive(Clone, Eq, Debug)]
pub struct Node {
    pub state: State,
    pub pushes: i32,
    pub moves: i32,
    heuristic: i32,
    strategy: Strategy,
}

impl Node {
    /// Creates a new `Node`.
    pub fn new(state: State, pushes: i32, moves: i32, solver: &Solver) -> Self {
        Self {
            heuristic: solver.heuristic(&state),
            state,
            pushes,
            moves,
            strategy: solver.strategy(),
        }
    }

    /// Returns true if the state is solved.
    pub fn is_solved(&self) -> bool {
        self.heuristic == 0
    }

    /// Returns the actual cost from the start node to this node (g-value).
    pub fn cost(&self) -> i32 {
        match self.strategy {
            Strategy::FastPush | Strategy::FastMove => 0,
            Strategy::OptimalPush => self.pushes,
            Strategy::OptimalMove => self.moves,
        }
    }

    /// Returns the heuristic estimated cost from this node to the goal
    /// (h-value).
    pub fn estimated_cost(&self) -> i32 {
        self.heuristic
    }

    /// Returns the estimated total cost from start to goal through this node
    /// (f-value).
    ///
    /// This is the sum of the actual cost and the heuristic cost: f(n) = g(n) +
    /// h(n).
    pub fn estimated_total_cost(&self) -> i32 {
        self.cost() + self.estimated_cost()
    }

    /// Returns the successors of the node.
    pub fn successors(&self, solver: &Solver) -> Vec<Node> {
        let mut successors = Vec::new();
        let player_reachable_area =
            compute_reachable_area(self.state.player_position, |position| {
                !solver.map()[position].intersects(Tiles::Wall)
                    && !self.state.box_positions.contains(&position)
            });
        // Creates successor states by pushing boxes
        for box_position in &self.state.box_positions {
            for push_direction in Direction::iter() {
                let mut new_box_position = box_position + &push_direction.into();

                // Checks if the box can be pushed
                if solver.map()[new_box_position].intersects(Tiles::Wall)
                    || self.state.box_positions.contains(&new_box_position)
                    || !solver.lower_bounds().contains_key(&new_box_position)
                {
                    continue;
                }

                // Checks if the player can push the box
                if !player_reachable_area.contains(&(box_position - &push_direction.into())) {
                    continue;
                }

                let mut new_player_position = *box_position;

                let push_from_position = *box_position - &push_direction.into();
                let path_to_push_position = find_path(
                    self.state.player_position,
                    push_from_position,
                    |position| {
                        !solver.map()[position].intersects(Tiles::Wall)
                            && !self.state.box_positions.contains(&position)
                    },
                )
                .unwrap();

                // Number of moves to get to the position to push, plus 1 for the push itself.
                let mut new_moves = self.moves + (path_to_push_position.len() - 1) as i32 + 1;
                let mut new_pushes = self.pushes + 1;

                // Skip pushes in tunnels
                while solver
                    .tunnels()
                    .contains(&(new_box_position, push_direction))
                {
                    new_player_position = new_box_position;
                    new_box_position += &push_direction.into();
                    new_pushes += 1;
                    new_moves += 1;
                }

                let mut new_box_positions = self.state.box_positions.clone();
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

                successors.push(Node::new(
                    State {
                        player_position: new_player_position,
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

    /// Returns the priority tuple.
    ///
    /// Lower values indicate higher priority.
    fn priority(&self) -> (i32, i32) {
        match self.strategy {
            Strategy::FastPush => (self.heuristic, self.pushes),
            Strategy::FastMove => (self.heuristic, self.moves),
            Strategy::OptimalPush => (self.pushes + self.heuristic, self.moves),
            Strategy::OptimalMove => (self.moves + self.heuristic, self.pushes),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority()).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
