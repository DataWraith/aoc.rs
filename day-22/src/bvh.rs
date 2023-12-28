use std::collections::BinaryHeap;
use std::{cmp::Reverse, ops::Range};

use indextree::{Arena, NodeId};
use utility_belt::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AABB {
    pub lower_bound: IVec3,
    pub upper_bound: IVec3,
}

impl AABB {
    pub fn union(&self, other: &Self) -> Self {
        let lower_bound = IVec3::new(
            self.lower_bound.x.min(other.lower_bound.x),
            self.lower_bound.y.min(other.lower_bound.y),
            self.lower_bound.z.min(other.lower_bound.z),
        );

        let upper_bound = IVec3::new(
            self.upper_bound.x.max(other.upper_bound.x),
            self.upper_bound.y.max(other.upper_bound.y),
            self.upper_bound.z.max(other.upper_bound.z),
        );

        Self {
            lower_bound,
            upper_bound,
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
        fn range_intersects(r1: Range<i32>, r2: Range<i32>) -> bool {
            r1.start >= r2.start && r1.start <= r2.end || r2.start >= r1.start && r2.start <= r1.end
        }

        let x_intersects = range_intersects(
            self.lower_bound.x..self.upper_bound.x,
            other.lower_bound.x..other.upper_bound.x,
        );

        let y_intersects = range_intersects(
            self.lower_bound.y..self.upper_bound.y,
            other.lower_bound.y..other.upper_bound.y,
        );

        let z_intersects = range_intersects(
            self.lower_bound.z..self.upper_bound.z,
            other.lower_bound.z..other.upper_bound.z,
        );

        x_intersects && y_intersects && z_intersects
    }

    pub fn volume(&self) -> i32 {
        let diff = self.upper_bound - self.lower_bound + IVec3::new(1, 1, 1);
        (diff.x * diff.y * diff.z).abs()
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

#[derive(Clone)]
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
        // If the tree is empty, the new node becomes the root
        if self.root.is_none() {
            let node = self.arena.new_node(aabb.clone());
            self.root = Some(node);
            return node;
        }

        // Otherwise, we look for the best sibling for the new node.
        let best_sibling = self.find_best_sibling(&aabb);

        // If the best sibling is the same as the node we're inserting, we're done.
        if self.arena[best_sibling].get() == &aabb {
            return best_sibling;
        }

        // Now we introduce a new node above the sibling. The sibling gets
        // re-parented to the new node, and the to-be-inserted node also becomes
        // a child of the new node.
        let new_node = self
            .arena
            .new_node(aabb.union(self.arena[best_sibling].get()));

        let parent = self.arena[best_sibling].parent();

        // Sever the tie between the sibling and its parent
        best_sibling.detach(&mut self.arena);

        new_node.append(best_sibling, &mut self.arena);
        new_node.append(self.arena.new_node(aabb.clone()), &mut self.arena);

        if let Some(parent) = parent {
            parent.append(new_node, &mut self.arena);
            self.update_bounding_boxes(new_node);
        } else {
            self.root = Some(new_node);
        }

        new_node
    }

    fn update_bounding_boxes(&mut self, start: NodeId) {
        let mut start = start;

        while let Some(cur) = self.arena[start].parent() {
            assert_eq!(cur.children(&self.arena).count(), 2);

            let child1 = self.arena[cur].first_child().unwrap();
            let child2 = self.arena[cur].last_child().unwrap();

            let new_aabb = self.arena[child1].get().union(self.arena[child2].get());

            if new_aabb == *self.arena[cur].get() {
                break;
            }

            *self.arena[cur].get_mut() = new_aabb;

            start = cur;
        }
    }

    fn find_best_sibling(&self, aabb: &AABB) -> NodeId {
        let mut q = BinaryHeap::new();

        let mut best_candidate = self.root.unwrap();
        let mut best_cost = aabb.union(self.arena[best_candidate].get()).volume();

        q.push((Reverse(best_cost), 0, best_candidate));

        while let Some((Reverse(cost), delta_cost, candidate)) = q.pop() {
            if cost < best_cost {
                best_cost = cost;
                best_candidate = candidate;
            }

            let direct_cost = self.arena[candidate].get().union(aabb).volume();
            let cur_delta = direct_cost - self.arena[candidate].get().volume();

            let lower_bound = aabb.volume() + cur_delta + delta_cost;

            if lower_bound < best_cost {
                if let Some(left) = self.arena[candidate].first_child() {
                    let left_cost =
                        self.arena[left].get().union(aabb).volume() + delta_cost + cur_delta;
                    q.push((Reverse(left_cost), delta_cost + cur_delta, left));
                }

                if let Some(right) = self.arena[candidate].last_child() {
                    let right_cost =
                        self.arena[right].get().union(aabb).volume() + delta_cost + cur_delta;
                    q.push((Reverse(right_cost), delta_cost + cur_delta, right));
                }
            }
        }

        let actual_best = self
            .root
            .unwrap()
            .descendants(&self.arena)
            .map(|n| {
                self.arena[n].get().union(aabb).volume() as i32
                    + n.ancestors(&self.arena)
                        .skip(1)
                        .map(|a| {
                            self.arena[a].get().union(&aabb).volume() - self.arena[a].get().volume()
                        })
                        .sum::<i32>()
            })
            .min()
            .unwrap();

        assert_eq!(best_cost, actual_best);

        best_candidate
    }

    pub fn get(&self, aabb: &AABB) -> Option<AABB> {
        if let Some(node_id) = self.get_f(aabb, |leaf: &AABB| leaf == aabb) {
            return self.arena[node_id].get().clone().into();
        }

        None
    }

    pub fn intersects_any(&self, aabb: &AABB) -> bool {
        !self.all_intersecting_leaves(aabb).is_empty()
    }

    pub fn all_intersecting_leaves(&self, aabb: &AABB) -> Vec<AABB> {
        let mut leaves = Vec::new();
        let mut queue = VecDeque::new();

        if self.root.is_none() {
            return leaves;
        }

        queue.push_back(self.root.unwrap());

        while let Some(cur) = queue.pop_front() {
            let child1 = self.arena[cur].first_child();
            let child2 = self.arena[cur].last_child();

            if child1.is_none() || child2.is_none() {
                leaves.push(self.arena[cur].get().clone());
                continue;
            }

            let child1 = child1.unwrap();
            let child2 = child2.unwrap();

            if self.arena[child1].get().intersects(aabb) {
                queue.push_back(child1);
            }

            if self.arena[child2].get().intersects(aabb) {
                queue.push_back(child2);
            }
        }

        leaves
    }

    // This walks the tree from the root until a leaf is found. The given test function
    // is applied to the leaf's AABB. If the test function returns true, the node ID
    // of the leaf is returned. Otherwise, None is returned.
    fn get_f(&self, aabb: &AABB, test: impl Fn(&AABB) -> bool) -> Option<NodeId> {
        let cur = self.root?;
        let mut queue = VecDeque::from(vec![cur]);

        while let Some(cur) = queue.pop_front() {
            let child1 = self.arena[cur].first_child();
            let child2 = self.arena[cur].last_child();

            if child1.is_none() || child2.is_none() {
                if test(self.arena[cur].get()) {
                    return Some(cur);
                }

                return None;
            }

            let child1 = child1.unwrap();
            let child2 = child2.unwrap();

            let child1_aabb = self.arena[child1].get();
            let child2_aabb = self.arena[child2].get();

            if child1_aabb.contains(aabb) {
                queue.push_back(child1);
            }

            if child2_aabb.contains(aabb) {
                queue.push_back(child2);
            }
        }

        None
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

        self.update_bounding_boxes(parent);

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
