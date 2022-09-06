pub mod pkm_utils;

use crate::{species::base::SpeciesInfo, game::enums::{pokemon_gender::PokemonGender, game_version::GameVersion, pokemon_nature::PokemonNature, language_id::LanguageID, species_id::SpeciesID}};

#[derive(Debug, Clone)]
pub struct PKMBase {
    // PKMCore/PKHex Specific
	valid_extensions: Vec<String>,
	current_extension: String,

    // Data
	data: Vec<u8>,
	extra_bytes: Vec<u8>,
	valid: bool,

    // Technical
	nickname_bytes: Vec<u8>,
	ot_bytes: Vec<u8>,
	ht_bytes: Vec<u8>,

	size_on_party: u16,
	size_when_stored: u16,

    // Personal
	species_info: SpeciesInfo,

    species: SpeciesID,
    gender: PokemonGender,
    nature: PokemonNature,
    stats_nature: PokemonNature,
    ability: u8,

    nickname: String,
    experience: u32,
    level: u8,
    form: u8,

    current_friendship: u8,

    trainer_public_id: u16,
    ot_name: String,
    ot_gender: u8,

    ball_type: u8,
    met_level: u8,

    // Battle
    move_1: u16,
    move_1_pp: u8,
    move_1_pp_ups: u8,

    move_2: u16,
    move_2_pp: u8,
    move_2_pp_ups: u8,

    move_3: u16,
    move_3_pp: u8,
    move_3_pp_ups: u8,

    move_4: u16,
    move_4_pp: u8,
    move_4_pp_ups: u8,

    ev_hitpoints: u16,
    ev_attack: u16,
    ev_defense: u16,
    ev_speed: u16,
    ev_special_attack: u16,
    ev_special_defense: u16,

    iv_hitpoints: u16,
    iv_attack: u16,
    iv_defense: u16,
    iv_speed: u16,
    iv_special_attack: u16,
    iv_special_defense: u16,

    hitpoints: u16,
    attack: u16,
    defense: u16,
    speed: u16,
    special_attack: u16,
    special_defense: u16,

    status_condition: u8,

    // Hidden
    version: GameVersion,
    trainer_secret_id: u16,

    pokerus_strain: u16,
    pokerus_days: u16,

    encryption_constant: u32,
    personality_id: u32,

    // Misc
    language: LanguageID,
    fateful_encounter: bool,

    /// Trainer ID derived attribute
    tsv: u8,
    /// Personality ID (PID) derived attribute
    psv: u8,
    /// Complex generated attribute
    characteristic: u32,

    mark_value: u16,
    met_location: u16,
    egg_location: u16,

    ot_friendship: u8,

    met_year: u8,
    met_month: u8,
    met_day: u8,

    /// Handler (Current Trainer) Name
    ht_name: String,
    /// Handler (Current Trainer) Gender
    ht_gender: u8,
    /// Handler (Current Trainer) Friendship
    ht_friendship: u8,

    enjoyment: u8,
    fullness: u8,

    ability_number: u16,

    egg_year: u8,
    egg_month: u8,
    egg_day: u8,

    relearn_move_1: u16,
    relearn_move_2: u16,
    relearn_move_3: u16,
    relearn_move_4: u16,

    current_handler: u32,

    max_move_id: u16,
    max_species_id: u16,
    max_item_id: u16,
    max_ball_id: u8,
    max_game_id: GameVersion,
    min_game_id: GameVersion,
    max_iv: u8,
    max_ev: u8,
    ot_name_length: u8,
    nickname_length: u8,
}

impl PKMBase {
    pub fn shiny_xor(&self) -> u32 {
        let upper_bits = (self.personality_id >> 16) ^ (self.trainer_secret_id as u32);

        (self.personality_id * 0xFFFF) ^ (self.trainer_public_id as u32) ^ upper_bits
    }

    /// Trainer public ID for Generation 7+
    pub fn trainer_public_id_new(&self) -> u16 {
        (((self.trainer_public_id as u32) | ((self.trainer_secret_id as u32) << 16)) % 1_000_000) as u16
    }

    /// Trainer secret ID for Generation 7+
    pub fn trainer_secret_id_new(&self) -> u16 {
        (((self.trainer_public_id as u32) | ((self.trainer_secret_id as u32) << 16)) / 1_000_000) as u16
    }

    fn set_id_new(&mut self, sid_new: u16, tid_new: u16) {
        let new_id = (((sid_new as u32) * 1_000_000) + ((tid_new as u32) % 1_000_000)) as u32;
        self.trainer_public_id = new_id as u16;
        self.trainer_secret_id = (new_id >> 16) as u16;
    }

    /// Trainer public ID for Generation 7+
    pub fn set_trainer_public_id_new(&mut self, tid: u16) {
        self.set_id_new(tid, self.trainer_secret_id_new())
    }

    /// Trainer secret ID for Generation 7+
    pub fn set_trainer_secret_id_new(&mut self, sid: u16) {
        self.set_id_new(self.trainer_public_id_new(), sid)
    }
}

pub trait PKMTrait {
    fn encrypt(&self) -> Vec<u8>;
    fn write(&self) -> Vec<u8>;
    fn read(data: Vec<u8>) -> Self;

    fn is_nicknamed(&self) -> bool;
    fn is_egg(&self) -> bool;
}
