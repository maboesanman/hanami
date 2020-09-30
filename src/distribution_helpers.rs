use std::{collections::HashMap, sync::RwLock};

use sprs::{CsMat, CsVec};

use crate::{distribution::Distribution, distribution::dist_index_from_genotype, distribution::genotype_from_dist_index, flowers::{COSMOS_LIST, FlowerColor, FlowerType, HYACINTHS_LIST, LILIES_LIST, MUMS_LIST, PANSIES_LIST, ROSES_LIST, TULIPS_LIST, WINDFLOWERS_LIST}};


lazy_static! {
    static ref COLOR_FILTERS: RwLock<HashMap<(FlowerType, FlowerColor), CsMat<f32>>> = RwLock::new(HashMap::new());
    static ref GENOTYPE_MAPS: RwLock<HashMap<FlowerType, (Vec<FlowerColor>, CsMat<f32>)>> = RwLock::new(HashMap::new());

    static ref BREED_MATRIX: CsMat<f32> = {
        let mut breed_mat = CsMat::<f32>::zero((81, 81 * 81)).transpose_into();

        for parent1 in 0..81 {
            for parent2 in 0..81 {
                let parent1_genotype = genotype_from_dist_index(parent1);
                let parent2_genotype = genotype_from_dist_index(parent2);
                let parent_index = parent1 * 81 + parent2;
                for (probability_numerator, child_genotype) in parent1_genotype.breed(&parent2_genotype) {
                    let probability = probability_numerator as f32 / 256f32;
                    let child_index = dist_index_from_genotype(child_genotype);

                    breed_mat.insert(parent_index, child_index, probability);
                }
            }
        }

        breed_mat.to_csr()
    };
}

fn process_flower(flower_type: &FlowerType) {
    let genotype_maps = GENOTYPE_MAPS.read().unwrap();
    if genotype_maps.contains_key(flower_type) {
        return;
    }
    std::mem::drop(genotype_maps);

    let map_slice: &[FlowerColor] = match flower_type {
        FlowerType::Rose => &ROSES_LIST,
        FlowerType::Cosmo => &COSMOS_LIST,
        FlowerType::Lily => &LILIES_LIST,
        FlowerType::Pansy => &PANSIES_LIST,
        FlowerType::Tulip => &HYACINTHS_LIST,
        FlowerType::Hyacinth => &TULIPS_LIST,
        FlowerType::Mum => &MUMS_LIST,
        FlowerType::Windflower => &WINDFLOWERS_LIST,
    };
    let mut new_genotype_map_mat = CsMat::zero((0, map_slice.len())).to_csc();
    let mut new_genotype_map_vec = Vec::new();
    let mut new_color_filters = HashMap::new();
    for (row, color) in map_slice.iter().enumerate() {
        let col = new_genotype_map_vec.iter().rposition(|x| x == color);
        let col = match col {
            Some(col) => col,
            None => {
                new_genotype_map_vec.push(*color);
                new_color_filters.insert((*flower_type, *color), CsMat::zero((map_slice.len(), map_slice.len())));
                new_genotype_map_vec.len() - 1
            }
        };
        new_genotype_map_mat.insert(row, col, 1f32);
        new_color_filters.get_mut(&(*flower_type, *color)).unwrap().insert(row, row, 1f32);
    }
    let mut genotype_maps = GENOTYPE_MAPS.write().unwrap();
    genotype_maps.insert(*flower_type, (new_genotype_map_vec, new_genotype_map_mat));
    let mut color_filters = COLOR_FILTERS.write().unwrap();
    color_filters.extend(new_color_filters.into_iter());
}

pub fn breed(a: &Distribution, b: &Distribution) -> Vec<Distribution> {
    if a.flower_type != b.flower_type {
        panic!()
    }
    println!("{:?}", a.inner);
    println!("{:?}", b.inner);
    let spread_mat = &(a.inner.col_view::<usize>()) * &b.inner.row_view();
    let spread_vec = as_one_row(spread_mat);
    let breed_dist = &spread_vec * &BREED_MATRIX;

    let flower_type = a.flower_type;
    process_flower(&flower_type);
    let genotype_maps = GENOTYPE_MAPS.read().unwrap();
    let color_filters = COLOR_FILTERS.read().unwrap();
    
    let (color_vec, mat) = genotype_maps.get(&flower_type).unwrap();
    let mut color_dist = vec![0f32; mat.cols()];
    (&breed_dist * &mat).scatter(&mut color_dist);

    
    let mut outputs = Vec::new();
    for (color, probability) in color_vec.iter().zip(color_dist.iter()) {
        let mut filtered = &breed_dist * color_filters.get(&(flower_type, *color)).unwrap();
        filtered.map_inplace(|x| x / probability);
        outputs.push(Distribution {
            flower_type,
            flower_color: *color,
            inner: filtered,
        })
    }

    outputs
}

pub fn as_one_row<T: Copy + Clone + Default>(mat: CsMat<T>) -> CsVec<T> {
    let mat = mat.to_csr();
    let (row, col) = mat.shape();
    let (indptr, mut indices, data) = mat.into_raw_storage();
    let mut prev_ind = 0;
    for (row, ind) in indptr.iter().enumerate() {
        while prev_ind < *ind {
            indices[prev_ind] += (row - 1) * col;
            prev_ind += 1;
        }
        prev_ind = *ind;
    }

    CsVec::new(row * col, indices, data)
}