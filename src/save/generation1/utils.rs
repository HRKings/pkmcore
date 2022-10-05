use crate::{
    game::enums::game_version::GameVersion,
    utils::constants::poke_crypto::{SIZE_GENERATION_1_PARTY, SIZE_GENERATION_1_STORED},
};

// region: Sizes
const SAVE_RAW_SIZE: usize = 0x8000;
const SAVE_BAT_SIZE: usize = 0x802C;

const STRING_LENGTH_JAPANESE: usize = 6;
const STRING_LENGTH_INTERNATIONAL: usize = 11;
// endregion

// region: Offsets
const INTERNATIONAL_POKEMON_LIST_1_OFFSET: usize = 0x2F2C;
const INTERNATIONAL_POKEMON_LIST_2_OFFSET: usize = 0x30C0;
const JAPANESE_POKEMON_LIST_1_OFFSET: usize = 0x2ED5;
const JAPANESE_POKEMON_LIST_2_OFFSET: usize = 0x302D;
// endregion

// region: Lists
// Acts as a magic capacity value; there aren't any other List sizes used by the games.
pub const FULL_INTERNATIONAL_POKEMON_BOX_COUNT: usize = 20;
pub const FULL_JAPANESE_POKEMON_BOX_COUNT: usize = 30;
// endregion

/// Determines if a Gen1/2 Pokémon List is valid
/// # Arguments
/// * `data` - Save data
/// * `offset` - Offset where the list starts
/// * `max_pokemon` - How many pokemon should be in the list
fn is_generation_1or2_pokemon_list_valid(data: &[u8], offset: usize, max_pokemon: usize) -> bool {
    let pokemon_count = data[offset] as usize;
    pokemon_count <= max_pokemon && data[offset + 1 + pokemon_count] == 0xFF
}

/// Checks to see if the data belongs to an International (American/U) Generation 1 save
fn is_generation_1_international(data: &[u8]) -> bool {
    is_generation_1or2_pokemon_list_valid(
        data,
        INTERNATIONAL_POKEMON_LIST_1_OFFSET,
        FULL_INTERNATIONAL_POKEMON_BOX_COUNT,
    ) && is_generation_1or2_pokemon_list_valid(
        data,
        INTERNATIONAL_POKEMON_LIST_2_OFFSET,
        FULL_INTERNATIONAL_POKEMON_BOX_COUNT,
    )
}

/// Checks to see if the data belongs to an Japanese (JAP) Generation 1 save
pub fn is_generation_1_japanese(data: &[u8]) -> bool {
    is_generation_1or2_pokemon_list_valid(
        data,
        JAPANESE_POKEMON_LIST_1_OFFSET,
        FULL_JAPANESE_POKEMON_BOX_COUNT,
    ) && is_generation_1or2_pokemon_list_valid(
        data,
        JAPANESE_POKEMON_LIST_2_OFFSET,
        FULL_JAPANESE_POKEMON_BOX_COUNT,
    )
}

/// Checks to see if the data belongs to a Generation 1 save
pub fn is_generation_1(data: &[u8]) -> GameVersion {
    if data.len() != SAVE_RAW_SIZE || data.len() != SAVE_BAT_SIZE {
        return GameVersion::Invalid;
    }

    // Check if it's not an american save or a japanese save
    if !(is_generation_1_international(data) || is_generation_1_japanese(data)) {
        return GameVersion::Invalid;
    }
    // I can't actually detect which game version, because it's not stored anywhere.
    // If you can think of anything to do here, please implement :)
    GameVersion::GroupRedBlueYellow
}

pub fn get_string_length(is_japanese: bool) -> usize {
    if is_japanese {
        STRING_LENGTH_JAPANESE
    } else {
        STRING_LENGTH_INTERNATIONAL
    }
}

fn is_capacity_from_party(pokemon_list_capacity: usize, max_party_size: usize) -> bool {
    pokemon_list_capacity == max_party_size || pokemon_list_capacity == 1
}

fn get_entry_size(is_party: bool) -> usize {
    if is_party {
        SIZE_GENERATION_1_PARTY
    } else {
        SIZE_GENERATION_1_STORED
    }
}

pub fn get_party_entry_size(pokemon_list_capacity: usize, max_party_size: usize) -> usize {
    get_entry_size(is_capacity_from_party(pokemon_list_capacity, max_party_size))
}


fn get_data_size(pokemon_list_capacity: usize, is_japanese: bool, entry_size: usize) -> usize {
    let entry_length = 1 + entry_size + (2 * get_string_length(is_japanese));
    2 + (pokemon_list_capacity * entry_length)
}

pub fn get_data_length(
    pokemon_list_capacity: usize,
    is_japanese: bool,
    max_party_size: usize,
) -> usize {
    get_data_size(
        pokemon_list_capacity,
        is_japanese,
        get_party_entry_size(pokemon_list_capacity, max_party_size),
    )
}
