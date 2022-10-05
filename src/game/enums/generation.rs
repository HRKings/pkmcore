#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Generation {
    None = 0,

    G1 = 1,
    G2 = 2,
    G3 = 3,
    G4 = 4,
    G5 = 5,
    G6 = 6,
    G7 = 7,
    G8 = 8,

    // region: Invalid for save
    INVALID_FOR_SAVE,

    G7a,
    G8a,
    G8b,

    MAX_INVALID_FOR_SAVE,
    // endregion
}

impl Generation {
    pub fn get_generation_number(&self) -> usize {
        match self {
            Generation::None => 0,
            Generation::G1 => 1,
            Generation::G2 => 2,
            Generation::G3 => 3,
            Generation::G4 => 4,
            Generation::G5 => 5,
            Generation::G6 => 6,
            Generation::G7 => 7,
            Generation::G8 => 8,
            Generation::INVALID_FOR_SAVE => 0,
            Generation::G7a => 7,
            Generation::G8a => 8,
            Generation::G8b => 8,
            Generation::MAX_INVALID_FOR_SAVE => 0,
        }
    }
}
