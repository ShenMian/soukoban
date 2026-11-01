//! Utilities for deadlocks detection.

use std::{
    collections::{HashSet, VecDeque},
    hash::BuildHasher,
};

use crate::{
    FxHashSet, direction::Direction, map::Map, point::Point, solver::Context, tiles::Tiles,
};

/// Checks if the given box position is a static deadlock.
///
/// # Time complexity
///
/// Typically `O(B)` where `B` is the total number of boxes.
#[must_use]
pub fn is_static_deadlock<S>(
    map: &Map,
    box_position: Point,
    box_positions: &HashSet<Point, S>,
) -> bool
where
    S: BuildHasher + Default,
{
    debug_assert!(box_positions.contains(&box_position));
    is_static_deadlock_inner(
        map,
        box_position,
        box_positions,
        &mut HashSet::<Point, S>::default(),
    )
}

#[must_use]
fn is_static_deadlock_inner<S>(
    map: &Map,
    box_position: Point,
    box_positions: &HashSet<Point, S>,
    visited: &mut HashSet<Point, S>,
) -> bool
where
    S: BuildHasher,
{
    if !visited.insert(box_position) {
        return true;
    }

    for direction in [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ]
    .windows(3)
    {
        let neighbors = [
            box_position + direction[0].into(),
            box_position + direction[1].into(),
            box_position + direction[2].into(),
        ];
        for neighbor in &neighbors {
            if map[*neighbor].intersects(Tiles::Wall) {
                continue;
            }
            if box_positions.contains(neighbor)
                && is_static_deadlock_inner(map, *neighbor, box_positions, visited)
            {
                continue;
            }
            return false;
        }
    }
    true
}

/// Checks if the given box position is a freeze deadlock.
///
/// A frozen group is only a deadlock if at least one of its boxes is off a
/// goal.
///
/// # Time complexity
///
/// Typically `O(B²)` where `B` is the number of boxes.
#[must_use]
pub fn is_freeze_deadlock<S>(
    map: &Map,
    box_position: Point,
    box_positions: &HashSet<Point, S>,
) -> bool
where
    S: BuildHasher + Default,
{
    debug_assert!(box_positions.contains(&box_position));
    if !is_frozen(map, box_position, box_positions) {
        return false;
    }

    let mut group = FxHashSet::default();
    let mut stack = vec![box_position];
    while let Some(position) = stack.pop() {
        if !group.insert(position) {
            continue;
        }
        for direction in Direction::iter() {
            let neighbor = position + direction.into();
            if box_positions.contains(&neighbor)
                && !group.contains(&neighbor)
                && is_frozen(map, neighbor, box_positions)
            {
                stack.push(neighbor);
            }
        }
    }

    group
        .iter()
        .any(|position| !map[*position].intersects(Tiles::Goal))
}

/// Checks if the given box position is frozen, regardless of goals.
#[must_use]
pub(crate) fn is_frozen<S>(
    map: &Map,
    box_position: Point,
    box_positions: &HashSet<Point, S>,
) -> bool
where
    S: BuildHasher + Default,
{
    is_frozen_inner(
        map,
        box_position,
        box_positions,
        &mut HashSet::<Point, S>::default(),
    )
}

#[must_use]
fn is_frozen_inner<S>(
    map: &Map,
    box_position: Point,
    box_positions: &HashSet<Point, S>,
    visited: &mut HashSet<Point, S>,
) -> bool
where
    S: BuildHasher,
{
    if !visited.insert(box_position) {
        return true;
    }

    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .chunks(2)
    {
        let neighbors = [
            box_position + direction[0].into(),
            box_position + direction[1].into(),
        ];

        // Check if any immovable walls on the axis.
        if map[neighbors[0]].intersects(Tiles::Wall) || map[neighbors[1]].intersects(Tiles::Wall) {
            continue;
        }

        // Check if any immovable boxes on the axis.
        if (box_positions.contains(&neighbors[0])
            && is_frozen_inner(map, neighbors[0], box_positions, visited))
            || (box_positions.contains(&neighbors[1])
                && is_frozen_inner(map, neighbors[1], box_positions, visited))
        {
            continue;
        }

        return false;
    }
    true
}

/// Computes the positions from which a box can never be pushed to a goal.
///
/// # Time complexity
///
/// `O(N)` where `N` is the number of cells in the map.
#[must_use]
pub fn compute_dead_positions(map: &Map) -> FxHashSet<Point> {
    let min_costs = Context::compute_minimum_push(map);
    let mut dead_positions = FxHashSet::default();
    for x in 0..map.dimensions().x {
        for y in 0..map.dimensions().y {
            let position = Point::new(x, y);
            if map[position].intersects(Tiles::Floor) && !min_costs.contains_key(&position) {
                dead_positions.insert(position);
            }
        }
    }
    dead_positions
}

/// Computes the positions of the useless floors.
///
/// # Time complexity
///
/// `O(N)` where `N` is the number of cells in the map.
#[must_use]
pub fn compute_useless_floors(mut map: Map) -> FxHashSet<Point> {
    let mut useless_floors = FxHashSet::default();

    // Add all floors to `unchecked_floors`
    let mut unchecked_floors = VecDeque::new();
    for y in 1..map.dimensions().y - 1 {
        for x in 1..map.dimensions().x - 1 {
            let position = Point::new(x, y);
            if map[position] == Tiles::Floor {
                unchecked_floors.push_back(position);
            }
        }
    }

    while let Some(position) = unchecked_floors.pop_front() {
        // Check if the current floor is in a dead end and store the exit
        // position in `neighbor_floor`
        let mut neighbor_floor = None;
        for direction in Direction::iter() {
            let neighbor = position + direction.into();
            if !map[neighbor].intersects(Tiles::Wall) {
                if neighbor_floor.is_some() {
                    neighbor_floor = None;
                    break;
                }
                neighbor_floor = Some(neighbor);
            }
        }
        // If the current floor is in a dead end
        if let Some(free_neighbor) = neighbor_floor {
            useless_floors.insert(position);
            map[position].remove(Tiles::Floor);
            map[position].insert(Tiles::Wall);
            // As the only floor affected by terrain changes, `free_neighbor`
            // may become a new unused floor and needs to be
            // rechecked.
            if map[free_neighbor] == Tiles::Floor && !map[free_neighbor].intersects(Tiles::Wall) {
                unchecked_floors.push_back(free_neighbor);
            }
        }
    }

    useless_floors
}

/// Computes the positions of the frozen boxes.
///
/// A box is frozen when both of its axes are blocked by walls or other
/// frozen boxes, which means it can never be pushed again.
///
/// # Time complexity
///
/// Typically `O(B)` where `B` is the number of boxes. In the worst case
/// this may degrade to `O(B²)`.
#[must_use]
pub(crate) fn compute_frozen_boxes(map: &Map) -> FxHashSet<Point> {
    map.box_positions()
        .iter()
        .copied()
        .filter(|&position| is_frozen(map, position, map.box_positions()))
        .collect()
}
