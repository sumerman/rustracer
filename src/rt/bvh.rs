use std::ops::Range;

use crate::math::*;

use super::Aabb;
use super::Hittable;

pub struct Bvh<T: Hittable> {
    objects: Vec<T>,
    nodes: BvhNode,
}

#[derive(Debug)]
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

    fn sah_sweep_split(mut self) -> Self {
        let mut nodes_to_split: Vec<&mut BvhNode> = vec![&mut self];
        loop {
            let current = match nodes_to_split.pop() {
                None => break,
                Some(cur) => cur,
            };
            match current {
                BvhNode::Leaf { .. } => {}
                BvhNode::Node { aabb, children, .. } => {
                    if children.len() < 3 {
                        continue;
                    }
                    let centroids = aabb.doubled_centroid();
                    // sweep only across the longes axis,
                    // it's faster and simplifies the code a lot
                    let axis = (0..2)
                        .map(|axis| (axis, centroids[axis]))
                        .filter(|c| !c.1.is_nan())
                        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                        .map(|(axis, _)| axis)
                        .unwrap_or(0 as usize);

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
                    // merging an aabb with itself won't change its SA
                    let mut aabb_acc = children.first().map(|c| c.aabb()).unwrap();
                    let mut n_acc = 0.0;
                    let mut sa_sums = vec![0.0; children.len()];
                    for i in 0..children.len() {
                        n_acc += children[i].objects_count() as f32;
                        aabb_acc = aabb_acc.surrounding_box(children[i].aabb());
                        sa_sums[i] += aabb_acc.surface_area() * n_acc;
                    }
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
                    let pivot = sa_sums
                        .iter()
                        .enumerate()
                        .filter(|(_, &sa)| !sa.is_nan())
                        .min_by(|(_, sa1), (_, sa2)| sa1.partial_cmp(sa2).unwrap())
                        .map(|(min_idx, _)| min_idx)
                        .unwrap();
                    // Since pivot is an index in the children array, it's always < children.len()
                    // Therefore pivot + 1 could be at most children.len().
                    // It will produce an empty right side though.
                    let right = children.split_off(pivot + 1);
                    if right.is_empty() {
                        // current node couldn't be split, leave it alone
                        // otherwise it'll cause an infinite loop
                        continue;
                    }
                    let right_node = BvhNode::new_node(right);
                    let left_node = BvhNode::new_node(children.drain(..).collect());
                    children.push(left_node);
                    children.push(right_node);
                    nodes_to_split.extend(children.iter_mut());
                }
            }
        }
        // println!("{:#?}", self);
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
                    if aabb.hit(r, t_min, t_max) {
                        nodes_to_try.extend(children.iter().map(|c| c))
                    }
                }
                BvhNode::Leaf { aabb, object_idx } => {
                    if aabb.hit(r, t_min, t_max) {
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
