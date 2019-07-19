#[derive(Clone)]
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

impl Allergies {
    pub fn new(score: u32) -> Self {
        let all_allergens = vec![Allergen::Cats, Allergen::Chocolate, 
                                 Allergen::Eggs, Allergen::Peanuts, 
                                 Allergen::Pollen, Allergen::Shellfish, 
                                 Allergen::Strawberries, Allergen::Tomatoes];

        Self { allergens: all_allergens.into_iter()
            .filter(|allergen| score & *allergen as u32 != 0)
            .collect() }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(&allergen)
    }

    pub fn allergies(self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}
