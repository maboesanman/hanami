use std::{rc::Rc, fmt};

const U1_LOOKUP: [u8; 2] = [
    0b0,
    0b1,
];
const U2_LOOKUP: [u8; 4] = [
    0b00,
    0b01, 0b10,
    0b11,
];
const U3_LOOKUP: [u8; 8] = [
    0b000,
    0b001, 0b010, 0b100,
    0b011, 0b101, 0b110,
    0b111,
];
const U4_LOOKUP: [u8; 16] = [
    0b0000,
    0b0001, 0b0010, 0b0100, 0b1000,
    0b0011, 0b0101, 0b0110, 0b1001, 0b1010, 0b1100,
    0b0111, 0b1011, 0b1101, 0b1110,
    0b1111,
];


#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Genotype(pub u8);

impl fmt::Debug for Genotype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dominant = "RRYYwwSS";
        let recessive = "rryyWWss";
        let iter = recessive.chars().zip(dominant.chars()).enumerate();
        let string = iter.map(|(i, (a, b))| if self.0 & (1 << (7 - i)) == 0 { a } else { b }).collect::<String>();
        write!(f, "{}", string)
    }
}

impl fmt::Binary for Genotype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        fmt::Binary::fmt(&val, f)
    }
}

impl Genotype {
    pub fn new(mut inner: u8) -> Self {
        for i in vec![0, 2, 4, 6] {
            if inner & (0b11 << i) == (0b10 << i) {
                inner ^= 0b11 << i;
            }
        }
        Genotype(inner)
    }

    pub fn breed(&self, other: &Self) -> impl Iterator<Item = (usize, Self)> {
        BreedResultIter::new(*self, *other)
    }

    fn get_genes(&self) -> [u8; 4] {
        let allele_1 = (self.0 & 0b11_00_00_00) >> 6;
        let allele_2 = (self.0 & 0b00_11_00_00) >> 4;
        let allele_3 = (self.0 & 0b00_00_11_00) >> 2;
        let allele_4 = self.0 & 0b00_00_00_11;
        [allele_1, allele_2, allele_3, allele_4]
    }
}

pub struct BreedResultIter {
    allele_121_indices: Vec<u8>,
    current_lookup_index: usize,

    allele_1_dist: [u8; 3],
    allele_2_dist: [u8; 3],
    allele_3_dist: [u8; 3],
    allele_4_dist: [u8; 3],

    allele_1_pos: usize,
    allele_2_pos: usize,
    allele_3_pos: usize,
    allele_4_pos: usize,

    finished: bool,
}

impl BreedResultIter {
    pub fn new(parent_1: Genotype, parent_2: Genotype) -> Self {
        let parent_1_alleles = parent_1.get_genes();
        let parent_2_alleles = parent_2.get_genes();

        let mut allele_1_dist = Self::get_allele_dist(parent_1_alleles[0], parent_2_alleles[0]);
        let mut allele_2_dist = Self::get_allele_dist(parent_1_alleles[1], parent_2_alleles[1]);
        let mut allele_3_dist = Self::get_allele_dist(parent_1_alleles[2], parent_2_alleles[2]);
        let mut allele_4_dist = Self::get_allele_dist(parent_1_alleles[3], parent_2_alleles[3]);

        let mut allele_121_indices = Vec::new();
        if allele_1_dist == [1, 2, 1] {
            allele_121_indices.push(1);
            allele_1_dist = [0, 2, 0];
        }
        if allele_2_dist == [1, 2, 1] {
            allele_121_indices.push(2);
            allele_2_dist = [0, 2, 0];
        }
        if allele_3_dist == [1, 2, 1] {
            allele_121_indices.push(3);
            allele_3_dist = [0, 2, 0];
        }
        if allele_4_dist == [1, 2, 1] {
            allele_121_indices.push(4);
            allele_4_dist = [0, 2, 0];
        }

        Self {
            allele_121_indices,
            current_lookup_index: 0,

            allele_1_dist,
            allele_2_dist,
            allele_3_dist,
            allele_4_dist,

            allele_1_pos: Self::get_start_pos(allele_1_dist),
            allele_2_pos: Self::get_start_pos(allele_2_dist),
            allele_3_pos: Self::get_start_pos(allele_3_dist),
            allele_4_pos: Self::get_start_pos(allele_4_dist),

            finished: false,
        }
    }

    fn get_start_pos(allele_dist: [u8; 3]) -> usize {
        
        match allele_dist {
            [0, 0, _] => 2,
            [0, _, _] => 1,
            [_, _, _] => 0,
        }
    }

