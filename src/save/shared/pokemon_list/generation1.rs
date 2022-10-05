use crate::pokemon;

use super::{PokemonList, PokemonListBase, Generation1PokemonList};

impl Generation1PokemonList {
    pub fn new(data: Option<Vec<u8>>,
        capacity: u8,
        is_japanese: bool,
        max_party_size: usize,) -> Self {
        let mut result = Self { base: Default::default() };

        result.base = result.create(data, capacity, is_japanese, max_party_size);

        result
    }
}

impl PokemonList for Generation1PokemonList {
    type Pokemon = pokemon::Generation1;

    fn get_base(&self) -> &PokemonListBase<pokemon::Generation1> {
        &self.base
    }

    fn get_entry(&self,
        raw_pokemon_data: &[u8],
        raw_ot_name: &[u8],
        raw_nickname: &[u8],
        _is_egg: bool, // There is no eggs in Generation 1
    ) -> pokemon::Generation1 {
        let mut result = pokemon::Generation1::new(raw_pokemon_data, self.base.is_japanese);

        result.base.ot_bytes = raw_ot_name.to_vec();
        result.base.nickname_bytes = raw_nickname.to_vec();

        result
    }
}
