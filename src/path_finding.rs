//! Utilities for path finding.

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, VecDeque},
};

use crate::{
    FxHashMap, FxHashSet, Tiles,
    bcc_graph::BccGraph,
    direction::{DirectedPosition, Direction},
    map::Map,
    point::Point,
    solver::Strategy,
};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Node<T> {
    state: T,
    priority: i32,
}

impl<T: Eq> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl<T: Eq> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Finds a path from one position to another on the map.
///
/// This function uses the A* algorithm to find the shortest path from the
/// starting position to the target position, based on the provided
/// `is_walkable` function.
pub fn find_path(
    from: Point,
    to: Point,
    is_walkable: impl Fn(Point) -> bool,
) -> Option<Vec<Point>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = FxHashMap::default();
    let mut cost = FxHashMap::default();

    cost.insert(from, 0);
    open_set.push(Node {
        state: from,
        priority: manhattan_distance(from, to),
    });

    while let Some(node) = open_set.pop() {
        if node.state == to {
            return Some(construct_path(from, to, &came_from));
        }

        for direction in Direction::iter() {
            let new_position = node.state + &direction.into();
            if !is_walkable(new_position) {
                continue;
            }

            let new_cost = cost[&node.state] + 1;
            if !cost.contains_key(&new_position) || new_cost < cost[&new_position] {
                cost.insert(new_position, new_cost);
                open_set.push(Node {
                    state: new_position,
                    priority: new_cost + manhattan_distance(new_position, to),
                });
                came_from.insert(new_position, node.state);
            }
        }
    }

    None
}

fn construct_path(from: Point, to: Point, came_from: &FxHashMap<Point, Point>) -> Vec<Point> {
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
/// position to the target position, based on the provided `is_walkable`
/// function.
pub fn compute_player_move_directions(map: &Map, to: Point) -> Option<Vec<Direction>> {
    let path = find_path(map.player_position(), to, |position| {
        map.is_walkable(position)
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

/// Calculates the waypoints for the box to move from their current position to
/// reachable positions.
pub fn compute_box_waypoints(
    map: &Map,
    initial_box_position: Point,
    strategy: Strategy,
) -> (
    FxHashMap<DirectedPosition, DirectedPosition>,
    FxHashMap<DirectedPosition, i32>,
) {
    debug_assert!(
        map.box_positions().contains(&initial_box_position),
        "no box at `initial_box_position`"
    );

    let mut queue = BinaryHeap::new();
    let mut costs = FxHashMap::default();
    let mut came_from = FxHashMap::default();

    let player_reachable_area =
        compute_reachable_area(map.player_position(), |position| map.is_walkable(position));
    for push_direction in Direction::iter() {
        let state = DirectedPosition::new(initial_box_position, push_direction);
        // Check if the box can be pushed
        let new_box_position = state.position + &push_direction.into();
        if !map.is_walkable(new_box_position) {
            continue;
        }

        // Check if the player can push the box
        let new_player_position = state.position - &state.direction.into();
        if !(map.is_walkable(new_player_position)) {
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
                    map.is_walkable(position)
                })
                .unwrap()
                .len() as i32
            }
        };
        came_from.insert(state, state);
        queue.push(Node {
            state,
            priority: cost,
        });
        costs.insert(state, cost);
    }

    // Treat other boxes as obstacles
    let is_walkable = |position| position == initial_box_position || map.is_walkable(position);
    let bcc = BccGraph::new(map.player_position(), is_walkable);

    while let Some(Node {
        state,
        priority: cost,
    }) = queue.pop()
    {
        let box_position = state.position;
        let player_position = state.position - &state.direction.into();

        for push_direction in Direction::iter() {
            // Check if the box can be pushed
            let new_box_position = box_position + &push_direction.into();
            if !is_walkable(new_box_position) {
                continue;
            }

            // Check if the player can push the box
            let new_player_position = box_position - &push_direction.into();
            if !is_walkable(new_player_position) {
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
                            (position == initial_box_position || map.is_walkable(position))
                                && position != box_position
                        })
                        .unwrap()
                        .len() as i32
                    }
                };
            let new_state = DirectedPosition::new(new_box_position, push_direction);
            let current_cost = costs.entry(new_state).or_insert(i32::MAX);
            if new_cost < *current_cost {
                *current_cost = new_cost;
                came_from.insert(new_state, state);
                queue.push(Node {
                    state: new_state,
                    priority: new_cost,
                });
            }
        }
    }

    (came_from, costs)
}

/// Constructs a path for the box to move to a target position.
pub fn construct_box_path(
    to: DirectedPosition,
    waypoints: &FxHashMap<DirectedPosition, DirectedPosition>,
) -> Vec<Point> {
    let mut path = Vec::new();
    let mut current = to;
    debug_assert!(waypoints.contains_key(&current));
    while let Some(&prev) = waypoints.get(&current) {
        if prev == current {
            break;
        }
        path.push(current.position);
        current = prev;
    }
    path.push(current.position);
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
    mut player_position: Point,
    box_path: &[Point],
) -> Vec<Point> {
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
pub fn compute_pushable_boxes(map: &Map) -> FxHashSet<Point> {
    let player_reachable_area =
        compute_reachable_area(map.player_position(), |position| map.is_walkable(position));
    let mut pushable_boxes = FxHashSet::default();
    for box_position in map.box_positions() {
        // Check if the player can push the box from any direction
        for push_direction in Direction::iter() {
            let player_position = box_position - &push_direction.into();
            let new_box_position = box_position + &push_direction.into();
            if map.is_walkable(new_box_position) && player_reachable_area.contains(&player_position)
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
/// `is_walkable` function.
pub fn compute_reachable_area(
    position: Point,
    is_walkable: impl Fn(Point) -> bool,
) -> FxHashSet<Point> {
    let mut reachable_area = FxHashSet::default();
    let mut deque = VecDeque::new();
    reachable_area.insert(position);
    deque.push_back(position);
    while let Some(position) = deque.pop_front() {
        for direction in Direction::iter() {
            let neighbor = position + &direction.into();
            if is_walkable(neighbor) && reachable_area.insert(neighbor) {
                deque.push_back(neighbor);
            }
        }
    }
    reachable_area
}

/// Computes the anchor point (top-left) for a given positions.
pub fn compute_area_anchor(area: &FxHashSet<Point>) -> Option<Point> {
    area.iter()
        .min_by(|a, b| (a.y, a.x).cmp(&(b.y, b.x)))
        .copied()
}

/// Calculates the Manhattan distance between two 2D vectors.
fn manhattan_distance(a: Point, b: Point) -> i32 {
    (a - b).abs().sum()
}
