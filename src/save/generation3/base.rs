use std::{borrow::Borrow, f32::consts::E};

use crate::{
    game::enums::{generation::Generation, species_id::SpeciesID},
    pokemon::{self, base::PokemonTrait},
    save::{
        base::{SaveFile, SaveFileConstants, SaveFileTrait},
        substructures::{
            generation3,
            mail::{self, MailDetailTrait},
        },
    },
    strings::{self, StringConverterOption},
    utils::{
        constants::poke_crypto::{self, SIZE_GENERATION_3_STORED},
        data_manipulation::{self, FlagTrait, SliceUtils},
        helpers::{checksum_helper, detection_helper},
    },
};

use super::{utils::SAVE_RAW_SIZE, Generation3Offsets};

// Similar to future games, the Generation 3 Mainline save files are comprised of separate objects:
// Object 1 - Small, containing misc configuration data & the Pokédex.
// Object 2 - Large, containing everything else that isn't PC Storage system data.
// Object 3 - Storage, containing all the data for the PC storage system.

// When the objects are serialized to the savedata, the game fragments each object and saves it to a sector.
// The main save data for a save file occupies 14 sectors; there are a total of two serialized main saves.
// After the serialized main save data, there is "extra data", for stuff like Hall of Fame and battle videos.
// Extra data is always at the same sector, while the main sectors rotate sectors within their region (on each successive save?).

// region: Constant
const REVISION_JAPANESE: u8 = 0;
const REVISION_INTERNATIONAL: u8 = 1;

const REVISION_JAPANESE_STRING: char = 'J';
const REVISION_INTERNATIONAL_STRING: char = 'U';

const SIZE_SECTOR: usize = 0x1000;
const SIZE_SECTOR_USED: usize = 0xF80;
/// How many sectors worth of data
const SECTOR_COUNT: usize = 14;
const SIZE_MAIN: usize = SECTOR_COUNT * SIZE_SECTOR;

const BOX_NAME_SIZE: usize = 8 + 1; // 8 characters + 1 terminator
                                    // endregion

pub struct Generation3SaveFileBase {
    pub base: SaveFile,
    pub specific_offsets: Generation3Offsets,

    pub small_data: Vec<u8>,
    pub large_data: Vec<u8>,
    pub storage_data: Vec<u8>,

    pub active_slot: u8,

    pub is_japanese: bool,
    pub save_revision: u8,
    pub save_revision_string: char,

    pub security_key: Option<u32>,

    pub daycare_slot_size: usize,

    pub mystery_event: generation3::mystery_data::MysteryEvent,

    pub pokedex_pid_unown: u32,
    pub pokedex_pid_spinda: u32,

    pub pokedex_sort: u8,
    pub pokedex_mode: u8,

    pub pokedex_national_magicnumber_rubysapphireemerald: u8,
    pub pokedex_national_magicnumber_fireredleafgreen: u8,

    pub colosseum_raw_1: u32,
    pub colosseum_raw_2: u32,
    /// PokéCoupons stored by Pokémon Colosseum and XD from Mt. Battle runs. Earned PokéCoupons are also added to ColosseumCouponsTotal.
    ///
    /// # Remarks
    /// Colosseum/XD caps this at 9,999,999, but will read up to 16,777,215.
    pub colosseum_coupons: u32,
    /// Used by the JP Colosseum bonus disc. Determines PokéCoupon rank to distribute rewards. Unread in International games.
    /// # Remarks
    /// Colosseum/XD caps this at 9,999,999.
    pub colosseum_coupons_total: u32,
    /// PP Max from JP Colosseum Bonus Disc; for reaching 2500 colosseum_coupons_total
    pub colosseum_pokecoupons_title_bronze: bool,
    /// Light Ball Pikachu from JP Colosseum Bonus Disc; for reaching 5000 colosseum_coupons_total
    pub colosseum_pokecoupons_title_silver: bool,
    /// Master Ball from JP Colosseum Bonus Disc; for reaching 30,000 colosseum_coupons_total
    pub colosseum_pokecoupons_title_gold: bool,
    /// Received Celebi Gift from JP Colosseum Bonus Disc
    pub colosseum_received_ageto: bool,

