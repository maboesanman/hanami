use std::hash::Hash;

use crate::{distribution_helpers::breed, flowers::FlowerColor, flowers::FlowerType, flowers::get_color, genotype::Genotype};

use sprs::CsVec;
use serde::ser::{Serialize, SerializeSeq};

#[derive(Clone)]
pub struct Distribution {
    pub flower_type:  FlowerType,
    pub flower_color: FlowerColor,
    pub inner:        CsVec<f32>,
}

impl Hash for Distribution {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.flower_type.hash(state);
        self.inner.indices().hash(state);
        // for d in self.inner.data().iter() {
        //     ((1f32 / d).round() as u8).hash(state);
        // }
    }
}

impl Serialize for Distribution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut list = serializer.serialize_seq(Some(self.inner.indices().len()))?;
        for (g, _) in self.inner.iter() {
            let genotype = Genotype::from_base_3(g as u8);
            let genotype = format!("{:?}", &genotype);
            list.serialize_element(&genotype)?;
        }
        list.end()
    }
}

impl Eq for Distribution { }
impl PartialEq for Distribution {
    fn eq(&self, other: &Self) -> bool {
        if self.flower_type != other.flower_type {
            return false;
        }
        if self.flower_color != other.flower_color {
            return false;
        }
        if self.inner.indices() != other.inner.indices() {
            return false;
        }
        return true;
    }
}

impl Distribution {
    pub fn new(flower_type: FlowerType, genotype: Genotype) -> Self {
        let index = dist_index_from_genotype(genotype);
        let inner = CsVec::new(81, vec![index], vec![1.0]);
        let flower_color = get_color(&flower_type, genotype);

        Self {
            flower_type,
            flower_color,
            inner,
        }
    }

    pub fn breed(&self, other: &Self) -> Vec<(f32, Self)> {
        breed(self, other)
    }
}

pub fn dist_index_from_genotype(genotype: Genotype) -> usize {
    genotype.get_base_3() as usize
}

pub fn genotype_from_dist_index(index: usize) -> Genotype {
    Genotype::from_base_3(index as u8)
}

// impl Eq for Distribution {}

// impl PartialEq for Distribution {
//     fn eq(&self, other: &Self) -> bool {
//         todo!()
//     }
// }