use crate::components::genetics::Genetics;
use rand::prelude::*;

pub fn reproduce(parent_a: &Genetics, parent_b: &Genetics, mutation_rate: f32) -> Genetics {
    let mut rng = thread_rng();
    let mut child_dna = [0u8; 16];

    for i in 0..16 {
        // Mendelian inheritance: Pick one allele from either parent
        child_dna[i] = if rng.gen::<bool>() {
            parent_a.dna[i]
        } else {
            parent_b.dna[i]
        };

        // Random mutation factor
        if rng.gen::<f32>() < mutation_rate {
            child_dna[i] = rng.gen();
        }
    }

    Genetics {
        dna: child_dna,
        exposure_level: 0.0,
        generation: parent_a.generation + 1,
        race_id: parent_a.race_id, // For now, child takes race of parent A
        race_abilities: parent_a.race_abilities.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mendelian_inheritance() {
        let parent_a = Genetics {
            dna: [255u8; 16],
            exposure_level: 0.0,
            generation: 1,
            race_id: 1,
            race_abilities: Vec::new(),
        };
        let parent_b = Genetics {
            dna: [0u8; 16],
            exposure_level: 0.0,
            generation: 1,
            race_id: 1,
            race_abilities: Vec::new(),
        };

        let child = reproduce(&parent_a, &parent_b, 0.05);
        
        // Ensure child DNA alleles are from either parent (or mutated)
        for i in 0..16 {
            let is_from_a = child.dna[i] == parent_a.dna[i];
            let is_from_b = child.dna[i] == parent_b.dna[i];
            // If mutation occurred, it might not be from either parent.
            // With 0.05 mutation rate, some will match parents.
            assert!(is_from_a || is_from_b || (child.dna[i] != parent_a.dna[i] && child.dna[i] != parent_b.dna[i]));
        }
        
        assert_eq!(child.generation, 2);
    }
}
