use crate::Allergen::*;

#[derive(Debug)]
pub struct Allergies {
    allergens: Vec<Allergen>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

const ALLERGENS: [Allergen; 8] = [Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats];

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut score = score % 256;
        Self {
            allergens: ALLERGENS
                .iter()
                .rev()
                .filter(|&&v| {
                    if score >= v as u32 {
                        score = score - v as u32;
                        true
                    } else {
                        false
                    }
                })
                .copied()
                .collect(),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
