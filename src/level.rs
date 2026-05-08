//! A level.

use std::{collections::BTreeMap, fmt, io::BufRead, str::FromStr};

use crate::{
    Action, Actions, FxHashSet,
    direction::Direction,
    error::{ActionError, ParseLevelError, ParseMapError},
    map::Map,
    path_finding::compute_reachable_area,
    point::Point,
    tiles::Tiles,
};

/// A level.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Level {
    map: Map,
    metadata: BTreeMap<String, String>,
    actions: Actions,
    undone_actions: Actions,
}

impl Level {
    /// Creates a new `Level` from map.
    pub fn from_map(map: Map) -> Self {
        Self {
            map,
            metadata: BTreeMap::new(),
            actions: Actions::default(),
            undone_actions: Actions::default(),
        }
    }

    /// Returns a reference to the map of the level.
    pub fn map(&self) -> &Map {
        &self.map
    }

    /// Returns a reference to the metadata of the level.
    pub fn metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    /// Returns a reference to the actions of the level.
    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    /// Returns a reference to the undone actions of the level.
    pub fn undone_actions(&self) -> &Actions {
        &self.undone_actions
    }

    /// Returns the current facing direction of the player.
    pub fn player_direction(&self) -> Option<Direction> {
        self.actions.last().map(Action::direction)
    }

