//! A sequence of actions.

use std::{
    fmt,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use nalgebra::Vector2;

use crate::{action::Action, error::ParseActionsError, run_length::rle_decode};

/// Secondary statistics for a sequence of actions.
pub struct SecondaryValues {
    /// Number of box lines.
    pub box_lines: i32,
    /// Number of times the pushed box changed (first push counts).
    pub box_changes: i32,
    /// Number of pushing sessions (transitions from moving to pushing).
    pub pushing_sessions: i32,
    /// Number of player lines (direction changes while moving).
    pub player_lines: i32,
}

/// A owned, mutable actions (akin to [`Vec<Action>`]).
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Actions(pub Vec<Action>);

impl Actions {
    /// Creates an empty actions.
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns the number of moves.
    pub fn moves(&self) -> usize {
        self.len()
    }

    /// Returns the number of pushes.
    pub fn pushes(&self) -> usize {
        self.iter().filter(|action| action.is_push()).count()
    }

    /// Rotates all actions 90° clockwise.
    pub fn rotate(&mut self) {
        for action in &mut self.0 {
            *action = action.rotate();
        }
    }

    /// Flips all actions horizontally.
    pub fn flip_horizontal(&mut self) {
        for action in &mut self.0 {
            *action = action.flip_horizontal();
        }
    }

    /// Flips all actions vertically.
    pub fn flip_vertical(&mut self) {
        for action in &mut self.0 {
            *action = action.flip_vertical();
        }
    }

    /// Returns the secondary values.
    pub fn secondary_values(&self) -> SecondaryValues {
        let mut box_lines = 0;
        let mut box_changes = 0;
        let mut pushing_sessions = 0;
        let mut player_lines = 0;

        let mut player_position = Vector2::zeros();
        // Previous pushed box position
        let mut prev_box_position = None;
        let mut prev_action: Option<Action> = None;
        for action in &self.0 {
            player_position += &action.direction().into();
            if let Some(prev_action) = prev_action {
                if action.direction() != prev_action.direction() {
                    player_lines += 1;
                }
                if action.is_push() {
                    if *action != prev_action {
                        box_lines += 1;
                    }
                    if !prev_action.is_push() {
                        pushing_sessions += 1;
                    }
                    if let Some(prev_box_position) = prev_box_position {
                        if player_position != prev_box_position {
                            box_changes += 1;
                        }
                    } else {
                        box_changes += 1;
                    }
                    prev_box_position = Some(player_position + &action.direction().into());
                }
            } else {
                if action.is_push() {
                    box_lines += 1;
                    pushing_sessions += 1;
                }
                player_lines += 1;
            }
            prev_action = Some(*action);
        }
        SecondaryValues {
            box_lines,
            box_changes,
            pushing_sessions,
            player_lines,
        }
    }
}

impl FromStr for Actions {
    type Err = ParseActionsError;

    /// Creates a new `Actions` with LURD format string.
    fn from_str(lurd: &str) -> Result<Self, Self::Err> {
        if lurd.contains(char::is_numeric) {
            return Actions::from_str(&rle_decode(lurd)?);
        }
        let mut instance = Actions::default();
        for char in lurd.chars() {
            instance.push(Action::try_from(char)?);
        }
        Ok(instance)
    }
}

impl Deref for Actions {
    type Target = Vec<Action>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Actions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Actions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for action in &self.0 {
            write!(f, "{action}")?;
        }
        Ok(())
    }
}
