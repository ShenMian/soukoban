//! Pre-computed data and configuration for search algorithms.

use std::collections::{HashMap, HashSet, VecDeque};

use nalgebra::Vector2;

use crate::{
    Map, Tiles,
    bcc_graph::BccGraph,
    direction::{DirectedPosition, Direction},
};

use super::Strategy;

/// Pre-computed data and configuration shared across search algorithms.
///
/// This holds the map, strategy, and lazily-computed lookup tables needed
/// during search. By separating this from `Solver`, search algorithms and
/// `Node` can reference the context without a circular dependency on the
/// solver itself.
#[derive(Clone, Debug)]
pub struct Context {
    /// The map to solve.
    map: Map,
    /// The search strategy to use.
    strategy: Strategy,
    /// Lower bounds for heuristic calculation.
    lower_bounds: HashMap<Vector2<i32>, i32>,
    /// Set of tunnel positions and directions.
    tunnels: HashSet<DirectedPosition>,
}

impl Context {
    /// Creates a new `Context`, eagerly computing lower bounds and
    /// tunnels.
    pub fn new(map: Map, strategy: Strategy) -> Self {
        let lower_bounds = Self::compute_minimum_push(&map);
        let tunnels = Self::compute_tunnels(&map, &lower_bounds);
        Self {
            map,
            strategy,
            lower_bounds,
            tunnels,
        }
    }

    /// Returns a reference to the map.
    pub fn map(&self) -> &Map {
        &self.map
    }

    /// Returns the strategy.
    pub fn strategy(&self) -> Strategy {
        self.strategy
    }

    /// Returns a reference to the set of lower bounds.
    pub fn lower_bounds(&self) -> &HashMap<Vector2<i32>, i32> {
        &self.lower_bounds
    }

    /// Returns a reference to the set of tunnels.
    pub fn tunnels(&self) -> &HashSet<DirectedPosition> {
        &self.tunnels
    }

    /// Computes and returns the minimum number of pushes to push the box to
    /// the nearest goal.
    fn compute_minimum_push(map: &Map) -> HashMap<Vector2<i32>, i32> {
        let mut lower_bounds = HashMap::new();
        let mut costs = HashMap::new();
        let mut queue = VecDeque::new();

        // Ignore other boxes
        let is_movable = |position| !map[position].intersects(Tiles::Wall);
        let bcc = BccGraph::new(map.player_position(), is_movable);

        for goal_position in map.goal_positions() {
            for pull_direction in Direction::iter() {
                let new_box_position = *goal_position + &pull_direction.into();
                let new_player_position = new_box_position + &pull_direction.into();

                if is_movable(new_box_position) && is_movable(new_player_position) {
                    let state = DirectedPosition(new_box_position, pull_direction);
                    costs.insert(state, 1);
                    queue.push_back((state, 1));
                    lower_bounds.insert(new_box_position, 1);
                }
            }
        }

        for goal in map.goal_positions() {
            lower_bounds.insert(*goal, 0);
        }

        while let Some((state, cost)) = queue.pop_front() {
            let box_position = state.position();
            let player_position = state.forward();

            for pull_direction in Direction::iter() {
                let new_box_position = box_position + &pull_direction.into();
                let new_player_position = new_box_position + &pull_direction.into();

                // Check if the player can pull the box
                if !is_movable(new_box_position) || !is_movable(new_player_position) {
                    continue;
                }
                if !bcc.is_reachable(player_position, new_box_position, box_position) {
                    continue;
                }

                let new_state = DirectedPosition(new_box_position, pull_direction);
                let new_cost = cost + 1;

                let current_cost = costs.entry(new_state).or_insert(i32::MAX);
                if new_cost < *current_cost {
                    *current_cost = new_cost;
                    queue.push_back((new_state, new_cost));

                    let current_min = lower_bounds.entry(new_box_position).or_insert(i32::MAX);
                    if new_cost < *current_min {
                        *current_min = new_cost;
                    }
                }
            }
        }

        lower_bounds
    }

    /// Computes and returns the set of tunnels.
    ///
    /// Tunnel is a common type of no influence push.
    /// Since tunnels are only determined by the map terrain, they can be
    /// pre-calculated.
    fn compute_tunnels(
        map: &Map,
        lower_bounds: &HashMap<Vector2<i32>, i32>,
    ) -> HashSet<DirectedPosition> {
        use itertools::Itertools;

        let mut tunnels = HashSet::new();
        for x in 1..map.dimensions().x - 1 {
            for y in 1..map.dimensions().y - 1 {
                let box_position = Vector2::new(x, y);
                if !map[box_position].intersects(Tiles::Floor) {
                    continue;
                }

                for (up, right, down, left) in [
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                ]
                .into_iter()
                .circular_tuple_windows()
                {
                    let push_direction = up;
                    let (up, right, down, left) =
                        (up.into(), right.into(), down.into(), left.into());

                    let player_position = box_position + &down;

                    // Tunnel template matching patterns:
                    //  .      .      .
                    // #$# or #$_ or _$#
                    // #@#    #@#    #@#
                    if map[player_position + &left].intersects(Tiles::Wall)
                        && map[player_position + &right].intersects(Tiles::Wall)
                        && (map[box_position + &left].intersects(Tiles::Wall)
                            && map[box_position + &right].intersects(Tiles::Wall)
                            || map[box_position + &left].intersects(Tiles::Wall)
                                && map[box_position + &right].intersects(Tiles::Floor)
                            || map[box_position + &left].intersects(Tiles::Floor)
                                && map[box_position + &right].intersects(Tiles::Wall))
                        && map[player_position].intersects(Tiles::Floor)
                        && lower_bounds.contains_key(&(box_position + &up))
                        && !map[box_position].intersects(Tiles::Goal)
                    {
                        tunnels.insert(DirectedPosition(box_position, push_direction));
                    }
                }
            }
        }
        tunnels
    }
}
