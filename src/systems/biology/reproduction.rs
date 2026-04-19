use crate::components::genetics::{Genetics, Gene, Allele};
use bracket_lib::prelude::*;

pub fn reproduce(parent_a: &Genetics, parent_b: &Genetics, mutation_rate: f32) -> Genetics {
    let mut rng = RandomNumberGenerator::new();
    let mut child_dna = Vec::new();

    // Mendelian Inheritance: One allele from each parent for each trait
    for (gene_a, gene_b) in parent_a.dna.iter().zip(parent_b.dna.iter()) {
        if gene_a.trait_id == gene_b.trait_id {
            let allele_from_a = if rng.range(0, 2) == 0 { &gene_a.allele_a } else { &gene_a.allele_b };
            let allele_from_b = if rng.range(0, 2) == 0 { &gene_b.allele_a } else { &gene_b.allele_b };

            let mut new_gene = Gene {
                allele_a: allele_from_a.clone(),
                allele_b: allele_from_b.clone(),
                trait_id: gene_a.trait_id.clone(),
            };

            // Mutation logic
            if rng.roll_dice(1, 1000) <= (mutation_rate * 1000.0) as i32 {
                if rng.range(0, 2) == 0 {
                    new_gene.allele_a = if rng.range(0, 2) == 0 { Allele::Dominant } else { Allele::Recessive };
                } else {
                    new_gene.allele_b = if rng.range(0, 2) == 0 { Allele::Dominant } else { Allele::Recessive };
                }
            }

            child_dna.push(new_gene);
        }
    }

    Genetics {
        dna: child_dna,
        generation: parent_a.generation.max(parent_b.generation) + 1,
        mutation_count: 0, 
        exposure_level: 0.0,
    }
}
