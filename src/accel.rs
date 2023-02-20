use crate::{ray::Ray, hittable::{HitRecord, Hittable}, bbox::BBox, object::{Object, self}, scene::Scene};

pub trait Accel : Hittable {
    fn build(&mut self, objects: &Vec<Object>);
}

const MAX_PRIMITIVES: usize = 4;

pub struct BVHNode {
    left: Option<usize>,
    right: Option<usize>,
    bbox: BBox,
    /// only used for leaf nodes
    primitive: Vec<usize>,
}

pub struct BVH<'scene> {
    root: usize,
    nodes: Vec<BVHNode>,
    objects: &'scene Vec<Object>,
}

impl <'scene> BVH<'scene> {
    pub fn new(objects: &Vec<Object>) -> BVH {
        let mut bvh = BVH { root: 0, nodes: Vec::new(), objects };
        bvh.build(objects);
        bvh
    }

    pub fn build(&mut self, objects: &Vec<Object>) {
        let mut indexes = Vec::new();
        for i in 0..objects.len() {
            indexes.push(i);
        }
        self.root = self.build_from(&objects, indexes);
    }

    pub fn build_from(&mut self, objects: &Vec<Object>, indexes: Vec<usize>) -> usize {
        let mut indexes = indexes;
        let mut bbox = BBox::default();
        for i in 0..indexes.len() {
            bbox = bbox.union(&objects[i].bbox());
        }
        if indexes.len() <= MAX_PRIMITIVES {
            let node = BVHNode { left: None, right: None, bbox, primitive: indexes };
            self.nodes.push(node);
            return self.nodes.len() - 1;
        }
        let axis = bbox.max_extent();
        let mid = indexes.len() / 2;
        indexes.sort_by(|a, b| {
            let a_center = objects[*a].bbox().center()[axis];
            let b_center = objects[*b].bbox().center()[axis];
            a_center.partial_cmp(&b_center).unwrap()
        });
       
        let left = self.build_from(&objects, indexes[0..mid].to_vec());
        let right = self.build_from(&objects, indexes[mid..indexes.len()].to_vec());
        let node = BVHNode { left: Some(left), right: Some(right), bbox, primitive: Vec::new() };
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn hit_node(&self, node: usize, ray: &Ray) -> Option<HitRecord> {
        if !self.nodes[node].bbox.hit(ray) {
            return None
        } 
        let mut hit = None;
        if self.nodes[node].left.is_none() && self.nodes[node].right.is_none() {
            for i in 0..self.nodes[node].primitive.len() {
                let object = &self.objects[self.nodes[node].primitive[i]];
                if let Some(record) = object.hit(ray) {
                    hit = Some(record);
                }
            }
            return hit
        }
        let left_hit = self.hit_node(self.nodes[node].left.unwrap(), ray);
        let right_hit = self.hit_node(self.nodes[node].right.unwrap(), ray);
        match (left_hit, right_hit) {
            (Some(left), Some(right)) => {
                if left.t < right.t {
                    Some(left)
                } else {
                    Some(right)
                }
            }
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (None, None) => None,
        }
    }
}



impl<'scene> Accel for BVH<'scene> {
    fn build(&mut self, objects: &Vec<Object>) {
        let mut indexes = Vec::new();
        for i in 0..objects.len() {
            indexes.push(i);
        }
        let mut bvh = BVH::new(objects);
        bvh.build_from(&objects, indexes);
    }    
}

impl<'scene> Hittable for BVH<'scene> {
    fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        self.hit_node(self.root, ray)
    }

    fn bbox(&self) -> BBox {
        self.nodes[self.root].bbox
    }
}
    