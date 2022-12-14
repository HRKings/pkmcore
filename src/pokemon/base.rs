use std::cmp::max;

use crate::{species::base::{SpeciesInfo, SpeciesGenderCategory}, game::enums::{pokemon_gender::PokemonGender, game_version::GameVersion, pokemon_nature::PokemonNature, language_id::LanguageID, species_id::SpeciesID, location}, trainer::TrainerInfo};

use super::utils::{experience::{get_level, get_minimum_level_experience}, gender::generate_from_pid_and_ratio};

#[derive(Debug, Default)]
pub struct PokemonBase {
    // PKMCore/PKHex Specific
	pub valid_extensions: Vec<String>,
	pub current_extension: String,

    // Data
	pub data: Vec<u8>,
	pub extra_bytes: Vec<u8>,
	pub valid: bool,

    // Technical
	pub nickname_bytes: Vec<u8>,
	/// Original Trainer
    pub ot_bytes: Vec<u8>,
	/// Handler (Current Trainer)
    pub ht_bytes: Vec<u8>,

	pub size_on_party: u16,
	pub size_when_stored: u16,

    // Personal
	pub species_info: SpeciesInfo,

    pub species: SpeciesID,
    pub gender: PokemonGender,
    pub nature: PokemonNature,
    pub stats_nature: PokemonNature,
    pub ability: u8,

    pub nickname: String,
    pub experience: u32,
    pub level: u8,
    pub form: u8,

    pub current_friendship: u8,

    /// Original Trainer
    pub ot_info: TrainerInfo,
    pub ot_friendship: u8,

    pub ball_type: u8,
    pub met_level: u8,

    // Battle
    pub move_1: u16,
    pub move_1_pp: u8,
    pub move_1_pp_ups: u8,

    pub move_2: u16,
    pub move_2_pp: u8,
    pub move_2_pp_ups: u8,

    pub move_3: u16,
    pub move_3_pp: u8,
    pub move_3_pp_ups: u8,

    pub move_4: u16,
    pub move_4_pp: u8,
    pub move_4_pp_ups: u8,

    pub ev_hitpoints: u16,
    pub ev_attack: u16,
    pub ev_defense: u16,
    pub ev_speed: u16,
    pub ev_special_attack: u16,
    pub ev_special_defense: u16,

    pub iv_hitpoints: u16,
    pub iv_attack: u16,
    pub iv_defense: u16,
    pub iv_speed: u16,
    pub iv_special_attack: u16,
    pub iv_special_defense: u16,

    pub hitpoints: u16,
    pub attack: u16,
    pub defense: u16,
    pub speed: u16,
    pub special_attack: u16,
    pub special_defense: u16,

    pub status_condition: u8,

    // Hidden
    pub version: GameVersion,

    pub pokerus_strain: u16,
    pub pokerus_days: u16,

    pub encryption_constant: u32,
    pub personality_id: u32,

    // Misc
    pub language: LanguageID,
    pub fateful_encounter: bool,

    /// Trainer ID derived attribute
    pub tsv: u8,
    /// Personality ID (PID) derived attribute
    pub psv: u8,
    /// Complex generated attribute
    pub characteristic: u32,

    pub mark_value: u16,
    pub met_location: u16,
    pub egg_location: u16,

    pub met_year: u8,
    pub met_month: u8,
    pub met_day: u8,

    /// Handler (Current Trainer) Name
    pub ht_info: TrainerInfo,
    pub ht_friendship: u8,

    pub enjoyment: u8,
    pub fullness: u8,

    pub ability_number: u16,

    pub egg_year: u8,
    pub egg_month: u8,
    pub egg_day: u8,

    pub relearn_move_1: u16,
    pub relearn_move_2: u16,
    pub relearn_move_3: u16,
    pub relearn_move_4: u16,

    pub max_move_id: u16,
    pub max_species_id: u16,
    pub max_item_id: u16,
    pub max_ball_id: u8,
    pub max_game_id: GameVersion,
    pub min_game_id: GameVersion,
    pub max_iv: u16,
    pub max_ev: u16,
    pub ot_name_length: u8,
    pub nickname_length: u8,
}

/// ID Related implementations
impl PokemonBase {
    pub fn is_shiny(&self) -> bool {
        u32::from(self.ot_info.secret_id) == self.personality_id
    }

    pub fn shiny_xor(&self) -> u32 {
        let upper_bits = (self.personality_id >> 16) ^ (self.ot_info.secret_id as u32);

        (self.personality_id * 0xFFFF) ^ (self.ot_info.public_id as u32) ^ upper_bits
    }
}

