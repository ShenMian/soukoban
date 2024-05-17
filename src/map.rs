//! A grid-based map.

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
    hash::{DefaultHasher, Hash, Hasher},
    ops::{Index, IndexMut},
    str::FromStr,
};

use nalgebra::Vector2;

use crate::{
    actions::Actions, deadlock::*, direction::Direction, error::ParseMapError, level::Level,
    path_finding::*, run_length::rle_decode, tiles::Tiles,
};

/// A grid-based map.
///
/// Map is used to store the map data of the Sokoban level, which is saved in a
/// one-bit array and can be accessed through two-dimensional coordinates. Some
/// data (such as the locations of all boxes) will be additionally stored in
/// other data structures to speed up certain operations.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Map {
    data: Vec<Tiles>,
    dimensions: Vector2<i32>,

    player_position: Vector2<i32>,
    box_positions: HashSet<Vector2<i32>>,
    goal_positions: HashSet<Vector2<i32>>,
}

impl Map {
    /// Creates a new `Map` from actions.
    ///
    /// Try to restore the map with a complete solution. This method can only
    /// restore the parts of the map that are used by the solution.
    pub fn from_actions(actions: &Actions) -> Result<Self, ParseMapError> {
        let mut min_position = Vector2::<i32>::zeros();
        let mut max_position = Vector2::<i32>::zeros();

        // Calculate the dimensions of the player's movement range
        let mut player_position = Vector2::zeros();
        for action in &**actions {
            player_position += &action.direction().into();
            min_position = min_position.zip_map(&player_position, |a, b| a.min(b));
            max_position = max_position.zip_map(&player_position, |a, b| a.max(b));
        }

        // Reserve space for walls
        min_position -= Vector2::new(1, 1);
        max_position += Vector2::new(1, 1);

        if min_position.x < 0 {
            player_position.x = min_position.x.abs();
        }
        if min_position.y < 0 {
            player_position.y = min_position.y.abs();
        }

        let dimensions = min_position.abs() + max_position.abs() + Vector2::new(1, 1);
        let mut instance = Map::with_dimensions(dimensions);

        let mut initial_box_positions = HashSet::new();
        let mut current_box_positions = HashSet::new();
        let mut current_player_position = player_position;
        for action in &**actions {
            instance[current_player_position] = Tiles::Floor;
            current_player_position += &action.direction().into();
            if action.is_push() {
                // The player pushed the box when moving, which means there is a box at the
                // player's current position
                if !current_box_positions.contains(&current_player_position) {
                    current_box_positions.insert(current_player_position);
                    initial_box_positions.insert(current_player_position);
                }
                current_box_positions.remove(&current_player_position);
                current_box_positions.insert(current_player_position + &action.direction().into());
            }
        }
        instance[current_player_position] = Tiles::Floor;

        // The initial position of boxes are the box positions, and the current (final)
        // position of boxes are the goal positions
        let box_positions = initial_box_positions;
        let goal_positions = current_box_positions;
        if box_positions.is_empty() {
            return Err(ParseMapError::NoBoxOrGoal);
        }

        instance[player_position].insert(Tiles::Player);
        for box_position in &box_positions {
            instance[*box_position].insert(Tiles::Box);
        }
        for goal_position in &goal_positions {
            instance[*goal_position].insert(Tiles::Goal);
        }

        instance.add_walls_around_floors();

        instance.player_position = player_position;
        instance.box_positions = box_positions;
        instance.goal_positions = goal_positions;

        // Verify solution
        let mut level = Level::from_map(instance.clone());
        for action in &**actions {
            level
                .do_action(action.direction())
                .map_err(|_| ParseMapError::InvalidActions)?;
        }

        Ok(instance)
    }

    /// Creates a new, empty `Map` with the specified dimensions.
    ///
    /// Warning: This will create an invalid map.
    pub fn with_dimensions(dimensions: Vector2<i32>) -> Self {
        Self {
            data: vec![Tiles::empty(); (dimensions.x * dimensions.y) as usize],
            dimensions,
            player_position: Vector2::zeros(),
            box_positions: HashSet::new(),
            goal_positions: HashSet::new(),
        }
    }

