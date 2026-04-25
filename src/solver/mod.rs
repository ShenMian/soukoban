//! A solver for the Sokoban problem.

mod context;
mod node;
mod search;
pub(crate) mod state;

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use crate::{Actions, Map, SearchError};

use context::Context;

/// The strategy to use when searching for a solution.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub enum Strategy {
    /// Search for any solution as quickly as possible.
    ///
    /// Using this strategy, A* search degrades into greedy best-first search.
    #[default]
    Quick,
    /// Find the push optimal solution.
    PushOptimal,
    /// Find the move optimal solution.
    MoveOptimal,
}

/// The algorithm to use when searching for a solution.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Algorithm {
    /// A* search. Fast, extremely high memory footprint.
    AStar,
    /// IDA* search. Slower, extremely low memory footprint.
    IDAStar,
    /// Breadth-first search. Slow, extremely high memory footprint.
    Bfs,
}

/// A solver for the Sokoban problem.
#[derive(Clone, Debug)]
pub struct Solver {
    /// Pre-computed context shared by search algorithms.
    ctx: Context,
    /// Flag to request stopping the search.
    stop_flag: Arc<AtomicBool>,
}

impl Solver {
    /// Creates a new `Solver`.
    pub fn new(map: Map, strategy: Strategy) -> Self {
        Self {
            ctx: Context::new(map, strategy),
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Searches for solution using the specified algorithm.
    pub fn search(&self, algorithm: Algorithm) -> Result<Actions, SearchError> {
        self.stop_flag.store(false, Ordering::Relaxed);
        match algorithm {
            Algorithm::AStar => search::a_star_search(&self.ctx, &self.stop_flag),
            Algorithm::IDAStar => search::ida_star_search(&self.ctx, &self.stop_flag),
            Algorithm::Bfs => search::bfs_search(&self.ctx, &self.stop_flag),
        }
    }

    /// Request to stop the ongoing search. This sets a shared flag checked by
    /// the search routines.
    pub fn request_stop(&self) {
        self.stop_flag.store(true, Ordering::Relaxed);
    }

    /// Returns a reference to the context.
    pub fn context(&self) -> &Context {
        &self.ctx
    }
}
