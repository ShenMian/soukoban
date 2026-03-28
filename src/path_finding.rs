//! Utilities for path finding.

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use nalgebra::Vector2;

use crate::{
    Tiles,
    bcc_graph::BccGraph,
    direction::{DirectedPosition, Direction},
    map::Map,
    solver::Strategy,
};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Node {
    position: Vector2<i32>,
    priority: i32,
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

/// Finds a path from one position to another on the map.
///
/// This function uses the A* algorithm to find the shortest path from the
/// starting position to the target position, based on the provided `is_movable`
/// function.
pub fn find_path(
    from: Vector2<i32>,
    to: Vector2<i32>,
    is_movable: impl Fn(Vector2<i32>) -> bool,
) -> Option<Vec<Vector2<i32>>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut cost = HashMap::new();

    cost.insert(from, 0);
    open_set.push(Node {
        position: from,
        priority: manhattan_distance(from, to),
    });

    while let Some(node) = open_set.pop() {
        if node.position == to {
            return Some(construct_path(from, to, came_from));
        }

        for direction in Direction::iter() {
            let new_position = node.position + &direction.into();
            if !is_movable(new_position) {
                continue;
            }

            let new_cost = cost[&node.position] + 1;
            if !cost.contains_key(&new_position) || new_cost < cost[&new_position] {
                cost.insert(new_position, new_cost);
                open_set.push(Node {
                    position: new_position,
                    priority: new_cost + manhattan_distance(new_position, to),
                });
                came_from.insert(new_position, node.position);
            }
        }
    }

    None
}

fn construct_path(
    from: Vector2<i32>,
    to: Vector2<i32>,
    came_from: HashMap<Vector2<i32>, Vector2<i32>>,
) -> Vec<Vector2<i32>> {
    let mut path = Vec::new();
    let mut current = to;
    while current != from {
        path.push(current);
        current = came_from[&current];
    }
    path.push(from);
    path.reverse();
    path
}

/// Calculates the path for the player to move from their current position to a
/// target position.
///
/// This function finds a path using the A* algorithm from the player's current
/// position to the target position, based on the provided `is_movable`
/// function.
pub fn compute_player_move_directions(map: &Map, to: Vector2<i32>) -> Option<Vec<Direction>> {
    let path = find_path(map.player_position(), to, |position| {
        map.is_movable(position)
    })?;
    // Converts the path of positions into a path of directions.
    #[expect(
        clippy::missing_panics_doc,
        reason = "infallible: `find_path` always returns an orthogonally contiguous path"
    )]
    let directions = path
        .windows(2)
        .map(|position| Direction::try_from(position[1] - position[0]).unwrap())
        .collect();
    Some(directions)
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct BoxNode {
    state: DirectedPosition,
    cost: i32,
}

impl BoxNode {
    fn new(state: DirectedPosition, cost: i32) -> Self {
        Self { state, cost }
    }
}

impl Ord for BoxNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for BoxNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Calculates the waypoints for the box to move from their current position to
/// reachable positions.
pub fn compute_box_waypoints(
    map: &Map,
    initial_box_position: Vector2<i32>,
    strategy: Strategy,
) -> (
    HashMap<DirectedPosition, DirectedPosition>,
    HashMap<DirectedPosition, i32>,
) {
    debug_assert!(
        map.box_positions().contains(&initial_box_position),
        "no box at `initial_box_position`"
    );

    let mut queue = BinaryHeap::new();
    let mut costs = HashMap::new();
    let mut came_from = HashMap::new();

    let player_reachable_area =
        compute_reachable_area(map.player_position(), |position| map.is_movable(position));
    for push_direction in Direction::iter() {
        let state = DirectedPosition(initial_box_position, push_direction);
        // Checks if the box can be pushed
        let new_box_position = state.forward();
        if !map.is_movable(new_box_position) {
            continue;
        }

        // Checks if the player can push the box
        let new_player_position = state.backward();
        if !(map.is_movable(new_player_position)) {
            continue;
        }
        if !player_reachable_area.contains(&new_player_position) {
            continue;
        }

        #[expect(
            clippy::missing_panics_doc,
            reason = "infallible: path is guaranteed to exist"
        )]
        let cost = match strategy {
            Strategy::PushOptimal | Strategy::Quick => 0,
            Strategy::MoveOptimal => {
                find_path(map.player_position(), new_player_position, |position| {
                    map.is_movable(position)
                })
                .unwrap()
                .len() as i32
            }
        };
        came_from.insert(state, state);
        queue.push(BoxNode::new(state, cost));
        costs.insert(state, cost);
    }

    let is_movable = |position| position == initial_box_position || map.is_movable(position);
    let bcc = BccGraph::new(map.player_position(), is_movable);
    while let Some(BoxNode { state, cost }) = queue.pop() {
        let (box_position, player_position) = (state.position(), state.backward());

        for push_direction in Direction::iter() {
            // Checks if the box can be pushed
            let new_box_position = box_position + &push_direction.into();
            if !is_movable(new_box_position) {
                continue;
            }

            // Checks if the player can push the box
            let new_player_position = box_position - &push_direction.into();
            if !is_movable(new_player_position) {
                continue;
            }
            if !bcc.is_reachable(player_position, new_player_position, box_position) {
                continue;
            }

            #[expect(
                clippy::missing_panics_doc,
                reason = "infallible: path is guaranteed to exist"
            )]
            let new_cost = cost
                + match strategy {
                    Strategy::PushOptimal | Strategy::Quick => 1,
                    Strategy::MoveOptimal => {
                        find_path(player_position, new_player_position, |position| {
                            (position == initial_box_position || map.is_movable(position))
                                && position != box_position
                        })
                        .unwrap()
                        .len() as i32
                    }
                };
            let new_state = DirectedPosition(new_box_position, push_direction);
            let current_cost = costs.entry(new_state).or_insert(i32::MAX);
            if new_cost < *current_cost {
                *current_cost = new_cost;
                came_from.insert(new_state, state);
                queue.push(BoxNode::new(new_state, new_cost));
            }
        }
    }

    (came_from, costs)
}

