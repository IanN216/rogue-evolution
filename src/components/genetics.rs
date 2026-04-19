use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Allele {
    Dominant,
    Recessive,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gene {
    pub allele_a: Allele,
    pub allele_b: Allele,
    pub trait_id: String,
}

impl Gene {
    pub fn is_expressed(&self) -> bool {
        self.allele_a == Allele::Dominant || self.allele_b == Allele::Dominant
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Genetics {
    pub dna: Vec<Gene>,
    pub generation: u32,
    pub mutation_count: u32,
    pub exposure_level: f32,
}

impl Genetics {
    pub fn new() -> Self {
        Genetics {
            dna: Vec::new(),
            generation: 0,
            mutation_count: 0,
            exposure_level: 0.0,
        }
    }

    pub fn mutate(&mut self, _rate: f32) {
        // Logic for random mutation of alleles
        self.mutation_count += 1;
    }
}

pub struct PlagueMember; // Marker for the evolved archetype
