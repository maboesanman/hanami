use crate::{distribution_helpers::breed, flowers::FlowerColor, flowers::FlowerType, flowers::get_color, genotype::Genotype};

use sprs::{CsMatI, CsVec, CsVecI};

lazy_static! {
}

pub struct Distribution {
    pub flower_type:  FlowerType,
    pub flower_color: FlowerColor,
    pub inner:        CsVec<f32>,
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

    pub fn breed(&self, other: &Self) -> Vec<Self> {
        breed(self, other)
    }
}

pub fn dist_index_from_genotype(genotype: Genotype) -> usize {
    genotype.get_base_3() as usize
}

pub fn genotype_from_dist_index(index: usize) -> Genotype {
    Genotype::from_base_3(index as u8)
}