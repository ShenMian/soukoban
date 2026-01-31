//! A solver for the Sokoban problem.

use std::{
    cell::OnceCell,
    collections::{BinaryHeap, HashMap, HashSet},
};

use itertools::Itertools;
use nalgebra::Vector2;

use crate::{
    Action, Actions, Map, SearchError, Tiles,
    direction::Direction,
    node::Node,
    path_finding::{compute_reachable_area, find_path},
    state::State,
};

/// The strategy to use when searching for a solution.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub enum Strategy {
    /// Search for any solution as quickly as possible.
    ///
    /// Using this strategy, A* search degrades into greedy search.
    #[default]
    Fast,
    /// Find the push optimal solution.
    OptimalPush,
    /// Find the move optimal solution.
    OptimalMove,
}

/// A solver for the Sokoban problem.
#[derive(Clone, Debug)]
pub struct Solver {
    map: Map,
    strategy: Strategy,
    /// Lower bounds for heuristic calculation.
    lower_bounds: OnceCell<HashMap<Vector2<i32>, i32>>,
    /// Set of tunnel positions and directions.
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
        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut cost = HashMap::new();

        let state: State = self.map.clone().into();
        cost.insert(state.normalized_hash(self.strategy, &self.map), 0);
        open_set.push(Node::new(state, 0, 0, self));

        while let Some(node) = open_set.pop() {
            if node.is_solved() {
                return Ok(self.construct_actions(&construct_path(node.state, &came_from)));
            }
            for successor in node.successors(self) {
                let hash = successor.state.normalized_hash(self.strategy, &self.map);
                if !cost.contains_key(&hash) || successor.cost() < cost[&hash] {
                    cost.insert(hash, successor.cost());
                    came_from.insert(successor.state.clone(), node.state.clone());
                    open_set.push(successor);
                }
            }
        }
        Err(SearchError::NoSolution)
    }

    /// Searches for solution using the IDA* algorithm.
    pub fn ida_star_search(&self) -> Result<Actions, SearchError> {
        let state: State = self.map.clone().into();
        let node = Node::new(state.clone(), 0, 0, self);

        let mut path = vec![state];
        let mut visited = HashSet::new();
        visited.insert(node.state.normalized_hash(self.strategy, &self.map));
        let mut threshold = node.estimated_total_cost();
        loop {
            match self.ida_star_depth_search(&node, &mut path, &mut visited, threshold) {
                Ok(_state) => return Ok(self.construct_actions(&path)),
                Err(new_threshold) => {
                    if new_threshold == i32::MAX {
                        return Err(SearchError::NoSolution);
                    }
                    threshold = new_threshold;
                }
            }
        }
    }

    /// Depth-limited search for IDA*.
    ///
    /// Returns `Ok(State)` if solution found, `Err(i32)` with the minimum
    /// f-value exceeding threshold.
    fn ida_star_depth_search(
        &self,
        node: &Node,
        path: &mut Vec<State>,
        visited: &mut HashSet<u64>,
        threshold: i32,
    ) -> Result<State, i32> {
        if node.estimated_total_cost() > threshold {
            return Err(node.estimated_total_cost());
        }

        if node.is_solved() {
            return Ok(node.state.clone());
        }

        let mut min_threshold = i32::MAX;
        for successor in node.successors(self) {
            let hash = successor.state.normalized_hash(self.strategy, &self.map);

            // Skip if this state is already in the current search path
            if visited.contains(&hash) {
                continue;
            }

            path.push(successor.state.clone());
            visited.insert(hash);

            match self.ida_star_depth_search(&successor, path, visited, threshold) {
                Ok(state) => return Ok(state),
                Err(new_threshold) => min_threshold = min_threshold.min(new_threshold),
            }

            path.pop();
            visited.remove(&hash);
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

    /// Returns the heuristic value of the state.
    pub fn heuristic(&self, state: &State) -> i32 {
        state
            .box_positions
            .iter()
            .map(|box_position| self.lower_bounds()[box_position])
            .sum()
    }

    /// Returns a reference to the set of lower bounds.
    pub fn lower_bounds(&self) -> &HashMap<Vector2<i32>, i32> {
        self.lower_bounds.get_or_init(|| {
            // TODO: Compute lower bounds based on strategy
            //
            // Since the pushes is always less than or equal to the moves, the current
            // implementation remains admissible.
            let mut lower_bounds = self.compute_minimum_push();
            lower_bounds.shrink_to_fit();
            lower_bounds
        })
    }

    /// Returns a reference to the set of tunnels.
    pub fn tunnels(&self) -> &HashSet<(Vector2<i32>, Direction)> {
        self.tunnels.get_or_init(|| {
            let mut tunnels = self.compute_tunnels();
            tunnels.shrink_to_fit();
            tunnels
        })
    }

    /// Computes and returns the minimum number of pushes to push the box to
    /// the nearest goal.
    fn compute_minimum_push(&self) -> HashMap<Vector2<i32>, i32> {
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
                self.computes_minimum_push_to(
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

    /// Computes the minimum push of the box to the specified position.
    ///
    /// Place the box on the goal, then computes all the positions the box can
    /// be pulled to and the minimum pulls it can be pulled to that position.
    fn computes_minimum_push_to(
        &self,
        box_position: Vector2<i32>,
        player_position: Vector2<i32>,
        lower_bounds: &mut HashMap<Vector2<i32>, i32>,
        visited: &mut HashSet<(Vector2<i32>, Direction)>,
    ) {
        let player_reachable_area = compute_reachable_area(player_position, |position| {
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

            self.computes_minimum_push_to(
                new_box_position,
                new_player_position,
                lower_bounds,
                visited,
            );
        }
    }

    /// Computes and returns the set of tunnels.
    ///
    /// Tunnel is a common type of no influence push.
    /// Since tunnels are only determined by the map terrain, they can be
    /// pre-calculated.
    fn compute_tunnels(&self) -> HashSet<(Vector2<i32>, Direction)> {
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

    fn construct_actions(&self, path: &[State]) -> Actions {
        let mut actions = Actions::new();
        for window in path.windows(2) {
            let (state, next_state) = (&window[0], &window[1]);
            // Find the positions where the box was moved from and to
            let previous_box_position = *state
                .box_positions
                .difference(&next_state.box_positions)
                .next()
                .unwrap();
            let box_position = *next_state
                .box_positions
                .difference(&state.box_positions)
                .next()
                .unwrap();

            // Determine the direction of the push
            let diff = box_position - previous_box_position;
            let push_direction =
                Direction::try_from(Vector2::new(diff.x.signum(), diff.y.signum())).unwrap();

            // Find the path for the player to reach the box position before pushing it
            let mut new_actions: Vec<_> = find_path(
                state.player_position,
                previous_box_position - &push_direction.into(),
                |position| {
                    !self.map()[position].intersects(Tiles::Wall)
                        && !state.box_positions.contains(&position)
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
            debug_assert_eq!(new_box_position, box_position);

            actions.extend(new_actions.iter());
        }
        actions
    }
}

fn construct_path(state: State, came_from: &HashMap<State, State>) -> Vec<State> {
    let mut path = vec![state];
    while let Some(prev_state) = came_from.get(path.last().unwrap()) {
        path.push(prev_state.clone());
    }
    path.reverse();
    path
}