/// Constructs a path for the box to move to a target position.
pub fn construct_box_path(
    to: DirectedPosition,
    waypoints: &HashMap<DirectedPosition, DirectedPosition>,
) -> Vec<Vector2<i32>> {
    let mut path = Vec::new();
    let mut current = to;
    debug_assert!(waypoints.contains_key(&current));
    while let Some(&prev) = waypoints.get(&current) {
        if prev == current {
            break;
        }
        path.push(current.position());
        current = prev;
    }
    path.push(current.position());
    path.reverse();
    path
}

/// Constructs player path based on box path.
///
/// # Panics
///
/// Panics if parameters are invalid.
pub fn construct_player_path(
    map: &Map,
    mut player_position: Vector2<i32>,
    box_path: &[Vector2<i32>],
) -> Vec<Vector2<i32>> {
    let mut path = Vec::new();
    let initial_box_position = *box_path.first().unwrap();
    for box_positions in box_path.windows(2) {
        let push_direction = box_positions[1] - box_positions[0];
        let new_player_position = box_positions[0] - push_direction;
        path.append(
            &mut find_path(player_position, new_player_position, |position| {
                (position == initial_box_position
                    || !map[position].intersects(Tiles::Wall | Tiles::Box))
                    && position != box_positions[0]
            })
            .unwrap(),
        );
        player_position = box_positions[0];
    }
    path.push(player_position);
    path
}

/// Returns a set of positions of the boxes that can be pushed by the player.
pub fn compute_pushable_boxes(map: &Map) -> HashSet<Vector2<i32>> {
    let player_reachable_area =
        compute_reachable_area(map.player_position(), |position| map.is_movable(position));
    let mut pushable_boxes = HashSet::new();
    for box_position in map.box_positions() {
        // Check if the player can push the box from any direction
        for push_direction in Direction::iter() {
            let player_position = box_position - &push_direction.into();
            let new_box_position = box_position + &push_direction.into();
            if map.is_movable(new_box_position) && player_reachable_area.contains(&player_position)
            {
                pushable_boxes.insert(*box_position);
                break;
            }
        }
    }
    pushable_boxes
}

/// Computes the reachable area starting from a given position.
///
/// This function performs a breadth-first search to determine all positions
/// that can be reached from the starting position, based on the provided
/// `is_movable` function.
pub fn compute_reachable_area(
    position: Vector2<i32>,
    is_movable: impl Fn(Vector2<i32>) -> bool,
) -> HashSet<Vector2<i32>> {
    let mut reachable_area = HashSet::new();
    let mut deque = VecDeque::new();
    reachable_area.insert(position);
    deque.push_back(position);
    while let Some(position) = deque.pop_front() {
        for direction in Direction::iter() {
            let neighbor = position + &direction.into();
            if is_movable(neighbor) && reachable_area.insert(neighbor) {
                deque.push_back(neighbor);
            }
        }
    }
    reachable_area
}

/// Computes the anchor point (top-left) for a given positions.
pub fn compute_area_anchor(area: &HashSet<Vector2<i32>>) -> Option<Vector2<i32>> {
    area.iter()
        .min_by(|a, b| (a.y, a.x).cmp(&(b.y, b.x)))
        .copied()
}

/// Calculates the Manhattan distance between two 2D vectors.
fn manhattan_distance(a: Vector2<i32>, b: Vector2<i32>) -> i32 {
    (a - b).abs().sum()
}