    /// Indicates if this save has connected to RSBOX and triggered the free False Swipe Swablu Egg giveaway.
    pub has_used_rubysapphirebox: bool,
    /// 1 for ExtremeSpeed Zigzagoon (at 100 deposited), 2 for Pay Day Skitty (at 500 deposited), 3 for Surf Pichu (at 1499 deposited)
    pub rubysapphirebox_deposit_eggs_unlocked: u8,
    /// Received Jirachi Gift from Colosseum Bonus Disc
    pub has_received_wishmkr_jirachi: bool,
}

impl SaveFileTrait for Generation3SaveFileBase {
    type Pokemon = pokemon::Generation3;

    fn get_base(&self) -> &SaveFile {
        todo!()
    }

    fn mutate_base(&mut self) -> &mut SaveFile {
        todo!()
    }

    fn short_summary(&self) -> String {
        format!(
            "{} ({}) - {}",
            self.base.trainer_info.name,
            self.base.game,
            self.playtime_string()
        )
    }

    fn validate_checksum(&self) -> bool {
        for i in 0..SECTOR_COUNT {
            if !self.is_sector_valid(i) {
                return false;
            }
        }

        if self.base.data.len() < SAVE_RAW_SIZE {
            // don't check HoF for half-sizes
            return true;
        }

        if !self.is_sector_valid_extra(0x1C000) || !self.is_sector_valid_extra(0x1D000) {
            return false;
        }

        return true;
    }

    fn set_checksum(&mut self) {
        let start = (self.active_slot as usize) * SIZE_MAIN;
        let end = start + SIZE_MAIN;

        (start..end).step_by(SIZE_SECTOR).for_each(|ofs| {
            let sector = self.base.data.get_mutable_offset(ofs, SIZE_SECTOR);
            let checksum = checksum_helper::checksum_32bit(&sector[..SIZE_SECTOR_USED], 0);

            sector.write_into(&checksum.to_le_bytes(), 0xFF6);
        });

        if self.base.data.len() < SAVE_RAW_SIZE {
            return;
        }

        // Hall of Fame Checksums
        {
            let sector = self.base.data.get_mutable_offset(0x1C000, SIZE_SECTOR);
            let checksum = checksum_helper::checksum_32bit(&sector[..SIZE_SECTOR_USED], 0);
            sector.write_into(&checksum.to_le_bytes(), 0xFF4);
        }
        {
            let sector = self.base.data.get_mutable_offset(0x1D000, SIZE_SECTOR);
            let checksum = checksum_helper::checksum_32bit(&sector[..SIZE_SECTOR_USED], 0);
            sector.write_into(&checksum.to_le_bytes(), 0xFF4);
        }
    }

    fn get_final_data(&mut self) -> Vec<u8> {
        self.write_sectors();
        self.set_checksum_and_return_data()
    }

    fn get_checksum_info(&self) -> String {
        let mut checks = Vec::<String>::new();

        (0..SECTOR_COUNT).for_each(|i| {
            if !self.is_sector_valid(i) {
                checks.push(format!("Sector {} @ {:5} invalid.", i, i * SIZE_SECTOR));
            }
        });

        // Don't check HoF for half-sizes
        if self.base.data.len() > SAVE_RAW_SIZE {
            if !self.is_sector_valid_extra(0x1C000) {
                checks.push("HoF first sector invalid.".to_string());
            }

            if !self.is_sector_valid_extra(0x1D000) {
                checks.push("HoF second sector invalid.".to_string());
            }
        }

        if !checks.is_empty() {
            checks.join("\n")
        } else {
            "Checksums are valid.".to_string()
        }
    }