    /// Moves the player in the specified direction.
    pub fn execute(&mut self, direction: Direction) -> Result<(), ActionError> {
        // If the next move is opposite to the last one, treat it as an undo.
        if self.actions.last() == Some(&Action::Move(-direction)) {
            #[expect(
                clippy::missing_panics_doc,
                reason = "infallible: at least one action exists"
            )]
            self.undo().unwrap();
            return Ok(());
        }

        let new_player_position = self.map.player_position() + &direction.into();
        if self.map[new_player_position].intersects(Tiles::Wall) {
            return Err(ActionError::MoveBlocked);
        }
        if self.map[new_player_position].intersects(Tiles::Box) {
            let new_box_position = new_player_position + &direction.into();
            if self.map[new_box_position].intersects(Tiles::Wall | Tiles::Box) {
                return Err(ActionError::PushBlocked);
            }
            self.map
                .set_box_position(new_player_position, new_box_position);
            self.actions.push(Action::Push(direction));
        } else {
            self.actions.push(Action::Move(direction));
        }
        self.map.set_player_position(new_player_position);
        self.undone_actions.clear();
        Ok(())
    }

    /// Moves the player through a sequence of directions.
    pub fn execute_batch<I: IntoIterator<Item = Direction>>(
        &mut self,
        directions: I,
    ) -> Result<(), ActionError> {
        for direction in directions {
            self.execute(direction)?;
        }
        Ok(())
    }

    /// Undoes the last action.
    pub fn undo(&mut self) -> Result<(), ActionError> {
        if let Some(last_action) = self.actions.pop() {
            if last_action.is_shift() {
                let box_position = self.map.player_position() + &last_action.direction().into();
                let prev_box_position = self.map.player_position();
                self.map.set_box_position(box_position, prev_box_position);
            }
            let prev_player_position = self.map.player_position() - &last_action.direction().into();
            self.map.set_player_position(prev_player_position);
            self.undone_actions.push(last_action);
            Ok(())
        } else {
            Err(ActionError::NoActions)
        }
    }

    /// Redoes the last action.
    pub fn redo(&mut self) -> Result<(), ActionError> {
        if let Some(last_undone_action) = self.undone_actions.pop() {
            let undone_actions = std::mem::take(&mut self.undone_actions);
            #[expect(
                clippy::missing_panics_doc,
                reason = "infallible: `undone_actions` contains only valid actions"
            )]
            self.execute(last_undone_action.direction()).unwrap();
            self.undone_actions = undone_actions;
            Ok(())
        } else {
            Err(ActionError::NoUndoneActions)
        }
    }

    /// Returns `true` if the level is solved.
    pub fn is_solved(&self) -> bool {
        self.map.box_positions() == self.map.goal_positions()
    }

    /// Rotates the level 90° clockwise.
    pub fn rotate_cw(&mut self) {
        self.map.rotate_cw();
        self.actions.rotate_cw();
        self.undone_actions.rotate_cw();
    }

    /// Rotates the level 90° counter-clockwise.
    pub fn rotate_ccw(&mut self) {
        self.map.rotate_ccw();
        self.actions.rotate_ccw();
        self.undone_actions.rotate_ccw();
    }

    /// Flips the level horizontally.
    pub fn flip_horizontal(&mut self) {
        self.map.flip_horizontal();
        self.actions.flip_horizontal();
        self.undone_actions.flip_horizontal();
    }

    /// Flips the level vertically.
    pub fn flip_vertical(&mut self) {
        self.map.flip_vertical();
        self.actions.flip_vertical();
        self.undone_actions.flip_vertical();
    }

    /// Returns the reachable area for the player.
    pub fn player_reachable_area(&self) -> FxHashSet<Point> {
        compute_reachable_area(self.map.player_position(), |position| {
            self.map.is_walkable(position)
        })
    }

    /// Lazily loads levels from an XSB format string.
    pub fn load_from_str(str: &str) -> impl Iterator<Item = Result<Self, ParseLevelError>> + '_ {
        Self::split_into_groups_from_str(str).map(Self::from_str)
    }

    /// Lazily loads levels from a buffer reader.
    pub fn load_from_reader<R: BufRead>(
        reader: R,
    ) -> impl Iterator<Item = Result<Self, ParseLevelError>> {
        Self::split_into_groups_from_reader(reader).map(|group| Self::from_str(&group))
    }

    /// Loads the nth level from an XSB format string.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn load_nth_from_str(str: &str, index: usize) -> Result<Self, ParseLevelError> {
        let group = Self::split_into_groups_from_str(str)
            .nth(index)
            .expect("level index out of bounds");
        Self::from_str(group)
    }

    /// Loads the nth level from a buffer reader.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn load_nth_from_reader<R: BufRead>(
        reader: R,
        index: usize,
    ) -> Result<Self, ParseLevelError> {
        let group = Self::split_into_groups_from_reader(reader)
            .nth(index)
            .expect("level index out of bounds");
        Self::from_str(&group)
    }

    /// Lazily splits text from a reader into groups separated by empty lines
    /// (excluding empty lines within block comment), and filter out groups
    /// without map data.
    ///
    /// # Panics
    ///
    /// Panics if `std::io::BufRead::read_line` on `reader` returns an error.
    pub fn split_into_groups_from_reader<R: BufRead>(reader: R) -> impl Iterator<Item = String> {
        reader.group().map(|group| group.unwrap())
    }

    /// Lazily and zero-copy splits a str into groups by empty lines (excluding
    /// empty lines within block comment), and filter out groups without map
    /// data.
    fn split_into_groups_from_str(str: &str) -> impl Iterator<Item = &str> + '_ {
        str.split(['\n', '|']).filter_map({
            let mut offset = 0;
            let mut len = 0;
            let mut in_block_comment = false;
            let mut has_map_data = false;
            move |line| {
                len += line.len() + 1;
                let trimmed_line = line.trim();
                if !in_block_comment {
                    if trimmed_line.is_empty() || offset + len == str.len() + 1 {
                        let group = &str[offset..offset + len - 1];
                        offset += len;
                        len = 0;
                        if group.is_empty() || !has_map_data {
                            return None;
                        }
                        has_map_data = false;
                        return Some(group);
                    }
                    if let Some(comment) = trimmed_line.to_lowercase().strip_prefix("comment:") {
                        if comment.trim_start().is_empty() {
                            // Enter block comment
                            in_block_comment = true;
                        }
                        return None;
                    }
                    if has_map_data {
                        return None;
                    }
                    if is_xsb_string(trimmed_line) {
                        has_map_data = true;
                    }
                } else if trimmed_line.to_lowercase().starts_with("comment-end") {
                    // Exit block comment
                    in_block_comment = false;
                }
                None
            }
        })
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.map)?;
        for key in self.metadata.keys() {
            let value = &self.metadata[key];
            if key == "comments" && value.lines().count() > 1 {
                writeln!(f, "comment:")?;
                for line in value.lines() {
                    writeln!(f, "{line}")?;
                }
                writeln!(f, "comment-end:")?;
                continue;
            }
            debug_assert!(
                !value.contains('\n'),
                "metadata value contains multiple line"
            );
            writeln!(f, "{key}: {value}")?;
        }
        Ok(())
    }
}

impl FromStr for Level {
    type Err = ParseLevelError;

