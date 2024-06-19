//! A solver for the Sokoban problem.

use std::{
    cell::OnceCell,
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;
use nalgebra::Vector2;

use crate::{
    direction::Direction, path_finding::reachable_area, state::State, Map, SearchError, Tiles,
};

/// The strategy to use when searching for a solution.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub enum Strategy {
    /// Search for any solution as quickly as possible
    #[default]
    Fast,

    /// Find the push optimal solution
    OptimalPush,
}

#[derive(Clone, Eq, Debug)]
struct Node {
    state: State,
    cost: i32,
    priority: i32,
}

impl Node {
    pub fn new(state: State, cost: i32, solver: &Solver) -> Self {
        let heuristic = state.heuristic(solver);
        let priority = match solver.strategy() {
            Strategy::Fast => heuristic,
            Strategy::OptimalPush => cost + heuristic,
        };
        Self {
            state,
            cost,
            priority,
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
        self.priority.cmp(&other.priority).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A solver for the Sokoban problem.
#[derive(Clone, Debug)]
pub struct Solver {
    map: Map,
    strategy: Strategy,
    lower_bounds: OnceCell<HashMap<Vector2<i32>, i32>>,
    tunnels: OnceCell<HashSet<(Vector2<i32>, Direction)>>,
}

impl Solver {
    /// Creates a new `Solver`.
    pub fn new(map: Map, strategy: Strategy) -> Self {
        Self {
            map,
            strategy,
            lower_bounds: OnceCell::new(),
            tunnels: OnceCell::new(),
        }
    }

    /// Searches for solution using the A* algorithm.
    pub fn a_star_search(&self) -> Result<(), SearchError> {
        let mut heap = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut visited = HashSet::new();

        let state: State = self.map.clone().into();
        heap.push(Node::new(state, 0, self));

        while let Some(Node { state, cost, .. }) = heap.pop() {
            if state.is_solved(self) {
                return Ok(());
            }
            for successor in state.successors(self) {
                if !visited.insert(successor.normalized_hash(&self.map)) {
                    continue;
                }
                came_from.insert(successor.clone(), state.clone());
                heap.push(Node::new(successor, cost + 1, self));
            }
        }
        Err(SearchError::NoSolution)
    }

    /// Searches for solution using the IDA* algorithm.
    pub fn ida_star_search(&self) -> Result<(), SearchError> {
        let state: State = self.map.clone().into();
        let mut threshold = state.heuristic(self);
        loop {
            match self.ida_star_search_inner(&state, 0, threshold, &mut HashSet::new()) {
                Ok(()) => return Ok(()),
                Err(t) => threshold = t,
            }
            if threshold == i32::MAX {
                return Err(SearchError::NoSolution);
            }
        }
    }

    fn ida_star_search_inner(
        &self,
        state: &State,
        cost: i32,
        threshold: i32,
        visited: &mut HashSet<u64>,
    ) -> Result<(), i32> {
        if !visited.insert(state.normalized_hash(&self.map)) {
            return Err(i32::MAX);
        }
        if state.is_solved(self) {
            return Ok(());
        }
        if cost > threshold {
            return Err(cost);
        }
        let mut min_threshold = i32::MAX;
        for successor in state.successors(self) {
            match self.ida_star_search_inner(&successor, cost + 1, threshold, visited) {
                Ok(()) => return Ok(()),
                Err(t) => min_threshold = min_threshold.min(t),
            }
        }
        Err(min_threshold)
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
        self.lower_bounds.get_or_init(|| {
            let mut lower_bounds = self.calculate_minimum_push();
            lower_bounds.shrink_to_fit();
            lower_bounds
        })
    }

    /// Returns a reference to the set of tunnels.
    pub fn tunnels(&self) -> &HashSet<(Vector2<i32>, Direction)> {
        self.tunnels.get_or_init(|| {
            let mut tunnels = self.calculate_tunnels();
            tunnels.shrink_to_fit();
            tunnels
        })
    }

    /// Calculates and returns the minimum push to goals.
    fn calculate_minimum_push(&self) -> HashMap<Vector2<i32>, i32> {
        let mut lower_bounds = HashMap::new();
        for goal_position in self.map.goal_positions() {
            lower_bounds.insert(*goal_position, 0);

            for pull_direction in Direction::iter() {
                let new_box_position = goal_position + &pull_direction.into();
                let new_player_position = new_box_position + &pull_direction.into();
                if !self.map.in_bounds(new_player_position)
                    || self.map[new_box_position].intersects(Tiles::Wall)
                    || self.map[new_player_position].intersects(Tiles::Wall)
                {
                    continue;
                }
                self.calculate_minimum_push_to(
                    *goal_position,
                    new_player_position,
                    &mut lower_bounds,
                    &mut HashSet::new(),
                );
                break;
            }
        }
        lower_bounds
    }

    /// Calculates the minimum push of the box to the specified position.
    ///
    /// Place the box on the goal, then calculate all the positions the box can
    /// be pulled to and the minimum pulls it can be pulled to that position.
    fn calculate_minimum_push_to(
        &self,
        box_position: Vector2<i32>,
        player_position: Vector2<i32>,
        lower_bounds: &mut HashMap<Vector2<i32>, i32>,
        visited: &mut HashSet<(Vector2<i32>, Direction)>,
    ) {
        let player_reachable_area = reachable_area(player_position, |position| {
            !(self.map[position].intersects(Tiles::Wall) || position == box_position)
        });
        for pull_direction in Direction::iter() {
            let new_box_position = box_position + &pull_direction.into();
            if self.map[new_box_position].intersects(Tiles::Wall) {
                continue;
            }
            let new_player_position = new_box_position + &pull_direction.into();
            if self.map[new_player_position].intersects(Tiles::Wall)
                || !player_reachable_area.contains(&new_player_position)
            {
                continue;
            }

            let lower_bound = *lower_bounds.get(&new_box_position).unwrap_or(&i32::MAX);
            if !visited.insert((new_box_position, pull_direction)) {
                continue;
            }
            let new_lower_bound = lower_bounds[&box_position] + 1;
            if new_lower_bound < lower_bound {
                lower_bounds.insert(new_box_position, new_lower_bound);
            }

            self.calculate_minimum_push_to(
                new_box_position,
                new_player_position,
                lower_bounds,
                visited,
            );
        }
    }

    fn calculate_tunnels(&self) -> HashSet<(Vector2<i32>, Direction)> {
        let mut tunnels = HashSet::new();
        for x in 1..self.map.dimensions().x - 1 {
            for y in 1..self.map.dimensions().y - 1 {
                let box_position = Vector2::new(x, y);
                if !self.map[box_position].intersects(Tiles::Floor) {
                    continue;
                }

                for (up, right, down, left) in [
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                    Direction::Left,
                    Direction::Up,
                    Direction::Right,
                    Direction::Down,
                ]
                .into_iter()
                .tuple_windows()
                {
                    //  .
                    // #$#
                    // #@#
                    if self.map[box_position + &down.into() + &left.into()].intersects(Tiles::Wall)
                        && self.map[box_position + &down.into() + &right.into()]
                            .intersects(Tiles::Wall)
                        && self.map[box_position + &left.into()].intersects(Tiles::Wall)
                        && self.map[box_position + &right.into()].intersects(Tiles::Wall)
                        && self.map[box_position].intersects(Tiles::Floor)
                        && self
                            .lower_bounds()
                            .contains_key(&(box_position + &up.into()))
                        && !self.map[box_position].intersects(Tiles::Goal)
                    {
                        tunnels.insert((box_position, up));
                    }

                    //  .   .
                    // #$_ _$#
                    // #@# #@#
                    if self.map[box_position + &down.into() + &left.into()].intersects(Tiles::Wall)
                        && self.map[box_position + &down.into() + &right.into()]
                            .intersects(Tiles::Wall)
                        && (self.map[box_position + &right.into()].intersects(Tiles::Wall)
                            && self.map[box_position + &left.into()].intersects(Tiles::Floor)
                            || self.map[box_position + &right.into()].intersects(Tiles::Floor)
                                && self.map[box_position + &left.into()].intersects(Tiles::Wall))
                        && self.map[box_position].intersects(Tiles::Floor)
                        && self
                            .lower_bounds()
                            .contains_key(&(box_position + &up.into()))
                        && !self.map[box_position].intersects(Tiles::Goal)
                    {
                        tunnels.insert((box_position, up));
                    }
                }
            }
        }
        tunnels
    }
}
