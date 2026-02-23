use std::collections::{HashMap, HashSet};

use nalgebra::Vector2;

use crate::direction::Direction;

/// A biconnected component graph.
pub struct BccGraph {
    /// Maps an undirected edge to its block ID.
    edge_blocks: HashMap<[Vector2<i32>; 2], usize>,
    /// The set of cut vertices (articulation points) in the graph.
    cut_vertices: HashSet<Vector2<i32>>,
}

impl BccGraph {
    /// Creates a new `BccGraph`.
    pub fn new(start: Vector2<i32>, is_movable: impl Fn(Vector2<i32>) -> bool) -> Self {
        let mut edge_blocks = HashMap::new();
        let mut cut_vertices = HashSet::new();
        let mut depth = HashMap::new();
        let mut low = HashMap::new();
        let mut edge_stack = Vec::new();
        let mut block_count = 0;
        let mut time = 0;
        let mut root_children = 0;

        let mut stack = Vec::new();

        time += 1;
        depth.insert(start, time);
        low.insert(start, time);
        stack.push((start, None::<Vector2<i32>>, 0));

        let directions = Direction::iter().collect::<Vec<_>>();
        while let Some((u, p, direction_idx)) = stack.pop() {
            let mut next_direction_idx = direction_idx;
            let mut pushed_child = false;

            while next_direction_idx < directions.len() {
                let direction = directions[next_direction_idx];
                next_direction_idx += 1;

                let v = u + &direction.into();
                if !is_movable(v) {
                    continue;
                }
                if Some(v) == p {
                    continue;
                }

                if let Some(&v_depth) = depth.get(&v) {
                    let u_depth = depth[&u];
                    if v_depth < u_depth {
                        // Back edge
                        edge_stack.push(canonicalize_edge(u, v));
                        let low_u = low.get_mut(&u).unwrap();
                        *low_u = (*low_u).min(v_depth);
                    }
                } else {
                    if p.is_none() {
                        root_children += 1;
                    }

                    time += 1;
                    depth.insert(v, time);
                    low.insert(v, time);
                    edge_stack.push(canonicalize_edge(u, v));

                    stack.push((u, p, next_direction_idx));
                    stack.push((v, Some(u), 0));
                    pushed_child = true;
                    break;
                }
            }

            if pushed_child {
                continue;
            }

            if let Some(parent) = p {
                let low_u = low[&u];
                let low_p = low.get_mut(&parent).unwrap();
                *low_p = (*low_p).min(low_u);

                if low_u >= depth[&parent] {
                    if parent != start {
                        cut_vertices.insert(parent);
                    }
                    block_count += 1;
                    let target_edge = canonicalize_edge(parent, u);
                    while let Some(edge) = edge_stack.pop() {
                        edge_blocks.insert(edge, block_count);
                        if edge == target_edge {
                            break;
                        }
                    }
                }
            }
        }

        if root_children > 1 {
            cut_vertices.insert(start);
        }

        Self {
            edge_blocks,
            cut_vertices,
        }
    }

    /// Checks if a path exists from `from` to `to` without passing through the
    /// given `obstacle`.
    ///
    /// # Panics
    ///
    /// Panics if `from` and `to` are not adjacent to the `obstacle`, or are
    /// outside the movable area.
    pub fn is_reachable(
        &self,
        from: Vector2<i32>,
        to: Vector2<i32>,
        obstacle: Vector2<i32>,
    ) -> bool {
        debug_assert_eq!((from - obstacle).abs().sum(), 1);
        debug_assert_eq!((to - obstacle).abs().sum(), 1);

        // If the obstacle is not located at a cut vertex, it indicates that the entire
        // graph remains connected
        if !self.cut_vertices.contains(&obstacle) {
            return true;
        }

        let (edge1, edge2) = (
            canonicalize_edge(from, obstacle),
            canonicalize_edge(to, obstacle),
        );
        let (block1, block2) = (
            self.edge_blocks.get(&edge1).unwrap(),
            self.edge_blocks.get(&edge2).unwrap(),
        );

        block1 == block2
    }
}

/// Normalizes an undirected edge between two nodes.
fn canonicalize_edge(a: Vector2<i32>, b: Vector2<i32>) -> [Vector2<i32>; 2] {
    if (a.y, a.x) < (b.y, b.x) {
        [a, b]
    } else {
        [b, a]
    }
}
