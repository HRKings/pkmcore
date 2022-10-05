use crate::{
    game::enums::{game_version::GameVersion, generation::Generation, language_id::LanguageID},
    legality::species_table::SpeciesTable,
    mystery_gifts::MysteryGiftAlbum,
    trainer::TrainerInfo,
    utils::constants::{MAX_COINS_DEFAULT, MAX_GIFT_FLAG_COUNT_DEFAULT, MAX_MONEY_DEFAULT}, pokemon::base::PokemonTrait, strings::StringConverterOption,
};

#[derive(Debug, Default)]
pub struct SaveFileMetadata {
    pub file_path: Option<String>,
    pub file_name: Option<String>,

    pub header: Option<Vec<u8>>,
    pub footer: Option<Vec<u8>>,

    pub extension: Option<String>,
}

impl SaveFileMetadata {
    pub fn has_header(&self) -> bool {
        self.header.is_some() && !self.header.as_ref().unwrap().is_empty()
    }

    pub fn has_footer(&self) -> bool {
        self.footer.is_some() && !self.footer.as_ref().unwrap().is_empty()
    }
}

#[derive(Debug, Default)]
pub struct SaveFileOffsets {
    pub party: Option<usize>,
    pub box_start: Option<usize>,
    pub pokedex: Option<usize>,
    pub daycare: Option<usize>,
}

pub struct SaveFileConstants {
    pub ot_length: u8,
    pub nickname_length: u8,
    pub max_move_id: u32,
    pub max_species_id: u16, // TODO: see if is really used
    pub max_ability_id: u16,
    pub max_item_id: u16,
    pub max_pokeball_id: u16,

    pub max_game_id: u16,
    pub min_game_id: u16,

    pub max_gift_count: i32,
    pub max_gift_flag_count: u32,

    pub max_money: u32,
    pub max_coins: u16,

    pub max_party_count: u8,

    pub box_count: u32,
    pub box_size: u16,

    pub pokemon_size_when_stored: u16,
    pub pokemon_size_in_party: u16,
}

impl SaveFileConstants {
    pub const fn default() -> Self {
        Self {
            ot_length: 0,
            nickname_length: 0,
            max_move_id: 0,
            max_species_id: 0, // todo: see if is really used
            max_ability_id: 0,
            max_item_id: 0,
            max_pokeball_id: 0,

            max_game_id: 0,
            min_game_id: 0,

            max_gift_count: 0,
            max_gift_flag_count: MAX_GIFT_FLAG_COUNT_DEFAULT,

            max_money: MAX_MONEY_DEFAULT,
            max_coins: MAX_COINS_DEFAULT,

            max_party_count: 6,

            box_count: 0,
            box_size: 0,

            pokemon_size_when_stored: 0,
            pokemon_size_in_party: 0,
        }
    }
}

pub struct SaveFile {
    pub data: Vec<u8>,
    pub metadata: SaveFileMetadata,

    pub offsets: SaveFileOffsets,
    pub constants: SaveFileConstants,

    pub is_exportable: bool,
    pub is_modified: bool,

    pub generation: Generation,

    pub species_table: SpeciesTable,

    // region: Mystery Gift
    pub wondercard_data: i32,

    pub has_wondercard: bool,

    pub mysterygift_album: MysteryGiftAlbum,
    // endregion

    // region: Player Data
    pub language: LanguageID,
    pub game: GameVersion,

    pub trainer_info: TrainerInfo,

    pub hours_played: u32,
    pub minutes_played: u32,
    pub seconds_played: u32,

    // TODO: see what this means
    pub seconds_to_start: u32,
    pub seconds_to_fame: u32,

    pub money: u32,
    // endregion

    // region: Party
    pub party_count: u8,
    // endregion

    pub current_box: u16,
}

impl Default for SaveFile {
    fn default() -> Self {
        Self {
            data: Default::default(),
            metadata: Default::default(),
            offsets: Default::default(),
            constants: SaveFileConstants::default(),
            is_exportable: Default::default(),
            is_modified: Default::default(),
            generation: Generation::None,
            species_table: SpeciesTable::default(),
            wondercard_data: Default::default(),
            has_wondercard: Default::default(),
            mysterygift_album: Default::default(),
            language: LanguageID::English,
            game: GameVersion::Any,
            trainer_info: Default::default(),
            hours_played: Default::default(),
            minutes_played: Default::default(),
            seconds_played: Default::default(),
            seconds_to_start: Default::default(),
            seconds_to_fame: Default::default(),
            money: Default::default(),
            party_count: Default::default(),
            current_box: Default::default(),
        }
    }
}

