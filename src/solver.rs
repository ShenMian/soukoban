//! A solver for the Sokoban problem.

use std::{
    cell::OnceCell,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;
use nalgebra::Vector2;

use crate::{
    direction::Direction,
    node::Node,
    path_finding::{find_path, reachable_area},
    state::State,
    Action, Actions, Map, SearchError, Tiles,
};

/// The strategy to use when searching for a solution.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub enum Strategy {
    /// Search for any solution as quickly as possible
    #[default]
    Fast,

    /// Find the push optimal solution
    OptimalPush,

    /// Find the move optimal solution
    OptimalMove,
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
    pub fn a_star_search(&self) -> Result<Actions, SearchError> {
        let mut heap = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut visited = HashSet::new();

        let state: State = self.map.clone().into();
        heap.push(Node::new(state, 0, 0, self));

        while let Some(node) = heap.pop() {
            if node.state.is_solved(self) {
                return Ok(self.construct_actions(node.state, &came_from));
            }
            for successor in node.successors(self) {
                if !visited.insert(successor.state.normalized_hash(&self.map)) {
                    continue;
                }
                came_from.insert(successor.state.clone(), node.state.clone());
                heap.push(successor);
            }
        }
        Err(SearchError::NoSolution)
    }

    /// Searches for solution using the IDA* algorithm.
    pub fn ida_star_search(&self) -> Result<(), SearchError> {
        let state: State = self.map.clone().into();
        let mut threshold = state.heuristic(self);
        let node = Node::new(state, 0, 0, self);
        loop {
            match self.ida_star_search_inner(&node, threshold, &mut HashSet::new()) {
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
        node: &Node,
        push_threshold: i32,
        visited: &mut HashSet<u64>,
    ) -> Result<(), i32> {
        if !visited.insert(node.state.normalized_hash(&self.map)) {
            return Err(i32::MAX);
        }
        if node.state.is_solved(self) {
            return Ok(());
        }
        if node.pushes > push_threshold {
            return Err(node.pushes);
        }
        let mut min_threshold = i32::MAX;
        for successor in node.successors(self) {
            match self.ida_star_search_inner(&successor, push_threshold, visited) {
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
        // FIXME: Calculate lower bounds based on strategy
        self.lower_bounds.get_or_init(|| {
            assert!(self.strategy == Strategy::OptimalPush || self.strategy == Strategy::Fast);
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

    /// Calculates and returns the minimum number of pushes to push the box to
    /// the nearest goal.
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

    /// Calculates and returns the set of tunnels.
    ///
    /// Tunnel is a common type of no influence push.
    /// Since tunnels are only determined by the map terrain, they can be
    /// pre-calculated.
    fn calculate_tunnels(&self) -> HashSet<(Vector2<i32>, Direction)> {
        let mut tunnels = HashSet::new();
        for x in 1..self.map.dimensions().x - 1 {
            for y in 1..self.map.dimensions().y - 1 {
                let box_position = Vector2::new(x, y);
                if !self.map[box_position].intersects(Tiles::Floor) {
                    continue;
                }

                for (up, right, down, left) in Direction::iter().circular_tuple_windows() {
                    let push_direction = up;
                    let (up, right, down, left) =
                        (up.into(), right.into(), down.into(), left.into());

                    let player_position = box_position + &down;

                    // Tunnel patterns:
                    //  .      .      .
                    // #$# or #$_ or _$#
                    // #@#    #@#    #@#
                    if self.map[player_position + &left].intersects(Tiles::Wall)
                        && self.map[player_position + &right].intersects(Tiles::Wall)
                        && (self.map[box_position + &left].intersects(Tiles::Wall)
                            && self.map[box_position + &right].intersects(Tiles::Wall)
                            || self.map[box_position + &right].intersects(Tiles::Wall)
                                && self.map[box_position + &left].intersects(Tiles::Floor)
                            || self.map[box_position + &right].intersects(Tiles::Floor)
                                && self.map[box_position + &left].intersects(Tiles::Wall))
                        && self.map[box_position].intersects(Tiles::Floor)
                        && self.lower_bounds().contains_key(&(box_position + &up))
                        && !self.map[box_position].intersects(Tiles::Goal)
                    {
                        tunnels.insert((player_position, push_direction));
                    }
                }
            }
        }
        tunnels
    }

    fn construct_actions(&self, mut state: State, came_from: &HashMap<State, State>) -> Actions {
        let mut actions = Actions::new();
        while let Some(prev_state) = came_from.get(&state) {
            // Find the positions where the box was moved from and to
            let previous_box_position = *prev_state
                .box_positions
                .difference(&state.box_positions)
                .next()
                .unwrap();
            let box_position = *state
                .box_positions
                .difference(&prev_state.box_positions)
                .next()
                .unwrap();

            // Determine the direction of the push
            let diff = box_position - previous_box_position;
            let push_direction =
                Direction::try_from(Vector2::new(diff.x.signum(), diff.y.signum())).unwrap();

            // Find the path for the player to reach the box position before pushing it
            let mut new_actions: Vec<_> = find_path(
                prev_state.player_position,
                previous_box_position - &push_direction.into(),
                |position| {
                    !self.map()[position].intersects(Tiles::Wall)
                        && !prev_state.box_positions.contains(&position)
                },
            )
            .unwrap()
            .windows(2)
            .map(|position| Direction::try_from(position[1] - position[0]).unwrap())
            .map(Action::Move)
            .collect();

            new_actions.push(Action::Push(push_direction));

            let mut new_box_position = previous_box_position + &push_direction.into();
            while self.tunnels().contains(&(new_box_position, push_direction)) {
                new_box_position += &push_direction.into();
                new_actions.push(Action::Push(push_direction));
            }

            actions.splice(0..0, new_actions.iter().copied());
            state = prev_state.clone();
        }
        actions
    }
}