    /// Returns the dimensions of the map.
    pub fn dimensions(&self) -> Vector2<i32> {
        self.dimensions
    }

    /// Returns the position of the player.
    pub fn player_position(&self) -> Vector2<i32> {
        self.player_position
    }

    /// Sets the position of the player.
    pub fn set_player_position(&mut self, position: Vector2<i32>) {
        self.index_mut(self.player_position).remove(Tiles::Player);
        self[position].insert(Tiles::Player);
        self.player_position = position;
    }

    /// Returns a reference to the positions of the boxes.
    pub fn box_positions(&self) -> &HashSet<Vector2<i32>> {
        &self.box_positions
    }

    /// Returns a reference to the positions of the goals.
    pub fn goal_positions(&self) -> &HashSet<Vector2<i32>> {
        &self.goal_positions
    }

    /// Sets a box position from one to another.
    pub fn set_box_position(&mut self, from: Vector2<i32>, to: Vector2<i32>) {
        self.remove_box_position(from);
        self.add_box_position(to);
    }

    /// Normalizes the map.
    ///
    /// Remove elements from the map that are not relevant to the solution.
    /// The map's solution will not change.
    ///
    /// This method can make different maps with the same solution more similar.
    /// Therefore, it can be used for map deduplication.
    pub fn normalize(&mut self) {
        self.set_immovable_boxes_to_walls();
        self.set_unused_floors_to_walls();
        self.remove_unreachable_walls();
        self.remove_unreachable_boxes();
        self.shrink_to_fit();
        self.normalize_transformation();
    }

    /// Shrinks the dimensions of the map by trims the empty area around the
    /// map.
    pub fn shrink_to_fit(&mut self) {
        let mut new_dimensions = self.dimensions;
        let mut offset = Vector2::new(0, 0);

        // Trim top empty rows and bottom empty rows
        let is_row_empty = |y| {
            let mut row = (0..self.dimensions.x).map(|x| self[Vector2::new(x, y)]);
            row.all(|tiles| tiles.is_empty())
        };
        for y in 0..self.dimensions.y {
            if is_row_empty(y) {
                offset.y += 1;
                new_dimensions.y -= 1;
            } else {
                break;
            }
        }
        debug_assert_ne!(new_dimensions.y, 0);
        for y in (0..self.dimensions.y).rev() {
            if is_row_empty(y) {
                new_dimensions.y -= 1;
            } else {
                break;
            }
        }

        // Trim left empty columns and right empty columns
        let is_column_empty = |x| {
            let mut column = (0..self.dimensions.y).map(|y| self[Vector2::new(x, y)]);
            column.all(|tiles| tiles.is_empty())
        };
        for x in 0..self.dimensions.x {
            if is_column_empty(x) {
                offset.x += 1;
                new_dimensions.x -= 1;
            } else {
                break;
            }
        }
        for x in (0..self.dimensions.x).rev() {
            if is_column_empty(x) {
                new_dimensions.x -= 1;
            } else {
                break;
            }
        }

        self.truncate(new_dimensions, offset);
    }

    /// Truncates the map to the provided dimensions and copies tiles from the
    /// original map start at the specified offset to the new map.
    pub fn truncate(&mut self, new_dimensions: Vector2<i32>, offset: Vector2<i32>) {
        let mut clamped_map = Map::with_dimensions(new_dimensions);
        for y in 0..new_dimensions.y {
            for x in 0..new_dimensions.x {
                let position = Vector2::new(x, y);
                clamped_map[position] = self[position + offset];
            }
        }
        self.data = clamped_map.data;
        self.dimensions = clamped_map.dimensions;

        self.player_position -= offset;
        self.box_positions = self
            .box_positions
            .iter()
            .map(|position| position - offset)
            .collect();
        self.goal_positions = self
            .goal_positions
            .iter()
            .map(|position| position - offset)
            .collect();
    }

    /// Returns tiles at the specified position or `None` if out of bounds.
    pub fn get(&self, position: Vector2<i32>) -> Option<&Tiles> {
        self.data
            .get((position.y * self.dimensions.x + position.x) as usize)
    }

