mod genotype;
mod path;

use std::{rc::Rc, collections::HashMap};

use genotype::Genotype;
use path::Path;

fn main() {
    let mut hashmap = HashMap::new();
    let red_seed = Genotype::new(0b11000001);
    let yellow_seed = Genotype::new(0b00110000);
    let white_seed = Genotype::new(0b00000100);

    let red_path = Rc::new(Path::new(red_seed));
    let yellow_path = Rc::new(Path::new(yellow_seed));
    let white_path = Rc::new(Path::new(white_seed));

    hashmap.insert(red_seed, red_path.clone());
    hashmap.insert(yellow_seed, yellow_path.clone());
    hashmap.insert(white_seed, white_path.clone());

    let iter = red_path.breed(yellow_path);

    for path in iter {
        let current_path = hashmap.get_mut(&path.target);
        match current_path {
            Some(current_path) => {
                if current_path.expected_time > path.expected_time {
                    *current_path = path;
                }
            }
            None => {
                hashmap.insert(path.target, path);
            }
        }
    }

    for (_, path) in hashmap.clone().iter() {
        for path in white_path.clone().breed(path.clone()) {
            let current_path = hashmap.get_mut(&path.target);
            match current_path {
                Some(current_path) => {
                    if current_path.expected_time > path.expected_time {
                        *current_path = path;
                    }
                }
                None => {
                    hashmap.insert(path.target, path);
                }
            }
        }
    }

    for (_, path) in hashmap.iter() {
        println!("{:?}", path);
    }
}