    fn write_to_data(&mut self) {
        self.storage_data[0] = self.base.current_box as u8;

        // Pokedex
        let pokedex_offset = self.base.offsets.pokedex.unwrap();

        self.small_data
            .write_into(&self.pokedex_pid_unown.to_le_bytes(), pokedex_offset + 0x4);
        self.small_data
            .write_into(&self.pokedex_pid_spinda.to_le_bytes(), pokedex_offset + 0x8);

        self.small_data[pokedex_offset + 0x01] = self.pokedex_mode;
        self.small_data[pokedex_offset + 0x02] =
            self.pokedex_national_magicnumber_rubysapphireemerald;
        self.small_data[pokedex_offset + 0x03] = self.pokedex_national_magicnumber_fireredleafgreen;

        // External Event Data
        let external_event_data = self.specific_offsets.external_event_data;
        let external_event_flag = self.specific_offsets.external_event_flag;

        self.colosseum_raw_1 = (self.colosseum_coupons << 8) | (self.colosseum_raw_1 & 0xFF);
        self.colosseum_raw_2 =
            (self.colosseum_raw_2 & (1 << 2)) | (self.colosseum_pokecoupons_title_bronze as u32);
        self.colosseum_raw_2 =
            (self.colosseum_raw_2 & (1 << 1)) | (self.colosseum_pokecoupons_title_silver as u32);
        self.colosseum_raw_2 =
            (self.colosseum_raw_2 & 1) | (self.colosseum_pokecoupons_title_gold as u32);
        self.colosseum_raw_2 = (self.colosseum_coupons_total << 8) | (self.colosseum_raw_2 & 0xFF);
        self.colosseum_raw_2 =
            (self.colosseum_raw_2 & (1 << 3)) | ((self.colosseum_received_ageto as u32) << 3);
        self.set_flag(external_event_flag, 0, self.has_used_rubysapphirebox);
        self.large_data[external_event_flag] = (self.large_data[external_event_flag] & !(3 << 1))
            | ((self.rubysapphirebox_deposit_eggs_unlocked & 3) << 1);
        self.set_flag(
            external_event_flag + 2,
            0,
            self.has_received_wishmkr_jirachi,
        );

        self.large_data.write_into(
            &self.colosseum_raw_2.to_le_bytes(),
            external_event_data + 11,
        );
        self.large_data
            .write_into(&self.colosseum_raw_1.to_le_bytes(), external_event_data + 7);
    }

    fn load_from_data(&mut self) {
        self.base.current_box = self.storage_data[0] as u16;

        // Pokedex
        let pokedex_offset = self.base.offsets.pokedex.unwrap();

        self.pokedex_pid_unown = self
            .small_data
            .get_u32_le_offset(pokedex_offset + 0x4)
            .unwrap();
        self.pokedex_pid_spinda = self
            .small_data
            .get_u32_le_offset(pokedex_offset + 0x8)
            .unwrap();

        self.pokedex_mode = self.small_data[pokedex_offset + 0x01];
        self.pokedex_national_magicnumber_rubysapphireemerald =
            self.small_data[pokedex_offset + 0x02];
        self.pokedex_national_magicnumber_fireredleafgreen = self.small_data[pokedex_offset + 0x03];

        // External Event Data
        let external_event_data_offset = self.specific_offsets.external_event_data;
        let external_event_flag_offset = self.specific_offsets.external_event_flag;

        self.colosseum_raw_1 = self
            .large_data
            .get_u32_le_offset(external_event_data_offset + 7)
            .unwrap();
        self.colosseum_raw_2 = self
            .large_data
            .get_u32_le_offset(external_event_data_offset + 11)
            .unwrap();

        self.colosseum_coupons = self.colosseum_raw_1 >> 8;
        self.colosseum_pokecoupons_title_bronze = (self.colosseum_raw_2 & (1 << 2)) != 0;
        self.colosseum_pokecoupons_title_silver = (self.colosseum_raw_2 & (1 << 1)) != 0;
        self.colosseum_pokecoupons_title_gold = (self.colosseum_raw_2 & (1 << 0)) != 0;
        self.colosseum_received_ageto = (self.colosseum_raw_2 & (1 << 3)) != 0;
        self.colosseum_coupons_total = self.colosseum_raw_2 >> 8;
        self.has_used_rubysapphirebox = self.get_flag(external_event_flag_offset, 0);
        self.rubysapphirebox_deposit_eggs_unlocked =
            (self.large_data[external_event_flag_offset] >> 1) & 3;
        self.has_received_wishmkr_jirachi = self.get_flag(external_event_flag_offset + 2, 0);
    }

