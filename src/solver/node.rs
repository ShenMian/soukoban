//! A node in the search tree.

use std::cmp::Ordering;

use crate::{
    FxHashSet, Tiles,
    deadlock::is_freeze_deadlock,
    direction::{DirectedPosition, Direction},
    path_finding::compute_reachable_distances,
    point::Point,
};

use super::{Strategy, context::Context, state::State};

/// A node in the search tree.
#[derive(Clone, Eq, Debug)]
pub struct Node {
    /// The state.
    pub state: State,
    /// Number of pushes from the start node to this node.
    pub pushes: i32,
    /// Number of moves from the start node to this node.
    pub moves: i32,
    /// Estimated remaining cost (h-value).
    heuristic: i32,
    /// Strategy that determines how [`cost`](Self::cost) is computed.
    strategy: Strategy,
}

impl Node {
    /// Creates a new `Node`.
    #[must_use]
    pub fn new(state: State, pushes: i32, moves: i32, ctx: &Context) -> Self {
        Self {
            heuristic: state.lower_bound(ctx),
            state,
            pushes,
            moves,
            strategy: ctx.strategy(),
        }
    }

    /// Returns the actual cost from the start node to this node (g-value).
    #[must_use]
    pub fn cost(&self) -> i32 {
        match self.strategy {
            Strategy::Quick => 0,
            Strategy::PushOptimal => self.pushes,
            Strategy::MoveOptimal => self.moves,
        }
    }

    /// Returns the estimated cost from this node to the goal (h-value).
    #[must_use]
    pub fn heuristic(&self) -> i32 {
        self.heuristic
    }

    /// Returns `true` if the state is solved.
    #[must_use]
    pub fn is_solved(&self) -> bool {
        self.heuristic == 0
    }

    /// Returns the successors of the node.
    #[must_use]
    pub fn successors(&self, ctx: &Context) -> Vec<Self> {
        let is_walkable = |position| {
            !ctx.map()[position].intersects(Tiles::Wall)
                && !self.state.box_positions.contains(&position)
        };
        let player_distances = compute_reachable_distances(self.state.player_position, is_walkable);

        // Creates successor states by pushing boxes
        let mut successors = Vec::new();
        for box_position in &self.state.box_positions {
            for push_direction in Direction::iter() {
                // Check if the box can be pushed
                let new_box_position = *box_position + push_direction.into();
                if ctx.map()[new_box_position].intersects(Tiles::Wall)
                    || self.state.box_positions.contains(&new_box_position)
                    || ctx.is_dead_position(new_box_position)
                {
                    continue;
                }

                // Check if the player can push the box
                let push_position = *box_position - push_direction.into();
                let Some(&moves) = player_distances.get(&push_position) else {
                    continue;
                };

                // Slides the box through a tunnel
                let (new_box_position, mut pushes) = Self::slide_through_tunnel(
                    ctx,
                    &self.state.box_positions,
                    new_box_position,
                    push_direction,
                );

                pushes += 1;

                let mut new_box_positions = self.state.box_positions.clone();
                new_box_positions.remove(box_position);
                new_box_positions.insert(new_box_position);

                // Skips freeze deadlocks
                if Self::is_deadlock(ctx, new_box_position, &new_box_positions) {
                    continue;
                }

                let new_player_position = new_box_position - push_direction.into();
                successors.push(Self::new(
                    State {
                        player_position: new_player_position,
                        box_positions: new_box_positions,
                    },
                    self.pushes + pushes,
                    self.moves + moves + pushes,
                    ctx,
                ));
            }
        }
        successors
    }

    /// Slides the box through a tunnel.
    ///
    /// Returns the final box position and the number of additional pushes that
    /// occurred inside the tunnel.
    fn slide_through_tunnel(
        ctx: &Context,
        box_positions: &FxHashSet<Point>,
        mut box_position: Point,
        push_direction: Direction,
    ) -> (Point, i32) {
        let mut pushes = 0;
        while ctx
            .tunnels()
            .contains(&DirectedPosition::new(box_position, push_direction))
            && !box_positions.contains(&(box_position + push_direction.into()))
            && !ctx.is_dead_position(box_position + push_direction.into())
        {
            box_position += push_direction.into();
            pushes += 1;
        }
        (box_position, pushes)
    }

    /// Returns `true` if placing a box at `box_position` would create a
    /// deadlock.
    fn is_deadlock(ctx: &Context, box_position: Point, box_positions: &FxHashSet<Point>) -> bool {
        ctx.is_dead_position(box_position)
            || is_freeze_deadlock(ctx.map(), box_position, box_positions)
    }

    /// Returns the priority tuple.
    ///
    /// Lower values indicate higher priority.
    fn priority(&self) -> (i32, i32) {
        match self.strategy {
            Strategy::Quick => (self.heuristic, self.pushes),
            Strategy::PushOptimal => (self.pushes + self.heuristic, self.moves),
            Strategy::MoveOptimal => (self.moves + self.heuristic, self.pushes),
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
