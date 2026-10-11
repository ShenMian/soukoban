//! Utilities for path finding.

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque, hash_map},
    hash::BuildHasher,
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
#[must_use]
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
            let new_position = node.state + direction.into();
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
#[must_use]
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
#[must_use]
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

    let player_distances =
        compute_reachable_distances(map.player_position(), |position| map.is_walkable(position));
    for push_direction in Direction::iter() {
        let state = DirectedPosition::new(initial_box_position, push_direction);
        // Check if the box can be pushed
        let new_box_position = state.position + push_direction.into();
        if !map.is_walkable(new_box_position) {
            continue;
        }

        // Check if the player can push the box
        let new_player_position = state.position - state.direction.into();
        if !(map.is_walkable(new_player_position)) {
            continue;
        }
        if !player_distances.contains_key(&new_player_position) {
            continue;
        }

        let cost = match strategy {
            Strategy::PushOptimal | Strategy::Quick => 0,
            Strategy::MoveOptimal => player_distances[&new_player_position] + 1,
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
        let player_position = state.position - state.direction.into();

        for push_direction in Direction::iter() {
            // Check if the box can be pushed
            let new_box_position = box_position + push_direction.into();
            if !is_walkable(new_box_position) {
                continue;
            }

            // Check if the player can push the box
            let new_player_position = box_position - push_direction.into();
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
#[must_use]
pub fn construct_box_path<S>(
    to: DirectedPosition,
    waypoints: &HashMap<DirectedPosition, DirectedPosition, S>,
) -> Vec<Point>
where
    S: BuildHasher,
{
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
#[must_use]
pub fn construct_player_path(
    map: &Map,
    mut player_position: Point,
    box_path: &[Point],
) -> Vec<Point> {
    let initial_box_position = *box_path.first().unwrap();

    let mut path = Vec::new();

    // Reuse BFS data structures across all iterations to avoid repeated
    // allocation. `came_from` maps each visited position to the position it
    // was reached from. We clear it each iteration but keep the allocation.
    let mut came_from = FxHashMap::default();
    let mut queue = VecDeque::new();

    for box_positions in box_path.windows(2) {
        let current_box_position = box_positions[0];
        let push_direction = box_positions[1] - box_positions[0];
        let target = current_box_position - push_direction;

        // Fast path: player is already at the push position.
        if player_position == target {
            path.push(player_position);
            player_position = current_box_position;
            continue;
        }

        // BFS from player_position to target.
        // The map is bounded by walls, so all positions encountered during BFS
        // are guaranteed to be in-bounds — use unchecked access.
        came_from.clear();
        queue.clear();

        came_from.insert(player_position, player_position);
        queue.push_back(player_position);

        let mut found = false;
        'bfs: while let Some(position) = queue.pop_front() {
            for direction in Direction::iter() {
                let neighbor = position + direction.into();

                // Skip the current box position (it's an obstacle).
                if neighbor == current_box_position {
                    continue;
                }

                // Walkability: either the cell is the initial box position
                // (which has been vacated) or it must not have wall/box tiles.
                // SAFETY: BFS expands only from walkable positions adjacent to
                // walls; the map always has a wall border so we never go out of
                // bounds.
                let walkable = neighbor == initial_box_position
                    || !unsafe { map.get_unchecked(neighbor) }.intersects(Tiles::Wall | Tiles::Box);

                if walkable && let hash_map::Entry::Vacant(entry) = came_from.entry(neighbor) {
                    entry.insert(position);
                    if neighbor == target {
                        found = true;
                        break 'bfs;
                    }
                    queue.push_back(neighbor);
                }
            }
        }

        assert!(found, "no path found for player to reach push position");

        // Trace back the path from `target` to `player_position` and append
        // it directly into `path` in reverse, then flip the segment in-place.
        let segment_start = path.len();
        let mut current = target;
        while current != player_position {
            path.push(current);
            current = came_from[&current];
        }
        path.push(player_position);
        path[segment_start..].reverse();

        player_position = current_box_position;
    }
    path.push(player_position);
    path
}

/// Returns a set of positions of the boxes that can be pushed by the player.
#[must_use]
pub fn compute_pushable_boxes(map: &Map) -> FxHashSet<Point> {
    let player_reachable_area =
        compute_reachable_area(map.player_position(), |position| map.is_walkable(position));
    let mut pushable_boxes = FxHashSet::default();
    for box_position in map.box_positions() {
        // Check if the player can push the box from any direction
        for push_direction in Direction::iter() {
            let player_position = *box_position - push_direction.into();
            let new_box_position = *box_position + push_direction.into();
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
#[must_use]
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
            let neighbor = position + direction.into();
            if is_walkable(neighbor) && reachable_area.insert(neighbor) {
                deque.push_back(neighbor);
            }
        }
    }
    reachable_area
}

/// Computes the reachable positions and their shortest step distances starting
/// from a given position.
///
/// This function performs a breadth-first search to determine all positions
/// and their distances that can be reached from the starting position, based on
/// the provided `is_walkable` function.
#[must_use]
pub fn compute_reachable_distances(
    position: Point,
    is_walkable: impl Fn(Point) -> bool,
) -> FxHashMap<Point, i32> {
    let mut distances = FxHashMap::default();
    let mut deque = VecDeque::new();
    distances.insert(position, 0);
    deque.push_back((position, 0));
    while let Some((curr_pos, curr_dist)) = deque.pop_front() {
        let next_dist = curr_dist + 1;
        for direction in Direction::iter() {
            let neighbor = curr_pos + direction.into();
            if is_walkable(neighbor)
                && let hash_map::Entry::Vacant(entry) = distances.entry(neighbor)
            {
                entry.insert(next_dist);
                deque.push_back((neighbor, next_dist));
            }
        }
    }
    distances
}

/// Computes the anchor point (top-left) for a given positions.
#[must_use]
pub fn compute_area_anchor<S>(area: &HashSet<Point, S>) -> Option<Point>
where
    S: BuildHasher,
{
    area.iter()
        .min_by(|a, b| (a.y, a.x).cmp(&(b.y, b.x)))
        .copied()
}

/// Calculates the Manhattan distance between two 2D vectors.
#[must_use]
fn manhattan_distance(a: Point, b: Point) -> i32 {
    (a - b).abs().sum()
}