    /// Creates a new `Level` from XSB format string.
    ///
    /// Reads level map and metadata from XSB formatted strings.
    fn from_str(xsb: &str) -> Result<Self, Self::Err> {
        let mut map_offset = 0;
        let mut map_len = 0;
        let mut metadata = BTreeMap::new();
        let mut comments = String::new();
        let mut in_block_comment = false;
        for line in xsb.split_inclusive(['\n', '|']) {
            if map_len == 0 {
                map_offset += line.len();
            }

            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
                continue;
            }

            // Parse comments
            if in_block_comment {
                if trimmed_line.to_lowercase().starts_with("comment-end") {
                    // Exit block comment
                    in_block_comment = false;
                } else {
                    comments += trimmed_line;
                    comments.push('\n');
                }
                continue;
            }
            if let Some(comment) = trimmed_line.strip_prefix(';') {
                comments += comment.trim_start();
                comments.push('\n');
                continue;
            }

            // Parse metadata
            if let Some((key, value)) = trimmed_line.split_once(':') {
                let key = key.trim().to_lowercase();
                let value = value.trim();

                if key == "comment" {
                    if value.is_empty() {
                        // Enter block comment
                        in_block_comment = true;
                    } else {
                        comments += value;
                        comments.push('\n');
                    }
                    continue;
                }

                if metadata.insert(key.clone(), value.to_string()).is_some() {
                    return Err(ParseLevelError::DuplicateMetadata(key));
                }
                continue;
            }

            // Discard line that are not map data (with RLE)
            if !is_xsb_string(trimmed_line) {
                if map_len != 0 {
                    return Err(ParseMapError::InvalidCharacter(
                        trimmed_line
                            .chars()
                            .find(|&c| !is_xsb_symbol_with_rle(c))
                            .unwrap(),
                    )
                    .into());
                }
                continue;
            }

            if map_len == 0 {
                map_offset -= line.len();
            }
            map_len += line.len();
        }
        if !comments.is_empty() {
            debug_assert!(!metadata.contains_key("comments"));
            metadata.insert("comments".to_string(), comments);
        }
        if in_block_comment {
            return Err(ParseLevelError::UnterminatedBlockComment);
        }
        if map_len == 0 {
            return Err(ParseLevelError::NoMap);
        }

        Ok(Self {
            map: Map::from_str(&xsb[map_offset..map_offset + map_len])?,
            metadata,
            actions: Actions::default(),
            undone_actions: Actions::default(),
        })
    }
}

impl From<Level> for Map {
    fn from(level: Level) -> Self {
        level.map
    }
}

#[derive(Debug)]
struct Group<B> {
    buf: B,
}

impl<B: BufRead> Iterator for Group<B> {
    type Item = std::io::Result<String>;

    fn next(&mut self) -> Option<std::io::Result<String>> {
        let mut buf = String::new();
        let mut in_block_comment = false;
        let mut has_map_data = false;
        loop {
            let mut line = String::new();
            match self.buf.read_line(&mut line) {
                Ok(0) => {
                    if buf.is_empty() {
                        return None;
                    }
                    return Some(Ok(buf));
                }
                Ok(_n) => {
                    let trimmed_line = line.trim();
                    buf += &line;
                    if !in_block_comment {
                        if trimmed_line.is_empty() {
                            if has_map_data {
                                return Some(Ok(buf));
                            }
                            buf.clear();
                            continue;
                        }
                        if let Some(comment) = trimmed_line.to_lowercase().strip_prefix("comment:")
                        {
                            if comment.trim_start().is_empty() {
                                // Enter block comment
                                in_block_comment = true;
                            }
                            continue;
                        }
                        if has_map_data {
                            continue;
                        }
                        if is_xsb_string(trimmed_line) {
                            has_map_data = true;
                        }
                    } else if trimmed_line.to_lowercase().starts_with("comment-end") {
                        // Exit block comment
                        in_block_comment = false;
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }
    }
}

trait GroupExt: BufRead {
    fn group(self) -> Group<Self>
    where
        Self: Sized,
    {
        Group { buf: self }
    }
}

impl<T: BufRead> GroupExt for T {}

fn is_xsb_string(str: &str) -> bool {
    str.chars().all(is_xsb_symbol)
        || (str.chars().all(is_xsb_symbol_with_rle) && str.chars().any(is_xsb_symbol))
}

const fn is_xsb_symbol(char: char) -> bool {
    matches!(char, ' ' | '-' | '_' | '#' | '$' | '.' | '@' | '*' | '+')
}

const fn is_xsb_symbol_with_rle(char: char) -> bool {
    is_xsb_symbol(char) || char::is_ascii_digit(&char) || char == '|'
}
