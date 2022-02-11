use std::collections::HashSet;
use std::ops::Range;

use crate::math::*;

use super::Aabb;
use super::Hittable;

pub struct Bvh<T: Hittable> {
    objects: Vec<T>,
    descriptors: Vec<HittableDescriptor>,
    nodes: BvhNode,
}

#[derive(Clone, Debug)]
struct HittableDescriptor {
    object_idx: usize,
    aabb: Aabb,
}

#[derive(Clone, Debug)]
enum BvhNode {
    Node {
        aabb: Aabb,
        left: Box<BvhNode>,
        right: Box<BvhNode>,
    },
    Partition {
        aabb: Aabb,
        range: Range<usize>,
    },
}

impl BvhNode {
    fn aabb(&self) -> Aabb {
        match self {
            Self::Partition { aabb, .. } => *aabb,
            Self::Node { aabb, .. } => *aabb,
        }
    }

    fn new_partition(objects: &[&HittableDescriptor], range: Range<usize>) -> Self {
        let aabb = objects[range.start..range.end]
            .iter()
            .map(|x| x.aabb)
            .reduce(|a, b| a.surrounding_box(b))
            .unwrap_or_else(|| Aabb::infinite());

        Self::Partition { aabb, range }
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
    fn sah_sweep_build(objects: &mut [HittableDescriptor]) -> Self {
        let mut left_ids: HashSet<usize> = HashSet::new();
        let mut buffer: Vec<&HittableDescriptor> = Vec::with_capacity(objects.len());
        let mut sa_sums: Vec<f32> = Vec::with_capacity(objects.len());
        let mut axes: [Vec<&HittableDescriptor>; 3] = [
            objects.iter().collect(),
            objects.iter().collect(),
            objects.iter().collect(),
        ];

        // Sort descriptors along each of the axis according to their centroids
        (0..2).for_each(|axis| {
            axes[axis].sort_unstable_by(|a, b| {
                let al = a.aabb.doubled_centroid()[axis];
                let bl = b.aabb.doubled_centroid()[axis];
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
        });

        // TODO split off NaN-s into its own node

        let mut root_node = Self::new_partition(axes[0].as_slice(), 0..objects.len());
        let mut partitions_to_split: Vec<&mut Self> = vec![&mut root_node];
        loop {
            let current = match partitions_to_split.pop() {
                None => break,
                Some(cur) => cur,
            };
            if let BvhNode::Partition {
                range,
                aabb: full_box,
                ..
            } = current
            {
                if range.len() < 3 {
                    continue;
                }
                let (_sa, best_axis, pivot) = (0..2)
                    .map(|axis| {
                        let in_partition = &axes[axis][range.start..range.end];
                        sa_sums.resize(in_partition.len(), 0.0);
                        // Merging an AABB with itself won't change its surface area
                        let mut aabb_acc = in_partition.first().map(|c| c.aabb).unwrap();
                        let mut n_acc = 0.0;
                        // sweep left-to-right and compute running
                        // weighted surface area sums SA(L)*N(L)
                        // of object i and the objects left of it.
                        for i in 0..in_partition.len() {
                            n_acc += 1.0;
                            aabb_acc = aabb_acc.surrounding_box(in_partition[i].aabb);
                            sa_sums[i] += aabb_acc.surface_area() * n_acc;
                        }
                        // Sweep right-to-left and compute running
                        // weighted surface area sums
                        // of objects right of an i-th object,
                        // excluding the i-th object itself.
                        // SA(R)*N(R)
                        aabb_acc = in_partition.last().map(|c| c.aabb).unwrap();
                        n_acc = 0.0;
                        for i in (0..in_partition.len()).rev() {
                            let j = i + 1;
                            if j < in_partition.len() {
                                n_acc += 1.0;
                                aabb_acc = aabb_acc.surrounding_box(in_partition[j].aabb);
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
                        (sa_lr, axis, pivot)
                    })
                    // Find the best split across all 3 axes
                    .min_by(|(sa1, _, _), (sa2, _, _)| sa1.partial_cmp(sa2).unwrap())
                    .unwrap(); // there are exactly 3 elements, minimum exists

                if pivot + 1 >= range.len() {
                    // current node couldn't be split, leave it alone
                    // otherwise it'll cause an infinite loop
                    continue;
                }

                left_ids.clear();
                axes[best_axis][range.start..range.end]
                    .iter()
                    .enumerate()
                    .for_each(|(i, desc)| {
                        if i <= pivot {
                            left_ids.insert(desc.object_idx);
                        }
                    });

                axes.iter_mut().enumerate().for_each(|(axis_id, axis)| {
                    // The objects on the "best axis" are already partitioned the way we need.
                    // For all other axis we replicate this partitioning, while preserving relative order.
                    if axis_id != best_axis {
                        buffer.clear();
                        let partition = &mut axis[range.start..range.end];
                        partition.iter().for_each(|desc| {
                            if left_ids.contains(&desc.object_idx) {
                                buffer.push(desc)
                            }
                        });
                        partition.iter().for_each(|desc| {
                            if !left_ids.contains(&desc.object_idx) {
                                buffer.push(desc)
                            }
                        });
                        partition.swap_with_slice(buffer.as_mut_slice());
                    }
                });

                let pivot = range.start + pivot;
                let left_partition =
                    Self::new_partition(axes[0].as_slice(), range.start..(pivot + 1));
                let right_partition =
                    Self::new_partition(axes[0].as_slice(), (pivot + 1)..range.end);
                *current = Self::Node {
                    aabb: *full_box,
                    left: Box::new(left_partition),
                    right: Box::new(right_partition),
                };
            }
            if let Self::Node { left, right, .. } = current {
                partitions_to_split.push(left);
                partitions_to_split.push(right);
            }
        }

        let mut partitioned_objects: Vec<HittableDescriptor> =
            axes[0].iter().map(|desc| (*desc).clone()).collect();
        objects.swap_with_slice(partitioned_objects.as_mut_slice());

        root_node
    }
}

impl<T: Hittable> Bvh<T> {
    pub fn new(scene_objects: Vec<T>, time_interval: Range<f32>) -> Self {
        let objects = scene_objects;
        let mut descriptors: Vec<HittableDescriptor> = objects
            .iter()
            .enumerate()
            .map(|(idx, o)| HittableDescriptor {
                aabb: o.bounding_box(time_interval.clone()),
                object_idx: idx,
            })
            .collect();

        let root_node = BvhNode::sah_sweep_build(descriptors.as_mut_slice());

        Bvh {
            objects,
            descriptors,
            nodes: root_node,
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
                BvhNode::Node {
                    aabb, left, right, ..
                } => {
                    if aabb.hit(r, t_min, t_max) {
                        nodes_to_try.push(left);
                        nodes_to_try.push(right);
                    }
                }
                BvhNode::Partition { aabb, range } => {
                    if aabb.hit(r, t_min, t_max) {
                        for d in self.descriptors[range.start..range.end].iter() {
                            if d.aabb.hit(r, t_min, closest_hit) {
                                let object = &self.objects[d.object_idx];
                                let current_hit = object.hit(r, t_min, closest_hit);
                                if let Some(ref hit) = current_hit {
                                    closest_hit = hit.t;
                                    final_hit = current_hit;
                                }
                            }
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
