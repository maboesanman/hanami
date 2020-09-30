#[macro_use]
extern crate lazy_static;

mod genotype;
mod path;
mod distribution;
mod distribution_helpers;
mod flowers;


use std::{rc::Rc, collections::HashMap};

use distribution::Distribution;
use flowers::FlowerType;
use genotype::Genotype;
use path::Path;

fn main() {
    let red_seed = Genotype::new(0b11000001);
    let yellow_seed = Genotype::new(0b00110000);
    let white_seed = Genotype::new(0b00000100);

    let red_rose_dist = Distribution::new(FlowerType::Rose, red_seed);
    let white_rose_dist = Distribution::new(FlowerType::Rose, white_seed);

    for x in red_rose_dist.breed(&white_rose_dist) {
        println!("{:?}", x.flower_color);
    }
}
