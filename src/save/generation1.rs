use self::utils::{FULL_JAPANESE_POKEMON_BOX_COUNT, FULL_INTERNATIONAL_POKEMON_BOX_COUNT };

use super::base::SaveFileConstants;

pub mod red_blue_yellow;
pub mod utils;

pub struct Generation1Offsets {
    pub ot: usize,
    pub pokedex_caught: usize,
    pub pokedex_seen: usize,
    pub items: usize,
    pub money: usize,
    pub rival: usize,
    pub options: usize,
    pub badges: usize,
    pub tid: usize,
    pub pikachu_friendship: usize,
    pub pikachu_beach_score: usize,
    pub printer_brightness: usize,
    pub pc_items: usize,
    pub current_box_index: usize,
    pub coins: usize,
    pub object_spawn_flags: usize, // 2 bytes after Coins
    pub starter: usize,
    pub event_flag: usize,
    pub play_time: usize,
    pub daycare: usize,
    pub party: usize,
    pub current_box: usize,
    pub checksum: usize,
}

impl Generation1Offsets {
    pub fn international() -> Self {
        Generation1Offsets {
            ot: 0x2598,
            pokedex_caught: 0x25A3,
            pokedex_seen: 0x25B6,
            items: 0x25C9,
            money: 0x25F3,
            rival: 0x25F6,
            options: 0x2601,
            badges: 0x2602,
            tid: 0x2605,
            pikachu_friendship: 0x271C,
            pikachu_beach_score: 0x2741,
            printer_brightness: 0x2744,
            pc_items: 0x27E6,
            current_box_index: 0x284C,
            coins: 0x2850,
            object_spawn_flags: 0x2852, // 2 bytes after Coins
            starter: 0x29C3,
            event_flag: 0x29F3,
            play_time: 0x2CED,
            daycare: 0x2CF4,
            party: 0x2F2C,
            current_box: 0x30C0,
            checksum: 0x3523,
        }
    }

    pub fn japanese() -> Self {
        Generation1Offsets {
            ot: 0x2598,
            pokedex_caught: 0x259E,
            pokedex_seen: 0x25B1,
            items: 0x25C4,
            money: 0x25EE,
            rival: 0x25F1,
            options: 0x25F7,
            badges: 0x25F8,
            tid: 0x25FB,
            pikachu_friendship: 0x2712,
            pikachu_beach_score: 0x2737,
            printer_brightness: 0x273A,
            pc_items: 0x27DC,
            current_box_index: 0x2842,
            coins: 0x2846,
            object_spawn_flags: 0x2848, // 2 bytes after Coins
            starter: 0x29B9,
            event_flag: 0x29E9,
            play_time: 0x2CA0,
            daycare: 0x2CA7,
            party: 0x2ED5,
            current_box: 0x302D,
            checksum: 0x3594,
        }
    }
}

impl SaveFileConstants {
    pub const fn generation_1_constants(is_japanese: bool) -> SaveFileConstants {
        SaveFileConstants {
            ot_length: if is_japanese { 5 } else { 7 },
            nickname_length: if is_japanese { 5 } else { 10 },
            max_move_id: 0,
            max_species_id: 0,
            max_ability_id: 0,
            max_item_id: 0,
            max_pokeball_id: 0,
            max_game_id: 0,
            min_game_id: 0,
            max_gift_count: 0,
            max_gift_flag_count: 0,
            max_money: 0,
            max_coins: 0,
            max_party_count: 0,
            box_count: if is_japanese { 8 } else { 12 },
            box_size: if is_japanese { FULL_JAPANESE_POKEMON_BOX_COUNT as u16 } else { FULL_INTERNATIONAL_POKEMON_BOX_COUNT as u16 },
            pokemon_size_when_stored: 0,
            pokemon_size_in_party: 0,
        }
    }
}
