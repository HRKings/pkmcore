pub mod generation1;

use crate::{save::generation1::utils::get_party_entry_size, strings, utils::data_manipulation::SliceUtils, pokemon::{base::PokemonTrait, self}};

const EMPTY_SLOT: u8 = u8::MAX;

// Structure:
// u8               Count of slots filled
// Option<u8>[capacity+1]   GB Species ID in Slot (-1 = no species)
// pokemon[capacity]    GB PKM data (no strings)
// str[capacity]    Trainer Name
// str[capacity]    Nickname
//
// where,
// - str has variable size (jp/int)
// - pkx is different size for gen1/gen2

#[derive(Debug, Default)]
pub struct PokemonListBase<T> where T: PokemonTrait {
    pub data: Vec<u8>,

    pub string_length: usize,
    pub capacity: u8,
    pub entry_size: usize,
    pub is_japanese: bool,

    pub pokemon: Vec<T>,
}

impl<T> PokemonListBase<T> where T: PokemonTrait {
    pub fn count(&self) -> u8 {
        self.data[0]
    }
}

pub trait PokemonList where Self::Pokemon: PokemonTrait {
    type Pokemon;

    fn get_base(&self) -> &PokemonListBase<Self::Pokemon>;

    fn get_pokemon_data_offset(&self, base_offset: usize, index: usize) -> usize {
        base_offset + (self.get_base().entry_size * index)
    }

    fn get_pokemon_ot_offset(&self, base_offset: usize, index: usize) -> usize {
        let base = self.get_base();
        self.get_pokemon_data_offset(base_offset, base.capacity as usize) + (base.string_length * index)
    }

    fn get_pokemon_nickname_offset(&self, base_offset: usize, index: usize) -> usize {
        let base = self.get_base();
        self.get_pokemon_ot_offset(base_offset, base.capacity as usize) + (base.string_length * index)
    }

    fn get_entry(&self, raw_pokemon_data: &[u8], raw_ot_name: &[u8], raw_nickname: &[u8], is_egg: bool) -> Self::Pokemon;

    fn get_pokemon_data(&self, base_offset: u8, index: usize) -> Self::Pokemon {
        let base = self.get_base();

        let pokemon_offset = self.get_pokemon_data_offset(base_offset.into(), index);
        let ot_offset = self.get_pokemon_ot_offset(base_offset.into(), index);
        let nickname_offset = self.get_pokemon_nickname_offset(base_offset.into(), index);

        let pokemon_data = base.data.get_offset(pokemon_offset, base.entry_size);
        let ot_name =  base.data.get_offset(ot_offset, base.string_length);
        let nickname =  base.data.get_offset(nickname_offset, base.string_length);

        self.get_entry(pokemon_data, ot_name, nickname,  base.data[1 + index] == 0xFD)
    }

    fn read_pokemon_list(&self) -> Vec<Self::Pokemon> {
        let base = self.get_base();

        let mut result = Vec::<Self::Pokemon>::with_capacity(base.capacity as usize);
        let base_offset = 2 + base.capacity;

        (0..(base.capacity as usize)).for_each(|i| {
            result[i] = self.get_pokemon_data(base_offset, i);
        });

        result
    }

    fn create_empty_list(capacity: u8, is_japanese: bool, max_party_size: usize) -> Vec<u8> {
        let size_intro = capacity + 1;
        let pokemon_size =
            get_party_entry_size(capacity as usize, max_party_size) * capacity as usize;
        let string_length =
            2 * crate::save::generation1::utils::get_string_length(is_japanese) * capacity as usize;

        let mut result = Vec::with_capacity(1 + size_intro as usize + pokemon_size + string_length);

        (1..=(size_intro as usize)).for_each(|i| {
            result[i] = EMPTY_SLOT;
        });

        ((1 + size_intro as usize + pokemon_size)..(result.len())).for_each(|i| {
            result[i] = strings::generation_1_or_2::TERMINATOR_CODE;
        });

        result
    }

    fn create(
        &self,
        data: Option<Vec<u8>>,
        capacity: u8,
        is_japanese: bool,
        max_party_size: usize,
    ) -> PokemonListBase<Self::Pokemon> {
        let mut result = PokemonListBase {
            data: if let Some(unwrapped_data) = data {
                unwrapped_data
            } else {
                Self::create_empty_list(capacity, is_japanese, max_party_size)
            },
            string_length: crate::save::generation1::utils::get_string_length(is_japanese),
            capacity,
            entry_size: get_party_entry_size(capacity as usize, max_party_size),
            is_japanese,
            pokemon: Vec::new(),
        };

        let data_size = 1
            + 1
            + (result.capacity as usize * (result.entry_size + 1 + (2 * result.string_length)));
        result.data.resize(data_size, u8::default());

        result.pokemon = Self::read_pokemon_list(self);

        result
    }
}


pub struct Generation1PokemonList {
    pub base: PokemonListBase<pokemon::Generation1>,
}
