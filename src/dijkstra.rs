use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;

pub trait Value {
    type Id: Eq + Hash;

    fn id(&self) -> Self::Id;
}

pub struct Dijkstra<V, FS, FN>
where
    V: Value,
{
    unvisited: BinaryHeap<Node<V>>,
    visited: HashSet<V::Id>,
    success: FS,
    neighbors: FN,
}

impl<V, FS, FN, I> Dijkstra<V, FS, FN>
where
    V: Value,
    FS: FnMut(&V) -> bool,
    FN: FnMut(&V) -> I,
    I: IntoIterator<Item = (V, usize)>,
{
    pub fn new(start: V, success: FS, neighbors: FN) -> Self {
        let mut unvisited = BinaryHeap::new();
        unvisited.push(Node {
            value: start,
            cost: 0,
        });

        Self {
            unvisited,
            visited: HashSet::new(),
            success,
            neighbors,
        }
    }
}

impl<V, FS, FN, I> Iterator for Dijkstra<V, FS, FN>
where
    V: Value,
    FS: FnMut(&V) -> bool,
    FN: FnMut(&V) -> I,
    I: IntoIterator<Item = (V, usize)>,
{
    type Item = (V, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(Node { value, cost }) = self.unvisited.pop() {
            let id = value.id();
            if self.visited.contains(&id) {
                continue;
            }

            self.visited.insert(id);

            if (self.success)(&value) {
                return Some((value, cost));
            }

            for (neighbor, move_cost) in (self.neighbors)(&value) {
                if !self.visited.contains(&neighbor.id()) {
                    self.unvisited.push(Node {
                        value: neighbor,
                        cost: cost + move_cost,
                    });
                }
            }
        }

        None
    }
}

/// Newtype to allow sorting ascending by cost in the BinaryHeap
struct Node<T> {
    value: T,
    cost: usize,
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<T> Eq for Node<T> {}

impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