    fn get_allele_dist(allele_1: u8, allele_2: u8) -> [u8; 3] {
        match (allele_1, allele_2) {
            (0b00, 0b00) => [4, 0, 0],
            (0b00, 0b01) => [2, 2, 0],
            (0b00, 0b11) => [0, 4, 0],
            (0b01, 0b00) => [2, 2, 0],
            (0b01, 0b01) => [1, 2, 1],
            (0b01, 0b11) => [0, 2, 2],
            (0b11, 0b00) => [0, 4, 0],
            (0b11, 0b01) => [0, 2, 2],
            (0b11, 0b11) => [0, 0, 4],
            _ => unreachable!(),
        }
    }

    fn increment_single(allele_dist: &[u8], allele_pos: &mut usize) -> bool {
        let mut carry = false;
        loop {
            *allele_pos += 1;
            if *allele_pos == 3 {
                *allele_pos = 0;
                carry = true;
            }
            if allele_dist[*allele_pos] == 0 {
                continue;
            }
            break;
        }

        carry
    }

    fn increment_121(&mut self) -> bool {
        self.current_lookup_index += 1;
        let next = match self.allele_121_indices.len() {
            0 => return true,
            1 => {
                if self.current_lookup_index == 2 { return true; }
                U1_LOOKUP[self.current_lookup_index]
            }
            2 => {
                if self.current_lookup_index == 4 { return true; }
                U2_LOOKUP[self.current_lookup_index]
            }
            3 => {
                if self.current_lookup_index == 8 { return true; }
                U3_LOOKUP[self.current_lookup_index]
            }
            4 => {
                if self.current_lookup_index == 16 { return true; }
                U4_LOOKUP[self.current_lookup_index]
            }
            _ => unreachable!()
        };

        for (b, i) in self.allele_121_indices.iter().rev().enumerate() {
            let (new_dist, new_pos) = if (next >> b) & 1 == 1 {
                ([1, 0, 1], 0)
            } else {
                ([0, 2, 0], 1)
            };
            match i {
                1 => {
                    self.allele_1_dist = new_dist;
                    self.allele_1_pos = new_pos;
                },
                2 => {
                    self.allele_2_dist = new_dist;
                    self.allele_2_pos = new_pos;
                },
                3 => {
                    self.allele_3_dist = new_dist;
                    self.allele_3_pos = new_pos;
                },
                4 => {
                    self.allele_4_dist = new_dist;
                    self.allele_4_pos = new_pos;
                },
                _ => unreachable!()
            }
        }

        false
    }

    fn increment(&mut self) {
        if !Self::increment_single(&self.allele_4_dist, &mut self.allele_4_pos) {
            return;
        }
        if !Self::increment_single(&self.allele_3_dist, &mut self.allele_3_pos) {
            return;
        }
        if !Self::increment_single(&self.allele_2_dist, &mut self.allele_2_pos) {
            return;
        }
        if !Self::increment_single(&self.allele_1_dist, &mut self.allele_1_pos) {
            return;
        }
        self.finished = self.increment_121();
    }

    fn get_current_numerator(&self) -> usize {
        let mut numerator = 1;
        numerator *= self.allele_1_dist[self.allele_1_pos] as usize;
        numerator *= self.allele_2_dist[self.allele_2_pos] as usize;
        numerator *= self.allele_3_dist[self.allele_3_pos] as usize;
        numerator *= self.allele_4_dist[self.allele_4_pos] as usize;
        numerator
    }

    fn get_current_genotype(&self) -> Genotype {
        let mut combined = 0u8;
        combined += Self::allele_from_pos(self.allele_1_pos) << 6;
        combined += Self::allele_from_pos(self.allele_2_pos) << 4;
        combined += Self::allele_from_pos(self.allele_3_pos) << 2;
        combined += Self::allele_from_pos(self.allele_4_pos);
        Genotype::new(combined)
    }

    fn allele_from_pos(pos: usize) -> u8 {
        match pos {
            0 => 0,
            1 => 1,
            2 => 3,
            _ => unreachable!()
        }
    }
}

impl Iterator for BreedResultIter {
    // the usize is the numerator of a probability with denominator 256.
    type Item = (usize, Genotype);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None
        }

        let numerator = self.get_current_numerator();
        let genotype = self.get_current_genotype();
        self.increment();
        Some((numerator, genotype))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (1, Some(81))
    }
}
