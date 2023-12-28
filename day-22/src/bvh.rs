use std::cmp::Reverse;
use std::collections::BinaryHeap;

use indextree::{Arena, NodeId};
use utility_belt::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AABB {
    pub lower_bound: IVec3,
    pub upper_bound: IVec3,
}

impl AABB {
    pub fn union(&self, other: &Self) -> Self {
        Self {
            lower_bound: self.lower_bound.min(other.lower_bound),
            upper_bound: self.upper_bound.max(other.upper_bound),
        }
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.lower_bound.x <= other.lower_bound.x
            && self.lower_bound.y <= other.lower_bound.y
            && self.lower_bound.z <= other.lower_bound.z
            && self.upper_bound.x >= other.upper_bound.x
            && self.upper_bound.y >= other.upper_bound.y
            && self.upper_bound.z >= other.upper_bound.z
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.lower_bound.x <= other.upper_bound.x
            && self.lower_bound.y <= other.upper_bound.y
            && self.lower_bound.z <= other.upper_bound.z
            && self.upper_bound.x >= other.lower_bound.x
            && self.upper_bound.y >= other.lower_bound.y
            && self.upper_bound.z >= other.lower_bound.z
    }

    pub fn volume(&self) -> i32 {
        let diff = self.upper_bound - self.lower_bound;
        diff.x * diff.y * diff.z
    }
}

impl std::fmt::Debug for AABB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}~{},{},{}",
            self.lower_bound.x,
            self.lower_bound.y,
            self.lower_bound.z,
            self.upper_bound.x,
            self.upper_bound.y,
            self.upper_bound.z
        )
    }
}

pub struct BVH {
    arena: Arena<AABB>,
    root: Option<NodeId>,
}

impl BVH {
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            root: None,
        }
    }

    pub fn insert(&mut self, aabb: AABB) -> NodeId {
        let node = self.arena.new_node(aabb.clone());

        // If the tree is empty, the new node becomes the root
        if self.root.is_none() {
            self.root = Some(node);
            return node;
        }

        // Otherwise, we look for the best sibling for the new node.
        let best_sibling = self.find_best_sibling(node);

        // If the best sibling is the same as the node we're inserting, we're done.
        if self.arena[best_sibling].get() == &aabb {
            return best_sibling;
        }

        // Now we introduce a new node above the sibling. The sibling gets
        // re-parented to the new node, and the to-be-inserted node also becomes
        // a child of the new node.
        let new_node = self
            .arena
            .new_node(self.arena[node].get().union(self.arena[best_sibling].get()));

        let parent = self.arena[best_sibling].parent();

        // Sever the tie between the sibling and its parent
        best_sibling.detach(&mut self.arena);

        new_node.append(best_sibling, &mut self.arena);
        new_node.append(node, &mut self.arena);

        if let Some(parent) = parent {
            parent.append(new_node, &mut self.arena);
        } else {
            self.root = Some(new_node);
            return new_node;
        }

        let cur = new_node;

        while let Some(cur) = self.arena[cur].parent() {
            let child1 = self.arena[cur].first_child().unwrap();
            let child2 = self.arena[cur].last_child().unwrap();

            let new_aabb = self.arena[child1].get().union(self.arena[child2].get());

            if new_aabb == *self.arena[cur].get() {
                break;
            }

            *self.arena[cur].get_mut() = new_aabb;
        }

        new_node
    }

    fn find_best_sibling(&self, node: NodeId) -> NodeId {
        let mut q = BinaryHeap::new();

        let mut best_candidate = self.root.unwrap();
        let mut best_cost = self.arena[best_candidate]
            .get()
            .union(self.arena[node].get())
            .volume();
        let delta_cost = best_cost - self.arena[best_candidate].get().volume();

        q.push((Reverse(best_cost), delta_cost, best_candidate));

        while let Some((Reverse(cost), inherited_cost, candidate)) = q.pop() {
            let direct_cost = self.arena[candidate]
                .get()
                .union(self.arena[node].get())
                .volume();
            let new_cost = inherited_cost + direct_cost;
            let delta_cost = new_cost - self.arena[candidate].get().volume();
            let lower_bound = self.arena[node].get().volume() + inherited_cost + delta_cost;

            if new_cost < best_cost {
                best_cost = new_cost;
                best_candidate = candidate;
            }

            if lower_bound < best_cost {
                if let Some(left) = self.arena[candidate].first_child() {
                    q.push((Reverse(new_cost), inherited_cost + delta_cost, left));
                }

                if let Some(right) = self.arena[candidate].last_child() {
                    q.push((Reverse(new_cost), delta_cost, right));
                }
            }
        }

        best_candidate
    }

    pub fn get(&self, aabb: &AABB) -> Option<AABB> {
        if let Some(node_id) = self.get_f(aabb, |leaf: &AABB| leaf == aabb) {
            return self.arena[node_id].get().clone().into();
        }

        None
    }

    pub fn intersects_any(&self, aabb: &AABB) -> bool {
        self.get_f(aabb, |leaf: &AABB| leaf.intersects(&aabb))
            .is_some()
    }

    // This walks the tree from the root until a leaf is found. The given test function
    // is applied to the leaf's AABB. If the test function returns true, the node ID
    // of the leaf is returned. Otherwise, None is returned.
    fn get_f(&self, aabb: &AABB, test: impl Fn(&AABB) -> bool) -> Option<NodeId> {
        let mut cur = self.root?;

        loop {
            let child1 = self.arena[cur].first_child();
            let child2 = self.arena[cur].last_child();

            // All interior nodes in the tree should have two children, so we
            // can check for just one of them here
            if child1.is_none() {
                if test(self.arena[cur].get()) {
                    return Some(cur);
                } else {
                    return None;
                }
            }

            let child1 = child1.unwrap();
            let child2 = child2.unwrap();

            let child1_aabb = self.arena[child1].get();
            let child2_aabb = self.arena[child2].get();

            // Descend into the child with the smaller enclosing bounding box
            if child1_aabb.contains(aabb) && !child2_aabb.contains(aabb) {
                cur = child1;
            } else if !child1_aabb.contains(aabb) && child2_aabb.contains(aabb) {
                cur = child2;
            } else if child1_aabb.contains(aabb) && child2_aabb.contains(aabb) {
                if child1_aabb.volume() < child2_aabb.volume() {
                    cur = child1;
                } else {
                    cur = child2;
                }
            } else {
                return None;
            }
        }
    }

    pub fn remove(&mut self, aabb: &AABB) -> bool {
        let node_id = self.get_f(aabb, |leaf: &AABB| leaf == aabb);

        if node_id.is_none() {
            return false;
        }

        let node_id = node_id.unwrap();

        let parent = self.arena[node_id].parent().unwrap();

        let sibling = if self.arena[parent].first_child() == Some(node_id) {
            self.arena[parent].last_child().unwrap()
        } else {
            self.arena[parent].first_child().unwrap()
        };

        self.arena[parent].get_mut().lower_bound = self.arena[sibling].get().lower_bound;
        self.arena[parent].get_mut().upper_bound = self.arena[sibling].get().upper_bound;

        sibling.remove(&mut self.arena);
        node_id.remove(&mut self.arena);

        true
    }

    pub fn root(&self) -> Option<NodeId> {
        self.root
    }
}

impl std::fmt::Debug for BVH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.root.is_none() {
            return write!(f, "Empty BVH");
        }

        let root = self.root.unwrap();

        writeln!(f, "\n{:?}", root.debug_pretty_print(&self.arena))
    }
}
