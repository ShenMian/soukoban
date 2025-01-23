//! Utilities for deadlocks detection.

use std::collections::{HashSet, VecDeque};

use nalgebra::Vector2;

use crate::{direction::Direction, map::Map, tiles::Tiles};

/// Checks if the given box position is a static deadlock.
///
/// Consider using [`calculate_static_deadlocks`] if you need to efficiently
/// compute multiple static deadlock positions.
pub fn is_static_deadlock(
    map: &Map,
    box_position: Vector2<i32>,
    box_positions: &HashSet<Vector2<i32>>,
    visited: &mut HashSet<Vector2<i32>>,
) -> bool {
    debug_assert!(box_positions.contains(&box_position));

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
            box_position + &direction[0].into(),
            box_position + &direction[1].into(),
            box_position + &direction[2].into(),
        ];
        for neighbor in &neighbors {
            if map[*neighbor].intersects(Tiles::Wall) {
                continue;
            }
            if box_positions.contains(neighbor)
                && is_static_deadlock(map, *neighbor, box_positions, visited)
            {
                continue;
            }
            return false;
        }
    }
    true
}

/// Checks if the given box position is a freeze deadlock.
pub fn is_freeze_deadlock(
    map: &Map,
    box_position: Vector2<i32>,
    box_positions: &HashSet<Vector2<i32>>,
    visited: &mut HashSet<Vector2<i32>>,
) -> bool {
    debug_assert!(box_positions.contains(&box_position));

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
            box_position + &direction[0].into(),
            box_position + &direction[1].into(),
        ];

        // Check if any immovable walls on the axis.
        if map[neighbors[0]].intersects(Tiles::Wall) || map[neighbors[1]].intersects(Tiles::Wall) {
            continue;
        }

        // Check if any immovable boxes on the axis.
        if (box_positions.contains(&neighbors[0])
            && is_freeze_deadlock(map, neighbors[0], box_positions, visited))
            || (box_positions.contains(&neighbors[1])
                && is_freeze_deadlock(map, neighbors[1], box_positions, visited))
        {
            continue;
        }

        return false;
    }
    true
}

/// Calculates static deadlock positions independent of the player's position.
///
/// This function returns an **incomplete** set of dead positions independent
/// of the player's position. Any box pushed to a point in the set will cause a
/// deadlock, regardless of the player's position.
pub fn calculate_static_deadlocks(map: &Map) -> HashSet<Vector2<i32>> {
    let mut dead_positions = HashSet::new();
    for x in 1..map.dimensions().x - 1 {
        for y in 1..map.dimensions().y - 1 {
            let position = Vector2::new(x, y);
            // Check if current position may be a new corner
            if !map[position].intersects(Tiles::Floor) || map[position].intersects(Tiles::Goal) {
                continue;
            }
            for directions in [
                Direction::Up,
                Direction::Right,
                Direction::Down,
                Direction::Left,
                Direction::Up,
            ]
            .windows(2)
            {
                let neighbor = [
                    position + &directions[0].into(),
                    position + &directions[1].into(),
                ];

                // Check whether the current position is a corner
                if !(map[neighbor[0]].intersects(Tiles::Wall)
                    && map[neighbor[1]].intersects(Tiles::Wall))
                {
                    continue;
                }
                dead_positions.insert(position);

                // Detects grooves based on current position
                let mut potential_dead_positions = HashSet::new();
                let mut next_position = position - &(directions[0]).into();
                while map[next_position + &directions[1].into()].intersects(Tiles::Wall) {
                    if map[next_position].intersects(Tiles::Goal) {
                        break;
                    }
                    if map[next_position].intersects(Tiles::Wall) {
                        dead_positions.extend(potential_dead_positions);
                        break;
                    }
                    potential_dead_positions.insert(next_position);
                    next_position -= &(directions[0]).into();
                }
            }
        }
    }
    dead_positions
}

/// Calculate the positions of the useless floors.
pub fn calculate_useless_floors(mut map: Map) -> HashSet<Vector2<i32>> {
    let mut useless_floors = HashSet::new();

    // Add all floors to `unchecked_floors`
    let mut unchecked_floors = VecDeque::new();
    for y in 1..map.dimensions().y - 1 {
        for x in 1..map.dimensions().x - 1 {
            let position = Vector2::new(x, y);
            if map[position] == Tiles::Floor {
                unchecked_floors.push_back(position);
            }
        }
    }

    while let Some(position) = unchecked_floors.pop_front() {
        // Check if the current floor is in a dead end and store the exit position in
        // `neighbor_floor`
        let mut neighbor_floor = None;
        for direction in Direction::iter() {
            let neighbor = position + &direction.into();
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
            // As the only floor affected by terrain changes, `free_neighbor` may become a
            // new unused floor and needs to be rechecked.
            if map[free_neighbor] == Tiles::Floor && !map[free_neighbor].intersects(Tiles::Wall) {
                unchecked_floors.push_back(free_neighbor);
            }
        }
    }

    useless_floors
}

/// Calculate the positions of the useless boxes.
pub fn calculate_useless_boxes(map: &Map) -> HashSet<Vector2<i32>> {
    map.box_positions()
        .iter()
        .cloned()
        .filter(|&position| {
            is_freeze_deadlock(map, position, map.box_positions(), &mut HashSet::new())
        })
        .collect()
}
