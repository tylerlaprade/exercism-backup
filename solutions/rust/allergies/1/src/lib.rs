pub struct Allergies {
    score: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Allergies {
    #[must_use]
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    #[must_use]
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & *allergen as u32 != 0
    }

    #[must_use]
    pub fn allergies(&self) -> Vec<Allergen> {
        // I tried to be clever and generate this from (0..8), but I couldn't figure out how to cast numbers to enum values
        [
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ]
        .into_iter()
        .filter(|allergen| self.is_allergic_to(allergen))
        .collect()
    }
}
