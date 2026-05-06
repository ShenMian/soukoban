//! Search algorithm implementations.

use std::{
    collections::BinaryHeap,
    sync::atomic::{AtomicBool, Ordering},
};

use itertools::Itertools;

use crate::{
    Action, Actions, FxHashMap, FxHashSet, SearchError, Tiles, direction::Direction,
    path_finding::find_path, solver::Strategy,
};

use super::{context::Context, node::Node, state::State};

/// Searches for a solution using the A* algorithm.
pub fn a_star_search(ctx: &Context, stop_flag: &AtomicBool) -> Result<Actions, SearchError> {
    let mut queue = BinaryHeap::new();
    let mut costs = FxHashMap::default();
    let mut came_from = FxHashMap::default();

    let state: State = ctx.map().clone().into();
    costs.insert(state.canonicalized_hash(ctx), 0);
    queue.push(Node::new(state, 0, 0, ctx));

    while let Some(node) = queue.pop() {
        if stop_flag.load(Ordering::Relaxed) {
            return Err(SearchError::Interrupted);
        }

        if node.is_solved() {
            return Ok(construct_actions(
                ctx,
                &construct_path(node.state, &came_from),
            ));
        }

        for successor in node.successors(ctx) {
            let hash = successor.state.canonicalized_hash(ctx);
            let current_cost = costs.entry(hash).or_insert(i32::MAX);
            if successor.cost() < *current_cost {
                *current_cost = successor.cost();
                came_from.insert(successor.state.clone(), node.state.clone());
                queue.push(successor);
            }
        }
    }
    Err(SearchError::NoSolution)
}

/// Searches for a solution using the IDA* algorithm.
pub fn ida_star_search(ctx: &Context, stop_flag: &AtomicBool) -> Result<Actions, SearchError> {
    let state: State = ctx.map().clone().into();
    let node = Node::new(state.clone(), 0, 0, ctx);

    let mut path = vec![state];
    let mut visited = FxHashSet::default();
    visited.insert(node.state.canonicalized_hash(ctx));
    let mut threshold = node.heuristic();
    loop {
        match ida_star_depth_search(ctx, stop_flag, &node, &mut path, &mut visited, threshold) {
            Ok(_state) => return Ok(construct_actions(ctx, &path)),
            Err(new_threshold) => {
                if new_threshold == i32::MIN {
                    return Err(SearchError::Interrupted);
                }
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
    ctx: &Context,
    stop_flag: &AtomicBool,
    node: &Node,
    path: &mut Vec<State>,
    visited: &mut FxHashSet<u64>,
    threshold: i32,
) -> Result<State, i32> {
    if stop_flag.load(Ordering::Relaxed) {
        return Err(i32::MIN);
    }

    let estimated_cost = node.cost() + node.heuristic();
    if estimated_cost > threshold {
        return Err(estimated_cost);
    }

    if node.is_solved() {
        return Ok(node.state.clone());
    }

    let mut min_threshold = i32::MAX;
    for successor in node.successors(ctx) {
        let hash = successor.state.canonicalized_hash(ctx);

        // Skips if this state is already in the current search path
        if visited.contains(&hash) {
            continue;
        }

        path.push(successor.state.clone());
        visited.insert(hash);

        match ida_star_depth_search(ctx, stop_flag, &successor, path, visited, threshold) {
            Ok(state) => return Ok(state),
            Err(new_threshold) => min_threshold = min_threshold.min(new_threshold),
        }

        path.pop();
        visited.remove(&hash);
    }

    Err(min_threshold)
}

/// Searches for a solution using the BFS algorithm.
///
/// While the tunnel pruning optimization introduces non-uniform costs between
/// adjacent nodes, it still guarantees push-optimality, yet fails to guarantee
/// move-optimality.
///
/// # Panics
///
/// Panics if the context's strategy is [`Strategy::MoveOptimal`].
pub fn bfs_search(ctx: &Context, stop_flag: &AtomicBool) -> Result<Actions, SearchError> {
    assert!(ctx.strategy() != Strategy::MoveOptimal);

    let mut queue = std::collections::VecDeque::new();
    let mut came_from = FxHashMap::default();
    let mut visited = FxHashSet::default();

    let state: State = ctx.map().clone().into();
    visited.insert(state.canonicalized_hash(ctx));

    queue.push_back(Node::new(state, 0, 0, ctx));

    while let Some(node) = queue.pop_front() {
        if stop_flag.load(Ordering::Relaxed) {
            return Err(SearchError::Interrupted);
        }

        if node.is_solved() {
            return Ok(construct_actions(
                ctx,
                &construct_path(node.state, &came_from),
            ));
        }

        for successor in node.successors(ctx) {
            let hash = successor.state.canonicalized_hash(ctx);
            if visited.insert(hash) {
                came_from.insert(successor.state.clone(), node.state.clone());
                queue.push_back(successor);
            }
        }
    }

    Err(SearchError::NoSolution)
}

/// Constructs the sequence of actions from a path of states.
fn construct_actions(ctx: &Context, path: &[State]) -> Actions {
    let mut actions = Actions::new();
    for (from_state, to_state) in path.iter().tuple_windows() {
        debug_assert_eq!(from_state.box_positions.len(), to_state.box_positions.len());
        // Find the positions where the box was moved from and to
        let box_from_position = *from_state
            .box_positions
            .difference(&to_state.box_positions)
            .next()
            .unwrap();
        let box_to_position = *to_state
            .box_positions
            .difference(&from_state.box_positions)
            .next()
            .unwrap();

        // Determine the direction of the push
        let delta = box_to_position - box_from_position;
        let push_direction = Direction::try_from((delta).map(i32::signum)).unwrap();

        // Find the path for the player to reach the box position before pushing it
        actions.extend(
            find_path(
                from_state.player_position,
                box_from_position - &push_direction.into(),
                |position| {
                    !ctx.map()[position].intersects(Tiles::Wall)
                        && !from_state.box_positions.contains(&position)
                },
            )
            .unwrap()
            .windows(2)
            .map(|position| Direction::try_from(position[1] - position[0]).unwrap())
            .map(Action::Move),
        );

        actions.push(Action::Push(push_direction));

        let mut new_box_position = box_from_position + &push_direction.into();
        while new_box_position != box_to_position {
            new_box_position += &push_direction.into();
            actions.push(Action::Push(push_direction));
        }
        debug_assert_eq!(new_box_position, box_to_position);
    }
    actions
}

/// Reconstructs the path from goal to start by following the `came_from` map.
fn construct_path(state: State, came_from: &FxHashMap<State, State>) -> Vec<State> {
    let mut path = vec![state];
    while let Some(prev_state) = came_from.get(path.last().unwrap()) {
        path.push(prev_state.clone());
    }
    path.reverse();
    path
}
