use crate::{
    game::enums::{game_version::GameVersion, generation::Generation, language_id::LanguageID},
    legality::SpeciesTable,
    mystery_gifts::MysteryGiftAlbum,
    trainer::TrainerInfo, pokemon::base::Pokemon,
};

pub struct SaveFileMetadata {
    file_path: Option<String>,
    file_name: Option<String>,

    header: Vec<u8>,
    footer: Vec<u8>,

    extension: Option<String>,
}

impl SaveFileMetadata {
    pub fn has_header(&self) -> bool {
        !self.header.is_empty()
    }

    pub fn has_footer(&self) -> bool {
        !self.footer.is_empty()
    }
}

#[allow(dead_code)]
pub struct SaveFile<T> where T: SpeciesTable {
    data: Vec<u8>,

    generation: Generation,

    species_table: T,

    // region: Mystery Gift
    wondercard_data: i32,

    has_wondercard: bool,

    mysterygift_album: MysteryGiftAlbum,
    // endregion

    // region: Player Data
    language: LanguageID,
    game: GameVersion,

    trainer_info: TrainerInfo,

    hours_played: u32,
    minutes_played: u32,
    seconds_played: u32,

    // TODO: see what this means
    seconds_to_start: u32,
    seconds_to_fame: u32,

    money: u32,
    // endregion

    // region: Party
    party_count: u8,
    // TODO: SaveFile.cs:282
    // endregion
}

pub trait SaveFileTrait {
    // region: Constants
    const OT_LENGTH: u8 = 0;
    const NICKNAME_LENGTH: u8 = 0;
    const MAX_MOVE_ID: u32 = 0;
    const MAX_SPECIES_ID: u16 = 0; // TODO: see if is really used
    const MAX_ABILITY_ID: u16 = 0;
    const MAX_ITEM_ID: u16 = 0;
    const MAX_POKEBALL_ID: u16 = 0;

    const MAX_GAME_ID: u16 = 0;
    const MIN_GAME_ID: u16 = 0;

    const MAX_GIFT_COUNT: i32 = 0;
    const MAX_GIFT_FLAG_COUNT: u32 = 0x800;

    const MAX_MONEY: u32 = 9999999;
    const MAX_COINS: u16 = 999;

    const MAX_PARTY_COUNT: u8 = 6;

    const BOX_COUNT: u32 = 0;
    // endregion

    // region: Offsets
    const PARTY_OFFSET: Option<u32> = Some(0);
    // endregion

    fn short_summary(&self) -> &str;

    fn validate_checksum(&self) -> bool;
    fn write(&self);

    fn playtime_string(&self) -> &str;

    fn has_party(&self) -> bool {
        if Self::PARTY_OFFSET.is_some() {
            return true;
        }

        false
    }

    fn get_party_slot_offset(&self, slot: u8) -> u32;
    fn get_party_slot<T>(&self, data: &[u8], offset: u32) -> T where T: Pokemon ;

    fn get_pokemon_at_party_slot<T>(&self, data: &[u8], slot: u8) -> T where T: Pokemon {
        self.get_party_slot(data, self.get_party_slot_offset(slot)) // SaveFile.cs:358
    }

    fn is_party_slot_not_egg_or_empty<T>(&self, data: &[u8], slot: u8) -> bool where T: Pokemon {
        let pokemon_at_slot: T = self.get_pokemon_at_party_slot::<T>(data, slot);
        !pokemon_at_slot.is_egg() && pokemon_at_slot.get_species() != 0
    }

    fn is_party_all_eggs<T>(&self, data: &[u8],except: Option<u8>) -> bool where T: Pokemon {
        if !self.has_party() {
            return false;
        }

        for i in 0..Self::MAX_PARTY_COUNT {
            if except.is_some() && except.unwrap() == i  {
                continue;
            }

            if self.is_party_slot_not_egg_or_empty::<T>(data, i) {
                return false;
            }
        }

        true
    }
}
