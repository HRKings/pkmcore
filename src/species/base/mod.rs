use rand::Rng;

use crate::{
    game::enums::pokemon_gender::PokemonGender,
    utils::constants::{MAGIC_RATIO_FEMALE, MAGIC_RATIO_GENDERLESS, MAGIC_RATIO_MALE},
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpeciesInfo {
    data: Vec<u8>,

    hitpoints: u16,
    attack: u16,
    defense: u16,
    speed: u16,
    special_attack: u16,
    special_defense: u16,

    ev_hitpoints: u16,
    ev_attack: u16,
    ev_defense: u16,
    ev_speed: u16,
    ev_special_attack: u16,
    ev_special_defense: u16,

    type_1: u8,
    type_2: u8,

    egg_group_1: u8,
    egg_group_2: u8,

    catch_rate: u8,

    evolution_stage: u8,

    pub gender_ratio: u8,

    valid_random_held_items: Vec<u16>,

    hatch_cycles: u16,

    base_friendship: u8,

    base_exp: u16,
    pub exp_growth: u8,

    valid_abilities: u8,

    escape_rate: u16,

    form_count: u8,
    form_index: u8,
    form_sprite: u8,

    color: u8,

    height: u16,
    weight: u16,

    compatible_tm_hm: Vec<bool>,

    type_tutors: Vec<bool>,
    special_type_tutors: Vec<bool>,
}

#[derive(PartialEq, Eq)]
pub enum SpeciesGenderCategory {
    Genderless = 2,
    FemaleOnly = 1,
    MaleOnly = 0,
    DualGender = -1,
}

impl SpeciesInfo {
    pub fn is_present_ingame(&self) -> bool {
        self.hitpoints != 0
    }

    pub fn gender(&self) -> SpeciesGenderCategory {
        match self.gender_ratio {
            MAGIC_RATIO_GENDERLESS => SpeciesGenderCategory::Genderless,
            MAGIC_RATIO_FEMALE => SpeciesGenderCategory::FemaleOnly,
            MAGIC_RATIO_MALE => SpeciesGenderCategory::MaleOnly,
            _ => SpeciesGenderCategory::DualGender,
        }
    }

    pub fn has_forms(&self) -> bool {
        self.form_count > 0
    }

    pub fn base_stat_total(&self) -> u16 {
        self.hitpoints
            + self.attack
            + self.defense
            + self.speed
            + self.special_attack
            + self.special_defense
    }

    pub fn random_gender(&self) -> PokemonGender {
        match self.gender() {
            SpeciesGenderCategory::Genderless => PokemonGender::Genderless,
            SpeciesGenderCategory::MaleOnly => PokemonGender::Male,
            SpeciesGenderCategory::FemaleOnly => PokemonGender::Female,
            _ => {
                // `gender_ratio` is the probability out of 254 (MAGIC_RATIO_FEMALE) of a Pokemon being female
                // this generate a random gender accordingly
                if rand::thread_rng().gen_ratio(self.gender_ratio.into(), MAGIC_RATIO_FEMALE.into()) {
                    PokemonGender::Female
                } else {
                    PokemonGender::Male
                }
            }
        }
    }
}

trait SpeciesTrait {
    fn ability_index(ability_id: u16) -> u16;
}
