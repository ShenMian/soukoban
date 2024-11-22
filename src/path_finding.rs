//! Utilities for path finding.

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

use nalgebra::Vector2;

use crate::{direction::Direction, map::Map, Tiles};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Node {
    position: Vector2<i32>,
    heuristic: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heuristic.cmp(&other.heuristic).reverse()
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
/// starting position to the target position, based on the provided `can_move`
/// function.
pub fn find_path(
    from: Vector2<i32>,
    to: Vector2<i32>,
    can_move: impl Fn(Vector2<i32>) -> bool,
) -> Option<Vec<Vector2<i32>>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut cost = HashMap::new();

    open_set.push(Node {
        position: from,
        heuristic: manhattan_distance(from, to),
    });
    cost.insert(from, 0);

    while let Some(node) = open_set.pop() {
        if node.position == to {
            return Some(construct_path(from, to, came_from));
        }

        for direction in Direction::iter() {
            let new_position = node.position + &direction.into();
            if !can_move(new_position) {
                continue;
            }

            let new_cost = cost[&node.position] + 1;
            if !cost.contains_key(&new_position) || new_cost < cost[&new_position] {
                cost.insert(new_position, new_cost);
                let priority = new_cost + manhattan_distance(new_position, to);
                open_set.push(Node {
                    position: new_position,
                    heuristic: priority,
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
/// position to the target position, based on the provided `can_move` function.
pub fn player_move_path(map: &Map, to: Vector2<i32>) -> Option<Vec<Direction>> {
    let path = find_path(map.player_position(), to, |position| map.can_move(position))?;
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
// 1. 使代码更加灵活, 以便支持不同指标(如移动数)优先的寻路.
// 2. 计算玩家可达全部位置没有必要且非常耗时, 有以下优化方式:
//   -  使用图论的割点来快速判断两点之间的连通性.
//      可以通过预先计算割点来快速判断玩家是否能到达箱子的一侧.
//      因为该方不支持涉及具体路径的计算, 因此无法以最优移动寻路.
//      <http://sokoban.ws/blog/?p=843>
//   - 使用 `find_path` 来检测玩家是否可达指定区域.
//     将上一次搜索路径的终点作为新的搜索起点. 这样曼哈顿距离更小,
//     在大部分情况下会更快.
//   - 使用递归, 增量更新玩家可达范围.
pub fn box_move_waypoints(
    map: &Map,
    initial_box_position: Vector2<i32>,
) -> HashMap<(Vector2<i32>, Direction), u64> {
    debug_assert!(
        map.box_positions().contains(&initial_box_position),
        "box position does not exist"
    );

    let mut deque = VecDeque::new();
    let mut path: HashMap<(Vector2<i32>, Direction), u64> = HashMap::new();

    let player_reachable_area = reachable_area(map.player_position(), |position| {
        position == initial_box_position || map.can_move(position)
    });
    for direction in Direction::iter() {
        if !player_reachable_area.contains(&(initial_box_position - &direction.into())) {
            continue;
        }
        let node = (initial_box_position, direction, 0);
        deque.push_back(node);
    }

    while let Some((box_position, push_direction, cost)) = deque.pop_front() {
        let player_position = box_position - &push_direction.into();
        let player_reachable_area = reachable_area(player_position, |position| {
            (position == initial_box_position || map.can_move(position)) && position != box_position
        });

        let new_cost = cost + 1;
        for push_direction in Direction::iter() {
            let new_box_position = box_position + &push_direction.into();
            if !(new_box_position == initial_box_position || map.can_move(new_box_position)) {
                continue;
            }
            let new_player_position = box_position - &push_direction.into();
            if !player_reachable_area.contains(&new_player_position) {
                continue;
            }

            if path
                .insert((new_box_position, push_direction), new_cost)
                .is_some()
            {
                continue;
            }
            deque.push_back((new_box_position, push_direction, new_cost));
        }
    }

    path
}

/// Creates a path for a box to move from its current position to a target
/// position.
pub fn construct_box_path(
    from: Vector2<i32>,
    to: Vector2<i32>,
    waypoints: &HashMap<(Vector2<i32>, Direction), u64>,
) -> Vec<Vector2<i32>> {
    let mut path = Vec::new();
    let mut current = to;
    // FIXME: 遇到回头路会提前退出
    while current != from {
        path.push(current);
        let mut directions = Vec::new();
        for push_direction in Direction::iter() {
            if waypoints.get(&(current, push_direction)).is_some() {
                directions.push(push_direction);
            }
        }
        let mut min_neighbor = Vector2::zeros();
        let mut min_cost = u64::MAX;
        for push_direction in &directions {
            let neighbor = current - &(*push_direction).into();
            if path.contains(&neighbor) {
                continue;
            }
            for push_direction in Direction::iter() {
                if let Some(cost) = waypoints.get(&(neighbor, push_direction)) {
                    if *cost < min_cost {
                        min_cost = *cost;
                        min_neighbor = neighbor;
                        break;
                    }
                }
            }
        }
        debug_assert_ne!(min_cost, u64::MAX);
        current = min_neighbor;
    }
    path.push(from);
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
        reachable_area(map.player_position(), |position| map.can_move(position));
    let mut pushable_boxes = HashSet::new();
    for box_position in map.box_positions() {
        // Check if the player can push the box from any direction
        for direction in Direction::iter() {
            let player_position = box_position - &direction.into();
            let new_box_position = box_position + &direction.into();
            if player_reachable_area.contains(&player_position) && map.can_move(new_box_position) {
                pushable_boxes.insert(*box_position);
                break;
            }
        }
    }
    pushable_boxes
}

/// Calculates the reachable area starting from a given position.
///
/// This function performs a breadth-first search to determine all positions
/// that can be reached from the starting position, based on the provided
/// `can_move` function.
pub fn reachable_area(
    position: Vector2<i32>,
    can_move: impl Fn(Vector2<i32>) -> bool,
) -> HashSet<Vector2<i32>> {
    let mut reachable_area = HashSet::new();
    let mut deque = VecDeque::<Vector2<i32>>::new();
    deque.push_back(position);

    while let Some(position) = deque.pop_front() {
        if !reachable_area.insert(position) {
            continue;
        }
        for direction in Direction::iter() {
            let neighbor = position + &direction.into();
            if can_move(neighbor) {
                deque.push_back(neighbor);
            }
        }
    }

    reachable_area
}

/// Returns the top-left position.
pub fn normalized_area(area: &HashSet<Vector2<i32>>) -> Option<Vector2<i32>> {
    area.iter()
        .min_by(|a, b| a.y.cmp(&b.y).then_with(|| a.x.cmp(&b.x)))
        .copied()
}

/// Calculates the Manhattan distance between two 2D vectors.
fn manhattan_distance(a: Vector2<i32>, b: Vector2<i32>) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}