    /// Returns a mutable reference to tiles at the specified position or `None`
    /// if out of bounds.
    pub fn get_mut(&mut self, position: Vector2<i32>) -> Option<&mut Tiles> {
        self.data
            .get_mut((position.y * self.dimensions.x + position.x) as usize)
    }

    /// Returns tiles at the specified position, without doing bounds checking.
    ///
    /// For a safe alternative see [`get`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds position is *[undefined
    /// behavior]* even if the resulting reference is not used.
    ///
    /// [`get`]: Map::get
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn get_unchecked(&self, position: Vector2<i32>) -> &Tiles {
        self.data
            .get_unchecked((position.y * self.dimensions.x + position.x) as usize)
    }

    /// Returns a mutable reference to tiles at the specified position, without
    /// doing bounds checking.
    ///
    /// For a safe alternative see [`get_mut`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds position is *[undefined
    /// behavior]* even if the resulting reference is not used.
    ///
    /// [`get_mut`]: Map::get_mut
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    pub unsafe fn get_unchecked_mut(&mut self, position: Vector2<i32>) -> &mut Tiles {
        self.data
            .get_unchecked_mut((position.y * self.dimensions.x + position.x) as usize)
    }

    /// Checks if a position is within the bounds of the map.
    pub fn in_bounds(&self, position: Vector2<i32>) -> bool {
        0 <= position.x
            && position.x < self.dimensions.x
            && 0 <= position.y
            && position.y < self.dimensions.y
    }

    /// Checks if a position is traversable.
    pub fn can_move(&self, position: Vector2<i32>) -> bool {
        self.in_bounds(position) && !self[position].intersects(Tiles::Wall | Tiles::Box)
    }

    /// Rotates the map 90° clockwise.
    pub fn rotate(&mut self) {
        let dimensions = self.dimensions;
        let rotate_position =
            |position: Vector2<i32>| Vector2::new(position.y, dimensions.x - 1 - position.x);
        self.transform(rotate_position, self.dimensions.yx());
    }

    /// Flips the map horizontally.
    pub fn flip(&mut self) {
        let dimensions = self.dimensions;
        let flip_position =
            |position: Vector2<i32>| Vector2::new(dimensions.x - 1 - position.x, position.y);
        self.transform(flip_position, self.dimensions);
    }

    /// Adds a box at the given position.
    fn add_box_position(&mut self, position: Vector2<i32>) {
        debug_assert!(
            !self.box_positions.contains(&position),
            "box position already exists"
        );
        self[position].insert(Tiles::Box);
        self.box_positions.insert(position);
    }

    /// Removes a box at the given position.
    fn remove_box_position(&mut self, position: Vector2<i32>) {
        debug_assert!(
            self.box_positions.contains(&position),
            "box position does not exist"
        );
        self[position].remove(Tiles::Box);
        self.box_positions.remove(&position);
    }

    /// Removes a goal at the given position.
    fn remove_goal_position(&mut self, position: Vector2<i32>) {
        debug_assert!(
            self.goal_positions.contains(&position),
            "goal position does not exist"
        );
        self[position].remove(Tiles::Goal);
        self.goal_positions.remove(&position);
    }

    /// Sets unused floors to walls.
    fn set_unused_floors_to_walls(&mut self) {
        for unused_floor in calculate_unused_floors(self.clone()) {
            self[unused_floor].remove(Tiles::Floor);
            self[unused_floor].insert(Tiles::Wall);
        }
    }

    /// Sets immovable boxes to walls.
    fn set_immovable_boxes_to_walls(&mut self) {
        for position in self
            .box_positions
            .intersection(&self.goal_positions)
            .cloned()
            .collect::<HashSet<_>>()
        {
            // If the current box is deadlocked
            if is_freeze_deadlock(self, position, &self.box_positions, &mut HashSet::new()) {
                debug_assert!(
                    self[position].contains(Tiles::Box | Tiles::Goal),
                    "map has no solution"
                );
                self.remove_goal_position(position);
                self.remove_box_position(position);
                self[position].remove(Tiles::Floor);
                self[position].insert(Tiles::Wall);
            }
        }
    }