    fn get_flag(&self, offset: usize, bit_index: u8) -> bool {
        self.large_data.get_flag(offset, bit_index)
    }

    fn set_flag(&mut self, offset: usize, bit_index: u8, value: bool) {
        self.large_data.set_flag(offset, bit_index, value);
    }

    fn has_party(&self) -> bool {
        true
    }

    fn get_party_slot_offset(&self, slot: u8) -> usize {
        todo!()
    }

    fn get_party_slot(&self, data: &[u8], offset: usize) -> Self::Pokemon {
        todo!()
    }

    fn is_pokemon_present(&self, data: &[u8]) -> bool {
        detection_helper::is_pokemon_present_gba(data)
    }

    fn get_pokemon(data: &[u8]) -> Self::Pokemon {
        Self::Pokemon::read(data)
    }

    fn decrypt_pokemon(data: &[u8]) -> Vec<u8> {
        poke_crypto::decrypt_generation3_array(data)
    }

    fn get_box_offset(&self, box_index: usize) -> usize {
        self.base.offsets.box_start.unwrap()
            + 4
            + (SIZE_GENERATION_3_STORED as usize
                * box_index
                * self.base.constants.box_size as usize)
    }

    fn get_box_wallpaper_offset(&self, box_index: usize) -> usize {
        let box_count = self.base.constants.box_count as usize;
        self.get_box_offset(box_count) + (box_count * BOX_NAME_SIZE) + box_index
    }

    fn set_box_wallpaper(&mut self, box_index: usize, value: u8) {
        if box_index > self.base.constants.box_count as usize {
            return;
        }

        let offset = self.get_box_wallpaper_offset(box_index);
        self.storage_data[offset] = value;
    }

    fn get_string(&self, data: &[u8]) -> String {
        strings::generation_3::get_string(data, self.is_japanese)
    }

    fn set_string(
        &self,
        buffer: &mut [u8],
        value: &mut [char],
        max_length: usize,
        option: StringConverterOption,
    ) -> usize {
        strings::generation_3::set_string(buffer, value, max_length, self.is_japanese, option)
    }

    fn get_box_name(&mut self, box_index: usize) -> String {
        let offset = self.get_box_offset(box_index);
        self.get_string(
            self.storage_data
                .get_offset(offset + (box_index * BOX_NAME_SIZE), BOX_NAME_SIZE),
        )
    }

    fn set_box_name(&mut self, box_index: usize, name: String) {
        let offset = self.get_box_offset(box_index);
        let box_name_data = self
            .storage_data
            .get_mutable_offset(offset + (box_index * BOX_NAME_SIZE), BOX_NAME_SIZE);

        strings::generation_3::set_string(
            box_name_data,
            &mut name.chars().collect::<Vec<char>>(),
            BOX_NAME_SIZE - 1,
            self.is_japanese,
            StringConverterOption::ClearZero,
        );
    }

    fn set_pokedex_from_pokemon(&mut self, pokemon: Self::Pokemon) {
        let species = pokemon.base.species as u16;

        if species == 0 || species > self.base.constants.max_species_id {
            return;
        }

        if pokemon.is_egg() {
            return;
        }

        if pokemon.base.species == SpeciesID::Unown && !self.get_seen(species) {
            self.pokedex_pid_unown = pokemon.base.personality_id;
        }

        if pokemon.base.species == SpeciesID::Spinda && !self.get_seen(species) {
            self.pokedex_pid_spinda = pokemon.base.personality_id;
        }

        self.set_caught(species, true);
        self.set_seen(species, true);
    }