pub trait SaveFileTrait where Self::Pokemon: PokemonTrait{
    type Pokemon;

    fn get_base(&self) -> &SaveFile;
    fn mutate_base(&mut self) -> &mut SaveFile;

    fn playtime_string(&self) -> String {
        format!(
            "{}:{}:{}",
            self.get_base().hours_played,
            self.get_base().minutes_played,
            self.get_base().seconds_played
        )
    }

    fn short_summary(&self) -> String;

    fn validate_checksum(&self) -> bool;
    fn set_checksum(&mut self);
    fn get_final_data(&mut self) -> Vec<u8>;
    fn set_checksum_and_return_data(&mut self) -> Vec<u8> {
        self.set_checksum();
        self.get_base().data.to_vec()
    }

    fn get_checksum_info(&self) -> String;

    fn write_to_data(&mut self);
    fn load_from_data(&mut self);

    fn get_flag(&self, offset: usize, bit_index: u8) -> bool;
    fn set_flag(&mut self, offset: usize, bit_index: u8, value: bool);

    fn has_party(&self) -> bool {
        if self.get_base().offsets.party.is_some() {
            return true;
        }

        false
    }

    fn get_party_slot_offset(&self, slot: u8) -> usize;
    fn get_party_slot(&self, data: &[u8], offset: usize) -> Self::Pokemon;

    fn get_pokemon_at_party_slot(&self, data: &[u8], slot: u8) -> Self::Pokemon {
        self.get_party_slot(data, self.get_party_slot_offset(slot)) // TODO: SaveFile.cs:358
    }

    fn is_party_slot_not_egg_or_empty(&self, data: &[u8], slot: u8) -> bool {
        let pokemon_at_slot = self.get_pokemon_at_party_slot(data, slot);
        !pokemon_at_slot.is_egg() && pokemon_at_slot.get_species() != 0
    }

    fn is_party_all_eggs(&self, data: &[u8], except: Option<u8>) -> bool {
        if !self.has_party() {
            return false;
        }

        for i in 0..self.get_base().constants.max_party_count {
            if except.is_some() && except.unwrap() == i {
                continue;
            }

            if self.is_party_slot_not_egg_or_empty(data, i) {
                return false;
            }
        }

        true
    }

    fn is_pokemon_present(&self, data: &[u8]) -> bool;

    fn get_pokemon(data: &[u8]) -> Self::Pokemon;

    fn decrypt_pokemon(data: &[u8]) -> Vec<u8>;

    fn get_box_offset(&self, box_index: usize) -> usize;
    fn get_box_wallpaper_offset(&self, box_index: usize) -> usize;

    fn set_box_wallpaper(&mut self, box_index: usize, value: u8);

    fn get_string(&self, data: &[u8]) -> String;
    fn set_string(&self, buffer: &mut [u8], value: &mut [char], max_length: usize, option: StringConverterOption) -> usize;

    fn get_box_name(&mut self, box_index: usize) -> String;
    fn set_box_name(&mut self, box_index: usize, name: String);

    fn set_pokedex_from_pokemon(&mut self, pokemon: Self::Pokemon);

    fn get_caught(&self, species: u16) -> bool;
    fn set_caught(&mut self, species: u16, caught: bool);

    fn get_seen(&self, species: u16) -> bool;
    fn set_seen(&mut self, species: u16, seen: bool);

    fn get_daycare_slot_offset(&self, loc: u32, slot: usize) -> usize;

    fn is_daycare_occupied(&self, loc: u32, slot: usize) -> bool;
    fn set_daycare_occupied(&mut self, loc: u32, slot: usize, occupied: bool);

    fn get_daycare_exp(&self, loc: u32, slot: usize) -> u32;
    fn set_daycare_exp(&mut self, loc: u32, slot: usize, exp: u32);

    fn get_daycare_has_egg(&mut self, loc: u32) -> bool;
    fn set_daycare_has_egg(&mut self, loc: u32, has_egg: bool);
}