/// Methods to check the game from where the Pokemon is from
impl PokemonBase {
    pub fn is_from_emerald(&self) -> bool {
        self.version == GameVersion::Emerald
    }

    pub fn is_from_firered_leafgreen(&self) -> bool {
        self.version == GameVersion::FireRed || self.version == GameVersion::LeafGreen
    }

    pub fn is_from_platinum(&self) -> bool {
        self.version == GameVersion::Platinum
    }

    pub fn is_from_heartgold_soulsilver(&self) -> bool {
        self.version == GameVersion::HeartGold || self.version == GameVersion::SoulSilver
    }

    pub fn is_from_black_white(&self) -> bool {
        self.version == GameVersion::Black || self.version == GameVersion::White
    }

    pub fn is_from_black2_white2(&self) -> bool {
        self.version == GameVersion::Black2 || self.version == GameVersion::White2
    }

    pub fn is_from_x_y(&self) -> bool {
        self.version == GameVersion::X || self.version == GameVersion::Y
    }

    pub fn is_from_omegaruby_alphasapphire(&self) -> bool {
        self.version == GameVersion::OmegaRuby || self.version == GameVersion::AlphaSapphire
    }

    pub fn is_from_sun_moon(&self) -> bool {
        self.version == GameVersion::Sun || self.version == GameVersion::Moon
    }

    pub fn is_from_ultrasun_ultramoon(&self) -> bool {
        self.version == GameVersion::UltraSun || self.version == GameVersion::UltraMoon
    }

    pub fn is_from_go(&self) -> bool {
        self.version == GameVersion::Go
    }

    pub fn is_from_virtualconsole_generation1(&self) -> bool {
        self.version >= GameVersion::VirtualRed && self.version <= GameVersion::VirtualYellow
    }

    pub fn is_from_virtualconsole_generation2(&self) -> bool {
        self.version >= GameVersion::VirtualGold && self.version <= GameVersion::VirtualCrystal
    }

    pub fn is_from_letsgopikachu_letsgoeeve(&self) -> bool {
        self.version == GameVersion::LetsGoPikachu || self.version == GameVersion::LetsGoEevee
    }

    pub fn is_from_sword_shield(&self) -> bool {
        self.version == GameVersion::Sword || self.version == GameVersion::Shield
    }

    pub fn is_from_brillianddiamond_shiningpearl(&self) -> bool {
        self.version == GameVersion::BrilliantDiamond || self.version == GameVersion::ShiningPearl
    }

    pub fn is_from_legendsarceus(&self) -> bool {
        self.version == GameVersion::LegendsArceus
    }

}

/// Methods to check for GO transfers, Let's Go games and Virtual Consoles
impl PokemonBase {
    /// Pokemon was transferred from Let's Go Eevee/Let's Go Pikachu
    pub fn is_from_gotransfer_letsgopikachu_letsgoeeve(&self) -> bool {
        self.is_from_go() && self.met_location == location::TRANSFER_GO_LETS_GO_EEVEE_LETS_GO_PIKACHU as u16
    }

    /// Pokemon was transferred from Go to Home
    pub fn is_from_gotransfer_home(&self) -> bool {
        self.is_from_go() && self.met_location == location::TRANSFER_GO_HOME as u16
    }

    pub fn is_from_virtualconsole(&self) -> bool {
        self.is_from_virtualconsole_generation1() || self.is_from_virtualconsole_generation2()
    }

    pub fn is_from_letsgo(&self) -> bool {
        self.is_from_letsgopikachu_letsgoeeve() || self.is_from_gotransfer_letsgopikachu_letsgoeeve()
    }
}

/// Generation related methods
impl PokemonBase {
    pub fn is_from_generation_1(&self) -> bool {
        self.version == GameVersion::GroupRedBlueYellow
    }

    pub fn is_from_generation_2(&self) -> bool {
        self.version == GameVersion::GroupGoldSilverCrystal
    }

    pub fn is_from_generation_3(&self) -> bool {
        (self.version >= GameVersion::Sapphire && self.version <= GameVersion::LeafGreen) || self.version == GameVersion::ColosseumXD
    }

    pub fn is_from_generation_4(&self) -> bool {
        (self.version >= GameVersion::HeartGold && self.version <= GameVersion::Platinum) && self.version !=  GameVersion::Generation4Unused
    }

    pub fn is_from_generation_5(&self) -> bool {
        self.version >= GameVersion::White || self.version <= GameVersion::Black2
    }

    pub fn is_from_generation_6(&self) -> bool {
        self.version >= GameVersion::X || self.version <= GameVersion::OmegaRuby
    }

