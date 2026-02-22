//! Utilities for path finding.

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque, hash_map::Entry},
};

use nalgebra::Vector2;

use crate::{
    Tiles,
    direction::{DirectedPosition, Direction},
    map::Map,
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
pub fn player_move_path(map: &Map, to: Vector2<i32>) -> Option<Vec<Direction>> {
    let path = find_path(map.player_position(), to, |position| {
        map.is_movable(position)
    })?;
    Some(convert_path_from_points_to_directions(path))
}

/// Converts a position path into a direction path.
fn convert_path_from_points_to_directions(path: Vec<Vector2<i32>>) -> Vec<Direction> {
    path.windows(2)
        .map(|position| Direction::try_from(position[1] - position[0]).unwrap())
        .collect()
}

/// Calculates the waypoints for the box to move from their current position to
/// reachable positions.
// TODO:
// 1. 通过预计算双连通分量, 实现常量时间内判断玩家可达性.
//    该方法好像无法支持穿透功能.
//    详情请参考: <http://sokoban.ws/blog/?p=843>
// 2. 支持移动数优先的寻路. 保持 costs 依然每次下降 1, 但是 deque 中的 cost
//    改为实际代价, 比如移动数.
// 3. 增量更新玩家可达范围.
pub fn box_move_waypoints(
    map: &Map,
    initial_box_position: Vector2<i32>,
) -> HashMap<DirectedPosition, u64> {
    debug_assert!(
        map.box_positions().contains(&initial_box_position),
        "no box at `initial_box_position`"
    );

    let mut deque = VecDeque::new();
    let mut costs = HashMap::new();

    let player_reachable_area =
        compute_reachable_area(map.player_position(), |position| map.is_movable(position));
    for push_direction in Direction::iter() {
        let state = DirectedPosition(initial_box_position, push_direction);
        let new_box_position = state.forward();
        if !map.is_movable(new_box_position) {
            continue;
        }
        let new_player_position = state.backward();
        if !player_reachable_area.contains(&new_player_position) {
            continue;
        }
        costs.insert(state, 0);
        deque.push_back((state, 0));
    }

    while let Some((state, cost)) = deque.pop_front() {
        let (box_position, player_position) = (state.position(), state.backward());
        let player_reachable_area = compute_reachable_area(player_position, |position| {
            (position == initial_box_position || map.is_movable(position))
                && position != box_position
        });

        for push_direction in Direction::iter() {
            // Checks if the box can be pushed
            let new_box_position = box_position + &push_direction.into();
            if !(new_box_position == initial_box_position || map.is_movable(new_box_position)) {
                continue;
            }

            // Checks if the player can push the box
            let new_player_position = box_position - &push_direction.into();
            if !player_reachable_area.contains(&new_player_position) {
                continue;
            }

            let new_cost = cost + 1;
            let new_state = DirectedPosition(new_box_position, push_direction);
            if let Entry::Vacant(entry) = costs.entry(new_state) {
                entry.insert(new_cost);
                deque.push_back((new_state, new_cost));
            }
        }
    }

    costs
}

/// Constructs a path for the box to move to a target position.
pub fn construct_box_path(
    to: Vector2<i32>,
    waypoints: &HashMap<DirectedPosition, u64>,
) -> Vec<Vector2<i32>> {
    // Computes the last push direction and cost
    let (mut direction, mut cost) = Direction::iter()
        .filter_map(|direction| {
            waypoints
                .get(&DirectedPosition(to, direction))
                .map(|&cost| (direction, cost))
        })
        .min_by_key(|&(_, cost)| cost)
        .unwrap();

    let mut path = Vec::new();
    let mut position = to;
    // Gradient descent with a step size of 1
    while cost > 0 {
        path.push(position);
        position -= &direction.into();
        cost -= 1;

        direction = Direction::iter()
            .find(|&direction| waypoints.get(&DirectedPosition(position, direction)) == Some(&cost))
            .unwrap();
    }
    path.push(position);
    path.reverse();
    path
}

/// Constructs player path based on box path.
pub fn construct_player_path(
    map: &Map,
    mut player_position: Vector2<i32>,
    box_path: &[Vector2<i32>],
) -> Vec<Vector2<i32>> {
    let mut path = Vec::new();
    let initial_box_position = *box_path.first().unwrap();
    for box_positions in box_path.windows(2) {
        let direction = box_positions[1] - box_positions[0];
        let new_player_position = box_positions[0] - direction;
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
pub fn pushable_boxes(map: &Map) -> HashSet<Vector2<i32>> {
    let player_reachable_area =
        compute_reachable_area(map.player_position(), |position| map.is_movable(position));
    let mut pushable_boxes = HashSet::new();
    for box_position in map.box_positions() {
        // Check if the player can push the box from any direction
        for direction in Direction::iter() {
            let player_position = box_position - &direction.into();
            let new_box_position = box_position + &direction.into();
            if player_reachable_area.contains(&player_position) && map.is_movable(new_box_position)
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
    let mut deque = VecDeque::<Vector2<i32>>::new();
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