    /// Removes unused walls.
    fn remove_unreachable_walls(&mut self) {
        self.data
            .iter_mut()
            .for_each(|tiles| tiles.remove(Tiles::Wall));
        self.add_walls_around_floors();
    }

    /// Removes unreachable boxes and goals.
    fn remove_unreachable_boxes(&mut self) {
        for position in self
            .box_positions
            .iter()
            .filter(|position| !self[**position].intersects(Tiles::Floor))
            .cloned()
            .collect::<HashSet<_>>()
        {
            debug_assert!(
                self[position].contains(Tiles::Box | Tiles::Goal),
                "map has no solution"
            );
            self.remove_goal_position(position);
            self.remove_box_position(position);
        }
    }

    /// Normalizes the transformation of the map.
    fn normalize_transformation(&mut self) {
        let mut transformed_maps = HashMap::with_capacity(8);
        let mut min_hash = u64::MAX;
        for i in 0..8 {
            if i == 4 {
                self.flip();
            }
            self.rotate();
            self.normalize_player_position();

            let mut hasher = DefaultHasher::new();
            self.hash(&mut hasher);
            let hash = hasher.finish();

            transformed_maps.insert(hash, self.clone());
            min_hash = min_hash.min(hash);
        }
        *self = transformed_maps.remove(&min_hash).unwrap();
    }

    /// Normalizes the position of the player on the map.
    fn normalize_player_position(&mut self) {
        let player_reachable_area =
            reachable_area(self.player_position, |position| self.can_move(position));
        self.set_player_position(normalized_area(&player_reachable_area).unwrap());
    }

    /// Transforms the map based on the provided operation and new dimensions.
    fn transform(
        &mut self,
        operation: impl Fn(Vector2<i32>) -> Vector2<i32>,
        new_dimensions: Vector2<i32>,
    ) {
        let mut transformed_map = Map::with_dimensions(new_dimensions);
        for x in 0..self.dimensions.x {
            for y in 0..self.dimensions.y {
                let position = Vector2::new(x, y);
                let transformed_position = operation(position);
                transformed_map[transformed_position] = self[position];
            }
        }
        self.data = transformed_map.data;
        self.dimensions = transformed_map.dimensions;
        self.player_position = operation(self.player_position);
        self.box_positions = self
            .box_positions
            .iter()
            .map(|position| operation(*position))
            .collect();
        self.goal_positions = self
            .goal_positions
            .iter()
            .map(|position| operation(*position))
            .collect();
    }

    /// Add walls around floors.
    fn add_walls_around_floors(&mut self) {
        for x in 1..self.dimensions.x - 1 {
            for y in 1..self.dimensions.y - 1 {
                let position = Vector2::<i32>::new(x, y);
                if self[position].intersects(Tiles::Floor) {
                    let offsets = [
                        Vector2::<i32>::y(),
                        -Vector2::<i32>::y(),
                        Vector2::<i32>::x(),
                        -Vector2::<i32>::x(),
                        Vector2::<i32>::new(1, 1),
                        Vector2::<i32>::new(-1, -1),
                        Vector2::<i32>::new(1, -1),
                        Vector2::<i32>::new(-1, 1),
                    ];
                    for offset in offsets {
                        let neighbor = position + offset;
                        if !self[neighbor].intersects(Tiles::Floor) {
                            self[neighbor].insert(Tiles::Wall);
                        }
                    }
                }
            }
        }
    }

    /// Add floors to map with only walls.
    fn add_floors(&mut self, player_position: Vector2<i32>) {
        self.flood_fill(player_position, Tiles::Floor, Tiles::Wall);
    }

    /// Performs a flood fill algorithm starting from the specified position,
    /// updating the tiles with the value provided within the area surrounded by
    /// the provided border.
    fn flood_fill(&mut self, position: Vector2<i32>, value: Tiles, border: Tiles) {
        let mut deque = VecDeque::new();
        deque.push_back(position);
        while let Some(position) = deque.pop_front() {
            if !self.in_bounds(position) || self[position].intersects(value | border) {
                continue;
            }
            self[position].insert(value);
            for direction in Direction::iter() {
                let neighbor = position + &direction.into();
                deque.push_back(neighbor);
            }
        }
    }
}