    pub fn is_from_generation_7(&self) -> bool {
        self.version >= GameVersion::Sun || self.version <= GameVersion::UltraMoon || self.is_from_letsgo()
    }

    pub fn is_from_generation_8(&self) -> bool {
        self.version >= GameVersion::Sword || self.version <= GameVersion::ShiningPearl || self.is_from_gotransfer_home()
    }

    pub fn generation(&self) -> Option<u8> {
        if self.is_from_generation_1() || self.is_from_virtualconsole_generation1() { return Some(1); }
        if self.is_from_generation_2() || self.is_from_virtualconsole_generation2() { return Some(2); }
        if self.is_from_generation_3() { return Some(3); }
        if self.is_from_generation_4() { return Some(4); }
        if self.is_from_generation_5() { return Some(5); }
        if self.is_from_generation_6() { return Some(6); }
        if self.is_from_generation_7() { return Some(7); }
        if self.is_from_generation_8() { return Some(8); }

        None
    }
}

/// Pokerus related methods
impl PokemonBase {
    pub fn is_infected_pokerus(&self) -> bool {
        self.pokerus_strain != 0
    }

    pub fn is_cured_pokerus(&self) -> bool {
        self.pokerus_days == 0 && self.pokerus_strain > 0
    }
}

/// Stat calculation methods
impl PokemonBase {
    pub fn current_level(&self) -> u8 {
        get_level(self.experience, self.species_info.exp_growth.into())
    }

    pub fn set_current_level(&mut self, mut level: u8) {
        if level >= 100 {
            level = 100;
        }

        if level == 0 {
            level = 1;
        }

        self.level = level;
        self.experience = get_minimum_level_experience(level, self.species_info.exp_growth.into())
    }

    pub fn iv_total(&self) -> u16 {
        self.iv_hitpoints + self.iv_attack + self.iv_defense + self.iv_special_attack + self.iv_special_defense + self.iv_speed
    }

    pub fn ev_total(&self) -> u16 {
        self.ev_hitpoints + self.ev_attack + self.ev_defense + self.ev_special_attack + self.ev_special_defense + self.ev_speed
    }

    /// Returns the largest IV
    pub fn maximum_iv(&self) -> u16 {
        max(self.iv_speed,
            max(self.iv_special_defense,
                max(self.iv_special_attack,
                    max(self.iv_defense,
                        max(self.iv_hitpoints,
                                    self.iv_attack)))))
    }

    pub fn flawless_iv_count(&self) -> u8 {
        let mut count = 0;

        if self.iv_hitpoints == self.max_iv { count += 1; }
        if self.iv_attack == self.max_iv { count += 1; }
        if self.iv_defense == self.max_iv { count += 1; }
        if self.iv_special_attack == self.max_iv { count += 1; }
        if self.iv_special_defense == self.max_iv { count += 1; }
        if self.iv_speed == self.max_iv { count += 1; }

        count
    }
}

impl PokemonBase {
    pub fn pid_ability(&self) -> Option<u32> {
        if self.generation() > Some(5) {
            return None;
        }

        if self.version == GameVersion::ColosseumXD {
            return None; // TODO: review this
        }

        if self.is_from_generation_5() {
            Some(self.personality_id >> 16 & 1)
        } else {
            Some(self.personality_id & 1)
        }
    }

    pub fn is_valid_gender(&self) -> bool {
        if self.species_info.gender() == SpeciesGenderCategory::Genderless {
            return self.gender == PokemonGender::Genderless
        }

        if self.species_info.gender() == SpeciesGenderCategory::FemaleOnly {
            return self.gender == PokemonGender::Female
        }

        if self.species_info.gender() == SpeciesGenderCategory::MaleOnly {
            return self.gender == PokemonGender::Male
        }

        if !self.is_from_generation_3() || !self.is_from_generation_4()|| !self.is_from_generation_5() {
            return self.gender as u8 == (self.gender as u8 & 1)
        }

        self.gender == generate_from_pid_and_ratio(self.personality_id, self.species_info.gender_ratio)
    }
}

pub trait PokemonTrait {
    fn get_base(&self) -> &PokemonBase;

    fn decrypt(&self) -> Vec<u8>;
    fn encrypt(&self) -> Vec<u8>;
    fn write(&self) -> Vec<u8>;
    fn read(data: &[u8]) -> Self;

    fn get_species(&self) -> u16;

    fn is_nicknamed(&self) -> bool;
    fn is_egg(&self) -> bool;

    fn regenerate_checksum(&self);
    fn validate_checksum(&self);
}
