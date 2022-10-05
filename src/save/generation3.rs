use crate::{
    legality::tables::generation3::*,
    utils::constants::{
        poke_crypto::{SIZE_GENERATION_3_PARTY, SIZE_GENERATION_3_STORED},
        MAX_GIFT_FLAG_COUNT_DEFAULT,
    },
};

use super::base::SaveFileConstants;

pub mod base;
pub mod utils;

impl SaveFileConstants {
    pub const fn generation_3_constants(is_japanese: bool) -> SaveFileConstants {
        let mut result = SaveFileConstants::default();

        result.ot_length = 7;
        result.nickname_length = 10;
        result.max_move_id = MAX_MOVE_ID;
        result.max_species_id = MAX_SPECIES_ID;
        result.max_ability_id = MAX_ABILITY_ID;
        result.max_item_id = MAX_ITEM_ID;
        result.max_pokeball_id = MAX_POKEBALL_ID;
        result.max_game_id = MAX_GAME_ID;
        result.max_gift_count = 1;
        result.max_gift_flag_count = MAX_GIFT_FLAG_COUNT_DEFAULT;
        result.max_money = 999999;
        result.box_count = 14;
        result.box_size = 30;
        result.pokemon_size_when_stored = SIZE_GENERATION_3_STORED;
        result.pokemon_size_in_party = SIZE_GENERATION_3_PARTY;

        result
    }

    pub const fn generation_3_colosseum_constants(is_japanese: bool) -> SaveFileConstants {
        let mut base = Self::generation_3_constants(is_japanese);

        base.max_item_id = MAX_ITEM_ID_COLO;

        base
    }

    pub const fn generation_3_xd_constants(is_japanese: bool) -> SaveFileConstants {
        let mut base = Self::generation_3_constants(is_japanese);

        base.max_item_id = MAX_ITEM_ID_XD;

        base
    }
}

#[derive(Debug, Default)]
pub struct Generation3Offsets {
    mail: usize,

    event_flag: usize,

    egg_event_flag: usize,

    external_event_data: usize,
    external_event_flag: usize,

    seen_2: usize,
    seen_3: usize,
}

impl Generation3Offsets {
    pub fn emerald() -> Self {
        Self {
            mail: todo!(),
            event_flag: todo!(),
            egg_event_flag: todo!(),
            external_event_data: todo!(),
            external_event_flag: 0 + 0x14,
            seen_2: todo!(),
            seen_3: todo!(), // TODO
        }
    }
}
