#[macro_use]
extern crate lazy_static;

mod genotype;
mod path;
mod distribution;
mod distribution_helpers;
mod flowers;


use std::{cmp::Reverse, collections::BinaryHeap, collections::HashMap, sync::Arc, sync::RwLock};

use distribution::Distribution;
use flowers::{FlowerColor, FlowerType};
use genotype::Genotype;
use path::Path;
use rayon::prelude::*;

fn main() {
    // sprs::smmp::set_thread_threading_strategy(sprs::smmp::ThreadingStrategy::Fixed(1));
    let red_seed = Genotype::new(0b11000001);
    let yellow_seed = Genotype::new(0b00110000);
    let white_seed = Genotype::new(0b00000100);

    let red_rose_dist = Distribution::new(FlowerType::Rose, red_seed);
    let yellow_rose_dist = Distribution::new(FlowerType::Rose, yellow_seed);
    let white_rose_dist = Distribution::new(FlowerType::Rose, white_seed);

    let red_rose_path = Arc::new(Path::new("red seed".to_string(), red_rose_dist.clone()));
    let yellow_rose_path = Arc::new(Path::new("yellow seed".to_string(), yellow_rose_dist.clone()));
    let white_rose_path = Arc::new(Path::new("white seed".to_string(), white_rose_dist.clone()));

    let mut processed: Vec<Arc<Path>> = Vec::new();
    let mut upcoming: BinaryHeap<Reverse<Arc<Path>>> = BinaryHeap::new();
    let mut visited: HashMap<Distribution, Arc<Path>> = HashMap::new();

    upcoming.push(Reverse(red_rose_path.clone()));
    upcoming.push(Reverse(yellow_rose_path.clone()));
    upcoming.push(Reverse(white_rose_path.clone()));

    loop {
        let new_path: Arc<Path>;
        if let Some(Reverse(path)) = upcoming.pop() {
            new_path = path;
        } else {
            break;
        }

        if visited.contains_key(&new_path.target) {
            continue;
        } else {
            visited.insert(new_path.target.clone(), new_path.clone());
        }

        println!("{:?}", new_path.target.flower_color);
        if new_path.target.flower_color == FlowerColor::Blue {
            let j = serde_json::to_string(new_path.as_ref()).unwrap();
            println!("{}", j);
        }

        processed.push(new_path.clone());
        let new_paths = processed.par_iter().flat_map_iter(|p| {
            new_path.clone().breed(p.clone())
        });
        // let new_paths = new_paths.filter(|p| {
        //     p.target.inner.indices().len() < 3
        // });
        let new_paths = new_paths.map(|p| Reverse(p));

        upcoming.par_extend(new_paths);
    }
}
