use std::{cmp::Ordering, fmt::Debug, sync::Arc};
use serde::ser::{Serialize, SerializeStruct};

use crate::distribution::Distribution;


pub struct Path {
    pub target: Distribution,
    pub expected_time: f32,
    source: PathSource,
}

enum PathSource {
    Label(String),
    Breed((Arc<Path>, Arc<Path>))
}

impl Path {
    pub fn new(label: String, distribution: Distribution) -> Self {
        Self { 
            target: distribution,
            expected_time: 0f32,
            source: PathSource::Label(label),
        }
    }

    pub fn breed(self: Arc<Self>, other: Arc<Self>) -> impl Iterator<Item = Arc<Path>> {
        let mut parent_cost = self.expected_time;
        if parent_cost.partial_cmp(&other.expected_time).unwrap() == Ordering::Less {
            parent_cost = other.expected_time;
        }
        let mut new_dists = self.target.breed(&other.target);
        new_dists.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap().reverse());
        new_dists.into_iter().map(move |(probability, distribution)| Arc::new(Self {
            target: distribution,
            expected_time: parent_cost + 1f32 / probability,
            source: PathSource::Breed((self.clone(), other.clone())),
        }))
    }

    fn parent_cost(&self) -> f32{
        match &self.source {
            PathSource::Label(_) => 0f32,
            PathSource::Breed((left, right)) => {
                let mut parent_cost = left.expected_time;
                if parent_cost.partial_cmp(&right.expected_time).unwrap() == Ordering::Less {
                    parent_cost = right.expected_time;
                }
                parent_cost
            }
        }
    }
}

impl Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.source {
            PathSource::Breed((left, right)) => write!(f, "{:?} t={:?} ({:?})-({:?})", self.target.flower_color, self.expected_time, left, right),
            PathSource::Label(label) => write!(f, "{:?} t={:?}", label, self.expected_time)
        }
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.expected_time.partial_cmp(&other.expected_time) {
            Some(Ordering::Equal) => self.parent_cost().partial_cmp(&other.parent_cost()).unwrap().reverse(),
            Some(ord) => ord,
            None => panic!()
        }
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Path {}
impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        
        match &self.source {
            PathSource::Label(label) => serializer.serialize_str(&label),
            PathSource::Breed((left, right)) => {
                let mut path = serializer.serialize_struct("Path", 4)?;
                path.serialize_field("color", &self.target.flower_color)?;
                path.serialize_field("expectedTime", &self.expected_time)?;
                path.serialize_field("genotypes", &self.target)?;
                path.serialize_field("parents", &(left.as_ref(), right.as_ref()))?;
                path.end()
            }
        }
    }
}