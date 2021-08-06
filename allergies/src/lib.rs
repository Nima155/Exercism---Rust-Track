pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
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

const ALLERGIES: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score >= *allergen as u32
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut score = self.score;

        ALLERGIES
            .iter()
            .rev()
            .filter(|allerg| {
                let val = **allerg as u32;
                let ans = val <= score;
                score %= val;
                ans
            })
            .copied()
            .collect()
    }
}
