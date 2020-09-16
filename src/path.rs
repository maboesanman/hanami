use std::{rc::Rc, cmp::max, fmt::Debug};
use crate::genotype::Genotype;


pub struct Path {
    pub target: Genotype,
    pub expected_time: usize,
    node: Node,
}

enum Node {
    Internal((Rc<Path>, Rc<Path>)),
    Leaf(Genotype),
}

impl Path {
    pub fn new(genotype: Genotype) -> Self {
        Self { 
            target: genotype,
            expected_time: 0,
            node: Node::Leaf(genotype),
        }
    }

    pub fn breed(self: Rc<Self>, other: Rc<Self>) -> impl Iterator<Item = Rc<Path>> {
        self.target.breed(&other.target).map(move |(numerator, genotype)| {
            Rc::new(Self {
                target: genotype,
                expected_time: 1 + 256 / numerator * max(self.expected_time, other.expected_time),
                node: Node::Internal((self.clone(), other.clone())),
            })
        })
    }
}

impl Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.node {
            Node::Internal((left, right)) => write!(f, "{:?} t={:?} ({:?})-({:?})", self.target, self.expected_time, left, right),
            Node::Leaf(_) => write!(f, "{:?} t={:?}", self.target, self.expected_time)
        }
    }
}