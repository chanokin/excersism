pub struct Allergies {
    bin_allergens: u32, 
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            bin_allergens: score,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen = *allergen as  u32;
        let mask: u32 = 1 << allergen;
        self.bin_allergens & mask > 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergic_to = Vec::<Allergen>::new();
        for bit in 0..8 {
            let mask = 1 << bit as u32;
            let val = match bit as u8 {
                0 => Allergen::Eggs,
                1 => Allergen::Peanuts,
                2 => Allergen::Shellfish,
                3 => Allergen::Strawberries,
                4 => Allergen::Tomatoes,
                5 => Allergen::Chocolate,
                6 => Allergen::Pollen,
                _ => Allergen::Cats,
            };
            if self.bin_allergens & mask > 0 {
                allergic_to.push(val);
            }
        }

        allergic_to
    }
}
