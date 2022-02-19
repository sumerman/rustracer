use std::ops::Range;

use crate::math::*;

use super::Aabb;
use super::Hittable;

pub struct Bvh<T: Hittable> {
    objects: Vec<T>,
    nodes: BvhNode,
}

#[derive(Debug, Clone)]
enum BvhNode {
    Node {
        aabb: Aabb,
        leafs_count: usize,
        children: Vec<BvhNode>,
    },
    Leaf {
        aabb: Aabb,
        object_idx: usize,
    },
}

impl BvhNode {
    fn new_node(children: Vec<BvhNode>) -> Self {
        let root_box = children
            .iter()
            .map(|x| x.aabb())
            .reduce(|a, b| a.surrounding_box(b))
            .unwrap_or_else(|| Aabb::infinite());

        let leafs_count = children.iter().fold(0, |acc, c| acc + c.objects_count());

        BvhNode::Node {
            children,
            leafs_count,
            aabb: root_box,
        }
    }

    fn objects_count(&self) -> usize {
        match self {
            Self::Leaf { .. } => 1,
            Self::Node { leafs_count, .. } => *leafs_count,
        }
    }

    fn aabb(&self) -> Aabb {
        match self {
            Self::Leaf { aabb, .. } => *aabb,
            Self::Node { aabb, .. } => *aabb,
        }
    }

    /*
    This function splits the BvhNode according to the Surface Area Heuristic (SAH)
    The key ideas are:
        - Probability of a ray hitting a node is proportional to its surface area
        - Cost of traversing a node depends on the number of objects in its leaves
        - To split a node, find the hyperplane that minimizes SA(L)*N(L) + SA(R)*N(R) where:
            - SA(L) and SA(R) are the surface areas of the AABBs that enclose objects whose 
              centroids are on the left/right of the split hyperplane.
            - N(L) and N(R) are the counts of objects left and right of the split hyperplane

    For details see:    
    - https://graphics.cg.uni-saarland.de/courses/cg1-2018/slides/Building_good_BVHs.pdf
    - https://www.cg.tuwien.ac.at/courses/Rendering/2020/slides/01_spatial_acceleration.pdf 
     */
    fn sah_sweep_split(mut self) -> Self {
        let mut nodes_to_split: Vec<&mut BvhNode> = vec![&mut self];
        loop {
            let current = match nodes_to_split.pop() {
                None => break,
                Some(cur) => cur,
            };
            if let BvhNode::Node { children, .. } = current {
                if children.len() < 3 {
                    continue;
                }
                // This loop does a bunch of allocations and re-sorts per axis,
                // but should do for now (100-1000 of objects)
                // For substantially larger scenes it'd be better to switch to binning.
                let (_sa, left, right) = (0..2)
                    .map(|axis| {
                        children.sort_unstable_by(|a, b| {
                            let al = a.aabb().doubled_centroid()[axis];
                            let bl = b.aabb().doubled_centroid()[axis];
                            al.partial_cmp(&bl).unwrap_or_else(|| {
                                if al.is_nan() && bl.is_nan() {
                                    core::cmp::Ordering::Equal
                                } else if al.is_nan() {
                                    core::cmp::Ordering::Greater
                                } else {
                                    core::cmp::Ordering::Less
                                }
                            })
                        });
                        // Merging an AABB with itself won't change its surface area
                        let mut aabb_acc = children.first().map(|c| c.aabb()).unwrap();
                        let mut n_acc = 0.0;
                        let mut sa_sums = vec![0.0; children.len()];
                        // sweep left-to-right and compute running
                        // weighted surface area sums SA(L)*N(L)
                        // of object i and the objects left of it.
                        for i in 0..children.len() {
                            n_acc += children[i].objects_count() as f32;
                            aabb_acc = aabb_acc.surrounding_box(children[i].aabb());
                            sa_sums[i] += aabb_acc.surface_area() * n_acc;
                        }
                        // Sweep right-to-left and compute running 
                        // weighted surface area sums 
                        // of objects right of an i-th object, 
                        // excluding the i-th object itself.
                        // SA(R)*N(R)
                        aabb_acc = children.last().map(|c| c.aabb()).unwrap();
                        n_acc = 0.0;
                        for i in (0..children.len()).rev() {
                            let j = i + 1;
                            if j < children.len() {
                                n_acc += children[j].objects_count() as f32;
                                aabb_acc = aabb_acc.surrounding_box(children[j].aabb());
                                // add SA of all objects to the right from the i-th place
                                sa_sums[i] += aabb_acc.surface_area() * n_acc;
                            }
                        }
                        // Find the split with the smallest (on this axis) 
                        // associated surface area sums 
                        let (pivot, &sa_lr) = sa_sums
                            .iter()
                            .enumerate()
                            .filter(|(_, &sa)| !sa.is_nan())
                            .min_by(|(_, sa1), (_, sa2)| sa1.partial_cmp(sa2).unwrap())
                            .unwrap();
                        // Since pivot is an index in the children array, it's always < children.len()
                        // Therefore pivot + 1 could be at most children.len().
                        // It will produce an empty right side though.
                        let (left, right) = children.split_at_mut(pivot + 1);
                        (sa_lr, left.to_vec(), right.to_vec())
                    })
                    // Find the best split across all 3 axes
                    .min_by(|(sa1, _, _), (sa2, _, _)| sa1.partial_cmp(sa2).unwrap())
                    .unwrap();

                if right.is_empty() {
                    // current node couldn't be split, leave it alone
                    // otherwise it'll cause an infinite loop
                    continue;
                }
                children.clear();
                children.push(BvhNode::new_node(left));
                children.push(BvhNode::new_node(right));
                nodes_to_split.extend(children.iter_mut());
            }
        }
        self
    }
}

impl<T: Hittable> Bvh<T> {
    pub fn new(scene_objects: Vec<T>, time_interval: Range<f32>) -> Self {
        let objects = scene_objects;
        let leafs: Vec<BvhNode> = objects
            .iter()
            .enumerate()
            .map(|(idx, o)| BvhNode::Leaf {
                aabb: o.bounding_box(time_interval.clone()),
                object_idx: idx,
            })
            .collect();

        Bvh {
            objects,
            nodes: BvhNode::new_node(leafs).sah_sweep_split(),
        }
    }
}

impl<T: Hittable> Hittable for Bvh<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<super::Hit> {
        let mut final_hit = None;
        let mut closest_hit = t_max;
        let mut nodes_to_try: Vec<&BvhNode> = vec![&self.nodes];

        loop {
            let current = match nodes_to_try.pop() {
                None => break,
                Some(cur) => cur,
            };
            match current {
                BvhNode::Node { aabb, children, .. } => {
                    if aabb.hit(r, t_min, closest_hit) {
                        nodes_to_try.extend(children.iter().map(|c| c))
                    }
                }
                BvhNode::Leaf { aabb, object_idx } => {
                    if aabb.hit(r, t_min, closest_hit) {
                        let object = &self.objects[*object_idx];
                        let current_hit = object.hit(r, t_min, closest_hit);
                        if let Some(ref hit) = current_hit {
                            closest_hit = hit.t;
                            final_hit = current_hit;
                        }
                    }
                }
            }
        }

        final_hit
    }

    fn bounding_box(&self, _time_interval: std::ops::Range<f32>) -> Aabb {
        self.nodes.aabb()
    }
}