    fn get_caught(&self, species: u16) -> bool {
        let bit = species - 1;
        let offset = (bit >> 3) as usize;
        let caught_offset = self.base.offsets.pokedex.unwrap() + 0x10;

        self.small_data
            .get_flag(caught_offset + offset, (bit & 7) as u8)
    }

    fn set_caught(&mut self, species: u16, caught: bool) {
        let bit = species - 1;
        let offset = (bit >> 3) as usize;

        let caught_offset = self.base.offsets.pokedex.unwrap() + 0x10;

        self.small_data
            .set_flag(caught_offset + offset, (bit & 7) as u8, caught);
    }

    fn get_seen(&self, species: u16) -> bool {
        let bit = species - 1;
        let offset = (bit >> 3) as usize;
        let seen_offset = self.base.offsets.pokedex.unwrap() + 0x44;

        self.small_data
            .get_flag(seen_offset + offset, (bit & 7) as u8)
    }

    fn set_seen(&mut self, species: u16, seen: bool) {
        let bit = species - 1;
        let offset = (bit >> 3) as usize;

        let seen_offset = self.base.offsets.pokedex.unwrap() + 0x44;

        self.small_data
            .set_flag(seen_offset + offset, (bit & 7) as u8, seen);
        self.large_data
            .set_flag(self.specific_offsets.seen_2 + offset, (bit & 7) as u8, seen);
        self.large_data
            .set_flag(self.specific_offsets.seen_3 + offset, (bit & 7) as u8, seen);
    }

    fn get_daycare_has_egg(&mut self, loc: u32) -> bool {
        self.get_event_flag(self.specific_offsets.egg_event_flag as u32)
    }

    fn set_daycare_has_egg(&mut self, loc: u32, has_egg: bool) {
        self.set_event_flag(self.specific_offsets.egg_event_flag as u32, has_egg);
    }

    fn get_daycare_slot_offset(&self, loc: u32, slot: usize) -> usize {
        self.base.offsets.daycare.unwrap() + (slot * self.daycare_slot_size)
    }

    fn is_daycare_occupied(&self, loc: u32, slot: usize) -> bool {
        self.is_pokemon_present(&self.large_data[self.get_daycare_slot_offset(loc, slot)..])
    }
    fn set_daycare_occupied(&mut self, loc: u32, slot: usize, occupied: bool) {
        todo!()
    }

    fn get_daycare_exp(&self, loc: u32, slot: usize) -> u32 {
        self.large_data
            .get_u32_le_offset(self.get_daycare_slot_offset(loc, slot))
            .unwrap()
    }

    fn set_daycare_exp(&mut self, loc: u32, slot: usize, exp: u32) {
        self.large_data
            .write_into(&exp.to_le_bytes(), self.get_daycare_slot_offset(loc, slot));
    }
}

impl Generation3SaveFileBase {
    fn is_sector_valid(&self, sectorIndex: usize) -> bool {
        let start = self.active_slot as usize * SIZE_MAIN;
        let offset = start + (sectorIndex * SIZE_SECTOR);

        let sector = self.base.data.get_offset(offset, SIZE_SECTOR);
        let checksum = checksum_helper::checksum_32bit(&sector[..SIZE_SECTOR_USED], 0);

        checksum == sector.get_u16_le_offset(0xFF6).unwrap()
    }

    fn is_sector_valid_extra(&self, ofs: usize) -> bool {
        let sector = self.base.data.get_offset(ofs, SIZE_SECTOR);
        let checksum = checksum_helper::checksum_32bit(&sector[..SIZE_SECTOR_USED], 0);

        checksum == sector.get_u16_le_offset(0xFF4).unwrap()
    }

