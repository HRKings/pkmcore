use crate::{
    game::enums::{game_version::GameVersion, generation::Generation, language_id::LanguageID},
    mystery_gifts::{MysteryGift, MysteryGiftAlbum},
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
pub struct SaveFile {
    data: Vec<u8>,

    generation: Generation,

    // region: Offsets
    ot_length: u8,
    nickname_length: u8,
    max_move_id: u32,
    max_species_id: u16,
    max_ability_id: u16,
    max_item_id: u16,
    max_pokeball_id: u16,
    max_game_id: u16,
    min_game_id: u16,
    // endregion

    // region: Mystery Gift
    max_gift_count:i32,

    max_gift_flag_count:u32,

    wondercard_data:i32,

    has_wondercard: bool,

    mysterygift_received_flags: Vec<bool>,
    mysterygift_data:Vec<MysteryGift>,
    mysterygift_album:MysteryGiftAlbum,
    // endregion

    // region: Player Data
    gender:u8,
    language:LanguageID,
    game:GameVersion,

    trainer_public_id:u16,
    trainer_secret_id:u16,
    /// OT name
    trainer_name: String,

    hours_played:u32,
    minutes_played:u32,
    seconds_played:u32,
    money:u32,
    box_count:u32,

    // TODO
    trainer_id7:u32,
    trainer_sid7: u32,
    max_money:u32,
    max_coins:u32,
    // TODO
    display_trainer_id: u32,
    display_trainer_sid7: u32,
    // endregion

    // region: Party
    party_count:i8,
    party:i32,
    has_party:bool,
    get_party_offset: u8,

    // TODO: SaveFile.cs:282
    // endregion
}

pub trait SaveFileTrait {
    fn write(&self);
}
