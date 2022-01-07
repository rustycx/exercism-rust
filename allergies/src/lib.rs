pub struct Allergies {
    score: u32,
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

use Allergen::*;
const ALLERGENS: [Allergen; 8] = [
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let &Allergies { score } = self;
        let value = *allergen as u32;
        value & score != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let allergens: Vec<Allergen> = ALLERGENS
            .iter()
            .filter(|a| self.is_allergic_to(a))
            .copied()
            .collect();
        allergens
    }
}