    fn read_sectors(&mut self) {
        let start = self.active_slot as usize * SIZE_MAIN;
        let end = start + SIZE_MAIN;

        let save_data = &self.base.data;
        let small_data = &mut self.small_data;
        let large_data = &mut self.large_data;

        (start..end).step_by(SIZE_SECTOR).for_each(|offset| {
            let sector_id = save_data.get_u16_le_offset(offset + 0xFF4).unwrap() as usize;

            if sector_id >= 5 {
                small_data.write_into(
                    save_data.get_offset(offset, SIZE_SECTOR_USED),
                    (sector_id - 5) * SIZE_SECTOR_USED,
                )
            } else if sector_id >= 1 {
                large_data.write_into(
                    save_data.get_offset(offset, SIZE_SECTOR_USED),
                    (sector_id - 1) * SIZE_SECTOR_USED,
                );
            } else {
                small_data.write_into(save_data.get_offset(offset, SIZE_SECTOR_USED), 0);
            }
        });
    }

    fn write_sectors(&mut self) {
        let start = self.active_slot as usize * SIZE_MAIN;
        let end = start + SIZE_MAIN;

        let save_data = &mut self.base.data;
        let small_data = &self.small_data;
        let large_data = &self.large_data;

        (start..end).step_by(SIZE_SECTOR).for_each(|offset| {
            let sector_id = save_data.get_u16_le_offset(offset + 0xFF4).unwrap() as usize;

            if sector_id >= 5 {
                save_data.write_into(
                    small_data.get_offset(offset, SIZE_SECTOR_USED),
                    (sector_id - 5) * SIZE_SECTOR_USED,
                );
            } else if sector_id >= 1 {
                save_data.write_into(
                    large_data.get_offset(offset, SIZE_SECTOR_USED),
                    (sector_id - 1) * SIZE_SECTOR_USED,
                );
            } else {
                save_data.write_into(small_data.get_offset(offset, SIZE_SECTOR_USED), 0);
            }
        });
    }

    pub fn is_all_sectors_present(&self, save_slot: u8) -> (bool, usize) {
        let start = SIZE_MAIN * save_slot as usize;
        let end = start + SIZE_MAIN;

        let mut bit_track: usize = 0;
        let mut sector_0_offset: usize = 0;

        let save_data = &self.base.data;

        (start..end).step_by(SIZE_SECTOR).for_each(|offset| {
            let sector_id = save_data.get_u16_le_offset(offset + 0xFF4).unwrap() as usize;
            bit_track |= 1 << sector_id;

            if sector_id == 0 {
                sector_0_offset = offset;
            }
        });

        (bit_track == 0b_0011_1111_1111_1111, sector_0_offset)
    }

    fn get_active_slot(&self) -> u8 {
        if self.base.data.len() == SAVE_RAW_SIZE {
            return 0;
        }

        let (is_all_sectors_present_in_slot_0, initial_sector_offset_slot_0) =
            self.is_all_sectors_present(0);
        let (is_all_sectors_present_in_slot_1, initial_sector_offset_slot_1) =
            self.is_all_sectors_present(1);

        if !is_all_sectors_present_in_slot_0 {
            // If both slots are incomplete, use the Slot 0
            return if is_all_sectors_present_in_slot_1 {
                1
            } else {
                0
            };
        }

        if !is_all_sectors_present_in_slot_1 {
            return 0;
        }

        let save_count_slot_0 = self
            .base
            .data
            .get_u16_le_offset(initial_sector_offset_slot_0 + 0x0FFC)
            .unwrap();
        let save_count_slot_1 = self
            .base
            .data
            .get_u16_le_offset(initial_sector_offset_slot_1 + 0x0FFC)
            .unwrap();

        if save_count_slot_1 > save_count_slot_0 {
            1
        } else {
            0
        }
    }

