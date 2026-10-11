//! Observation hooks for the search.

use crate::{Actions, SearchError};

use super::node::Node;

/// Observer of the search process.
///
/// Hooks have empty default bodies; implement only what is needed. Hooks take
/// borrowed data, so observers clone only what they retain.
pub trait Observer {
    /// Called once with the initial node.
    fn on_search_start(&mut self, _root: &Node) {}

    /// Called when a node is expanded.
    fn on_node_expanded(&mut self, _node: &Node) {}

    /// Called for every generated successor, before duplicate filtering.
    fn on_successor(&mut self, _parent: &Node, _child: &Node) {}

    /// Called once with the final result.
    fn on_search_end(&mut self, _result: &Result<Actions, SearchError>) {}
}

/// No-op observer.
impl Observer for () {}

/// Forwards events to the referenced observer.
impl<T: Observer + ?Sized> Observer for &mut T {
    fn on_search_start(&mut self, root: &Node) {
        (**self).on_search_start(root);
    }

    fn on_node_expanded(&mut self, node: &Node) {
        (**self).on_node_expanded(node);
    }

    fn on_successor(&mut self, parent: &Node, child: &Node) {
        (**self).on_successor(parent, child);
    }

    fn on_search_end(&mut self, result: &Result<Actions, SearchError>) {
        (**self).on_search_end(result);
    }
}

/// Combines two observers.
impl<A: Observer, B: Observer> Observer for (A, B) {
    fn on_search_start(&mut self, root: &Node) {
        self.0.on_search_start(root);
        self.1.on_search_start(root);
    }

    fn on_node_expanded(&mut self, node: &Node) {
        self.0.on_node_expanded(node);
        self.1.on_node_expanded(node);
    }

    fn on_successor(&mut self, parent: &Node, child: &Node) {
        self.0.on_successor(parent, child);
        self.1.on_successor(parent, child);
    }

    fn on_search_end(&mut self, result: &Result<Actions, SearchError>) {
        self.0.on_search_end(result);
        self.1.on_search_end(result);
    }
}
