//! A sequence of actions.

use std::{
    fmt,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use nalgebra::Vector2;

use crate::{action::Action, error::ParseActionError, run_length::rle_decode};

/// A sequence of actions.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Actions(pub Vec<Action>);

impl Actions {
    /// Returns the total number of moves.
    pub fn moves(&self) -> usize {
        self.len()
    }

    /// Returns the total number of pushes.
    pub fn pushes(&self) -> usize {
        self.iter().filter(|x| x.is_push()).count()
    }

    /// Returns the secondary values.
    pub fn secondary_values(&self) -> (i32, i32, i32, i32) {
        let mut player_lines = 0;
        let mut box_lines = 0;
        let mut box_changes = 0;
        let mut pushing_sessions = 0;
        let mut player_position = Vector2::zeros();
        let mut prev_pushed_box_position = None;
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
                        if !prev_action.is_push() {
                            pushing_sessions += 1;
                        }
                    }
                    if let Some(prev_pushed_box_position) = prev_pushed_box_position {
                        if player_position != prev_pushed_box_position {
                            box_changes += 1;
                        }
                    }
                    prev_pushed_box_position = Some(player_position + &action.direction().into());
                }
            } else {
                player_lines += 1;
            }
            prev_action = Some(*action);
        }
        if prev_pushed_box_position.is_some() {
            box_changes += 1;
        }
        (box_lines, box_changes, pushing_sessions, player_lines)
    }
}

impl FromStr for Actions {
    type Err = ParseActionError;

    /// Creates a new `Actions` with LURD format string.
    fn from_str(lurd: &str) -> Result<Self, Self::Err> {
        if lurd.contains(char::is_numeric) {
            return Actions::from_str(&rle_decode(lurd).unwrap());
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
            write!(f, "{}", action)?;
        }
        Ok(())
    }
}
