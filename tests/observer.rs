use soukoban::prelude::{Actions, SearchError};
use soukoban::solver::*;

mod utils;
use utils::*;

#[derive(Default)]
struct Counter {
    expanded: u64,
    generated: u64,
}

impl Observer for Counter {
    fn on_node_expanded(&mut self, _node: &Node) {
        self.expanded += 1;
    }

    fn on_successor(&mut self, _parent: &Node, _child: &Node) {
        self.generated += 1;
    }
}

#[derive(Default)]
struct BestHeuristic {
    best: Option<i32>,
}

impl Observer for BestHeuristic {
    fn on_node_expanded(&mut self, node: &Node) {
        let h = node.heuristic();
        if self.best.is_none_or(|best| h < best) {
            self.best = Some(h);
        }
    }
}

#[test]
fn noop_observer_matches_plain_search() {
    let level = load_level_from_file("assets/BoxWorld_100.xsb", 1);
    let solver = Solver::new(level.map().clone(), Strategy::Quick);
    let plain = solver.search(Algorithm::AStar).unwrap();
    let mut noop = ();
    let observed = solver
        .search_with_observer(Algorithm::AStar, &mut noop)
        .unwrap();
    assert_eq!(plain, observed);
}

#[test]
fn expansion_hook_counts_nodes() {
    let level = load_level_from_file("assets/BoxWorld_100.xsb", 1);
    let solver = Solver::new(level.map().clone(), Strategy::Quick);
    let mut counter = Counter::default();
    let solution = solver
        .search_with_observer(Algorithm::AStar, &mut counter)
        .unwrap();
    assert!(!solution.is_empty());
    assert!(counter.expanded > 0);
    assert!(counter.generated >= counter.expanded);
}

#[test]
fn observers_compose_via_tuple() {
    let level = load_level_from_file("assets/BoxWorld_100.xsb", 1);
    let solver = Solver::new(level.map().clone(), Strategy::Quick);
    let mut combined = (BestHeuristic::default(), Counter::default());
    solver
        .search_with_observer(Algorithm::AStar, &mut combined)
        .unwrap();
    assert!(combined.0.best.is_some());
    assert!(combined.1.expanded > 0);
}

#[test]
fn on_search_end_receives_result() {
    struct Recorder {
        solved: Option<bool>,
        action_len: Option<usize>,
        error: Option<SearchError>,
    }
    impl Observer for Recorder {
        fn on_search_end(&mut self, result: &Result<Actions, SearchError>) {
            match result {
                Ok(actions) => {
                    self.solved = Some(true);
                    self.action_len = Some(actions.len());
                }
                Err(error) => {
                    self.solved = Some(false);
                    self.error = Some(error.clone());
                }
            }
        }
    }

    let level = load_level_from_file("assets/BoxWorld_100.xsb", 1);
    let solver = Solver::new(level.map().clone(), Strategy::Quick);
    let mut recorder = Recorder {
        solved: None,
        action_len: None,
        error: None,
    };
    let solution = solver
        .search_with_observer(Algorithm::AStar, &mut recorder)
        .unwrap();
    assert_eq!(recorder.solved, Some(true));
    assert_eq!(recorder.action_len, Some(solution.len()));
    assert_eq!(recorder.error, None);
}
