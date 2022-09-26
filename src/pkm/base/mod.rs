pub mod pkm_utils;

use std::cmp::max;

use crate::{species::base::{SpeciesInfo, SpeciesGenderCategory}, game::enums::{pokemon_gender::PokemonGender, game_version::GameVersion, pokemon_nature::PokemonNature, language_id::LanguageID, species_id::SpeciesID, location::Locations}};

use super::utils::{experience::{get_level, get_minimum_level_experience}, gender::generate_from_pid_and_ratio};

#[allow(dead_code)]
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
    max_iv: u16,
    max_ev: u16,
    ot_name_length: u8,
    nickname_length: u8,
}

/// ID Related implementations
impl PKMBase {
    pub fn is_shiny(&self) -> bool {
        u32::from(self.trainer_secret_id) == self.personality_id
    }

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

    pub fn trainer_public_id_to_display(&self) {
        // TODO
    }

    pub fn trainer_secret_id_to_display() {
        // TODO
    }
}

/// Methods to check the game from where the Pokemon is from
impl PKMBase {
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
impl PKMBase {
    /// Pokemon was transferred from Let's Go Eevee/Let's Go Pikachu
    pub fn is_from_gotransfer_letsgopikachu_letsgoeeve(&self) -> bool {
        self.is_from_go() && self.met_location == Locations::Transfer_Go_LetsGoEeveeLetsGoPikachu as u16
    }

    /// Pokemon was transferred from Go to Home
    pub fn is_from_gotransfer_home(&self) -> bool {
        self.is_from_go() && self.met_location == Locations::Transfer_Go_Home as u16
    }

    pub fn is_from_virtualconsole(&self) -> bool {
        self.is_from_virtualconsole_generation1() || self.is_from_virtualconsole_generation2()
    }

    pub fn is_from_letsgo(&self) -> bool {
        self.is_from_letsgopikachu_letsgoeeve() || self.is_from_gotransfer_letsgopikachu_letsgoeeve()
    }
}

/// Generation related methods
impl PKMBase {
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

    pub fn generation(&self) -> i8 {
        if self.is_from_generation_1() || self.is_from_virtualconsole_generation1() { return 1; }
        if self.is_from_generation_2() || self.is_from_virtualconsole_generation2() { return 2; }
        if self.is_from_generation_3() { return 3; }
        if self.is_from_generation_4() { return 4; }
        if self.is_from_generation_5() { return 5; }
        if self.is_from_generation_6() { return 6; }
        if self.is_from_generation_7() { return 7; }
        if self.is_from_generation_8() { return 8; }

        return -1;
    }
}

/// Pokerus related methods
impl PKMBase {
    pub fn is_infected_pokerus(&self) -> bool {
        self.pokerus_strain != 0
    }

    pub fn is_cured_pokerus(&self) -> bool {
        self.pokerus_days == 0 && self.pokerus_strain > 0
    }
}

/// Stat calculation methods
impl PKMBase {
    pub fn current_level(&self) -> u8 {
        get_level(self.experience, self.species_info.exp_growth.into())
    }

    pub fn set_current_level(&mut self, level: u8) {
        if level >= 100 {
            level = 100;
        }

        if level <= 0 {
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
        let count = 0;

        if self.iv_hitpoints == self.max_iv { count += 1; }
        if self.iv_attack == self.max_iv { count += 1; }
        if self.iv_defense == self.max_iv { count += 1; }
        if self.iv_special_attack == self.max_iv { count += 1; }
        if self.iv_special_defense == self.max_iv { count += 1; }
        if self.iv_speed == self.max_iv { count += 1; }

        count
    }
}

impl PKMBase {
    pub fn pid_ability(&self) -> Option<u32> {
        if self.generation() > 5 {
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

        return self.gender == generate_from_pid_and_ratio(self.personality_id, self.species_info.gender())
    }
}

pub trait PKMTrait {
    fn encrypt(&self) -> Vec<u8>;
    fn write(&self) -> Vec<u8>;
    fn read(data: Vec<u8>) -> Self;

    fn is_nicknamed(&self) -> bool;
    fn is_egg(&self) -> bool;

    fn regenerate_checksum(&self);
    fn validate_checksum(&self);
}
