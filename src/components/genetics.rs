use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Genetics {
    pub dna: [u8; 16],        // ADN compacto (genes para velocidad, tamaño, color, etc.)
    pub exposure_level: f32,  // Acumulación de anomalías
    pub generation: u32,
    pub race_id: u32,
    pub race_abilities: Vec<u8>, // IDs de habilidades compactos
}

impl Genetics {
    pub fn mutate(&mut self, rate: f32) {
        use rand::prelude::*;
        let mut rng = thread_rng();
        for gene in self.dna.iter_mut() {
            if rng.gen::<f32>() < rate {
                *gene = rng.gen();
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlagueMember; // Marker for the evolved archetype