impl FromStr for Map {
    type Err = ParseMapError;

    /// Creates a new `Map` from XSB format string.
    ///
    /// `Map` assumes that the map has a complete exterior wall and a solution.
    /// Some invalid maps will return [`Err`] when created.
    /// Returning [`Ok`] does not mean the map is fully valid, as it is
    /// difficult or even impossible to verify that the map is fully valid.
    fn from_str(xsb: &str) -> Result<Self, Self::Err> {
        debug_assert!(!xsb.trim().is_empty(), "string is empty");

        // Calculate map dimensions and indentation
        let mut indent = i32::MAX;
        let mut dimensions = Vector2::<i32>::zeros();
        let mut buffer = String::with_capacity(xsb.len());
        for line in xsb.split(['\n', '|']) {
            let mut line = line.trim_end().to_string();
            if line.is_empty() {
                continue;
            }
            // If the `line` contains digits, perform RLE decoding
            if line.chars().any(char::is_numeric) {
                line = rle_decode(&line)?;
            }
            dimensions.x = dimensions.x.max(line.len() as i32);
            dimensions.y += 1;
            indent = indent.min(line.chars().take_while(char::is_ascii_whitespace).count() as i32);
            buffer += &(line + "\n");
        }
        dimensions.x -= indent;

        let mut instance = Map::with_dimensions(dimensions);

        // Parse map data
        let mut player_position = None;
        for (y, line) in buffer.lines().enumerate() {
            // Trim map indentation
            let line = &line[indent as usize..];
            for (x, char) in line.chars().enumerate() {
                let position = Vector2::new(x as i32, y as i32);
                instance[position] = match char {
                    ' ' | '-' | '_' => Tiles::empty(),
                    '#' => Tiles::Wall,
                    '$' => {
                        instance.box_positions.insert(position);
                        Tiles::Box
                    }
                    '.' => {
                        instance.goal_positions.insert(position);
                        Tiles::Goal
                    }
                    '@' => {
                        if player_position.is_some() {
                            return Err(ParseMapError::MoreThanOnePlayer);
                        }
                        player_position = Some(position);
                        Tiles::Player
                    }
                    '*' => {
                        instance.box_positions.insert(position);
                        instance.goal_positions.insert(position);
                        Tiles::Box | Tiles::Goal
                    }
                    '+' => {
                        if player_position.is_some() {
                            return Err(ParseMapError::MoreThanOnePlayer);
                        }
                        player_position = Some(position);
                        instance.goal_positions.insert(position);
                        Tiles::Player | Tiles::Goal
                    }
                    _ => return Err(ParseMapError::InvalidCharacter(char)),
                };
            }
        }
        if instance.box_positions.len() != instance.goal_positions.len() {
            return Err(ParseMapError::BoxGoalMismatch);
        }
        if instance.box_positions.is_empty() {
            return Err(ParseMapError::NoBoxOrGoal);
        }
        if let Some(player_position) = player_position {
            instance.player_position = player_position;
        } else {
            return Err(ParseMapError::NoPlayer);
        }

        instance.add_floors(instance.player_position);

        Ok(instance)
    }
}

impl Index<Vector2<i32>> for Map {
    type Output = Tiles;

    fn index(&self, position: Vector2<i32>) -> &Tiles {
        assert!(0 <= position.x && position.x < self.dimensions.x && 0 <= position.y);
        &self.data[(position.y * self.dimensions.x + position.x) as usize]
    }
}

impl IndexMut<Vector2<i32>> for Map {
    fn index_mut(&mut self, position: Vector2<i32>) -> &mut Tiles {
        assert!(0 <= position.x && position.x < self.dimensions.x && 0 <= position.y);
        &mut self.data[(position.y * self.dimensions.x + position.x) as usize]
    }
}

impl Hash for Map {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.dimensions.y {
            for x in 0..self.dimensions.x {
                write!(f, "{}", self[Vector2::new(x, y)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
