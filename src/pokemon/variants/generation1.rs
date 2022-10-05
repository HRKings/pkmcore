use crate::pokemon::{base::{PokemonTrait, PokemonBase}, Generation1};

impl Generation1 {
  pub fn new(data: &[u8], is_japanese: bool) -> Self {
      let mut result = Self {
        base: Default::default(),
        is_japanese,
      };

      result.base.data = data.to_vec();

      result
  }
}

impl PokemonTrait for Generation1 {
    fn get_base(&self) -> &PokemonBase {
        &self.base
    }

    fn decrypt(&self) -> Vec<u8> {
        todo!()
    }

    fn encrypt(&self) -> Vec<u8> {
        todo!()
    }

    fn write(&self) -> Vec<u8> {
        todo!()
    }

    fn read(data: &[u8]) -> Self {
        todo!()
    }

    fn get_species(&self) -> u16 {
        todo!()
    }

    fn is_nicknamed(&self) -> bool {
        todo!()
    }

    fn is_egg(&self) -> bool {
        todo!()
    }

    fn regenerate_checksum(&self) {
        todo!()
    }

    fn validate_checksum(&self) {
        todo!()
    }
}
