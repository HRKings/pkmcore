pub mod generation1;

pub struct SpeciesTableBase {
  pub max_species_id: u16,
}

pub enum SpeciesTable {
  Latest(SpeciesTableBase),
  RedBlue(SpeciesTableBase),
  Yellow(SpeciesTableBase),
  RubySapphire(SpeciesTableBase),
  Emerald(SpeciesTableBase),
  FireRed(SpeciesTableBase),
  LeafGreen(SpeciesTableBase),
}

impl Default for SpeciesTable {
    fn default() -> Self {
        todo!()
    }
}