    pub fn initilize(data: Vec<u8>) -> Self {
        let is_japanese = false;

        let mut result = Generation3SaveFileBase {
            base: Default::default(),
            specific_offsets: Default::default(),
            small_data: Default::default(),
            large_data: Default::default(),
            storage_data: Default::default(),
            active_slot: Default::default(),
            is_japanese,
            save_revision: if is_japanese {
                REVISION_JAPANESE
            } else {
                REVISION_INTERNATIONAL
            },
            save_revision_string: if is_japanese {
                REVISION_JAPANESE_STRING
            } else {
                REVISION_INTERNATIONAL_STRING
            },
            security_key: None,
            mystery_event: Default::default(),
            colosseum_raw_1: Default::default(),
            colosseum_raw_2: Default::default(),
            colosseum_coupons: Default::default(),
            colosseum_coupons_total: Default::default(),
            colosseum_pokecoupons_title_bronze: Default::default(),
            colosseum_pokecoupons_title_silver: Default::default(),
            colosseum_pokecoupons_title_gold: Default::default(),
            colosseum_received_ageto: Default::default(),
            has_used_rubysapphirebox: Default::default(),
            rubysapphirebox_deposit_eggs_unlocked: Default::default(),
            has_received_wishmkr_jirachi: Default::default(),
            pokedex_sort: Default::default(),
            pokedex_mode: Default::default(),
            pokedex_national_magicnumber_rubysapphireemerald: Default::default(),
            pokedex_national_magicnumber_fireredleafgreen: Default::default(),
            pokedex_pid_unown: Default::default(),
            pokedex_pid_spinda: Default::default(),
            daycare_slot_size: Default::default(),
        };

        result.base.data = data;
        result.base.constants = SaveFileConstants::generation_3_constants(is_japanese);
        result.base.generation = Generation::G3;
        result.active_slot = result.get_active_slot();

        result
    }

    pub fn get_mail_offset(&self, index: usize) -> usize {
        (index * mail::SIZE_GENERATION_3) + self.specific_offsets.mail
    }

    pub fn get_mail(&self, index: usize) -> mail::Generation3Mail {
        let offset = self.get_mail_offset(index);
        let data = self.large_data.get_offset(offset, mail::SIZE_GENERATION_3);

        mail::Generation3Mail::new(data, offset, self.is_japanese)
    }

    pub fn get_hall_of_fame_data(&self) -> Vec<u8> {
        // HoF Data is split across two sectors
        let mut data = Vec::with_capacity(SIZE_SECTOR_USED * 2);
        data.write_into(self.base.data.get_offset(0x1C000, SIZE_SECTOR_USED), 0);
        data.write_into(
            self.base.data.get_offset(0x1D000, SIZE_SECTOR_USED),
            SIZE_SECTOR_USED,
        );

        data
    }

    pub fn set_hall_of_fame_data(&mut self, data: &[u8]) {
        assert_eq!(data.len(), SIZE_SECTOR_USED * 2);

        self.base
            .data
            .write_into(&data[..SIZE_SECTOR_USED], 0x1C000);
        self.base
            .data
            .write_into(&data[SIZE_SECTOR_USED..SIZE_SECTOR_USED], 0x1D000);
    }

    pub fn is_corrupt_pokedex_ff(&self) -> bool {
        u64::from_le_bytes(self.small_data[0xAC..(0xAC + 4)].try_into().unwrap()) == u64::MAX
    }

    pub fn get_event_flag(&self, flag_number: u32) -> bool {
        assert!(flag_number as usize >= self.specific_offsets.event_flag);

        self.get_flag(
            self.specific_offsets.event_flag + ((flag_number as usize) >> 3),
            (flag_number & 7) as u8,
        )
    }

    pub fn set_event_flag(&mut self, flag_number: u32, value: bool) {
        assert!(flag_number as usize >= self.specific_offsets.event_flag);

        self.set_flag(
            self.specific_offsets.event_flag + ((flag_number as usize) >> 3),
            (flag_number & 7) as u8,
            value,
        );
    }
}

pub trait Generation3SaveFileTrait {
    fn get_daycare_exp_offset(&self) -> usize;

    fn get_event_flag_count(&self) -> usize;
    fn get_event_work_count(&self) -> usize;
    fn get_event_flag(&self) -> usize;
    fn get_event_work(&self) -> usize;

    fn get_eberry_name(&self) -> String;
    fn is_eberry_enigma(&self) -> String;
}
