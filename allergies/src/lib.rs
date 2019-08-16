use std::ops::BitAnd;

#[derive(Clone, Copy)]
pub struct Allergies(pub u32);

#[derive(Clone, Copy, Debug, PartialEq)]
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

static ALLERGENS: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

impl BitAnd<&Allergen> for Allergies {
    type Output = bool;

    fn bitand(self, a: &Allergen) -> Self::Output {
        self.0 & (*a as u32) == (*a as u32)
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        *self & allergen
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS
            .iter()
            .filter_map(|a| {
                if self.is_allergic_to(a) {
                    Some(*a)
                } else {
                    None
                }
            })
            .collect()
    }
}
