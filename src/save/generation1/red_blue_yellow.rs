// use crate::{
//     game::enums::game_version::GameVersion,
//     legality::species_table::SpeciesTable,
//     pokemon,
//     save::{
//         base::{SaveFile, SaveFileTrait, SaveFileConstants},
//         shared::pokemon_list::{Generation1PokemonList, PokemonList},
//     },
//     utils::{constants::poke_crypto::SIZE_GENERATION_1_PARTY, data_manipulation::SliceUtils},
// };

// use super::{
//     Generation1Offsets,
// };

// // region: Constants
// const YELLOW_PIKACHU_STARTER: u8 = 0x54;

// const DATA_RESERVED_SIZE: u16 = 32768;
// // endregion

// pub struct RedBlueYellowSaveFile {
//     pub base: SaveFile,

//     pub is_japanese: bool,

//     pub specific_offsets: Generation1Offsets,

//     pub is_yellow: bool,

//     pub current_box: u8,
// }

// impl RedBlueYellowSaveFile {
//     pub fn is_virtual_console(&self) -> bool {
//         self.base.is_exportable
//             && self.base.metadata.extension.is_some()
//             && self
//                 .base
//                 .metadata
//                 .extension
//                 .as_ref()
//                 .unwrap()
//                 .starts_with("sav")
//             && self
//                 .base
//                 .metadata
//                 .extension
//                 .as_ref()
//                 .unwrap()
//                 .contains(".dat")
//     }

//     pub fn save_revision(&self) -> u8 {
//         if self.is_japanese {
//             0
//         } else {
//             1
//         }
//     }

//     pub fn save_revision_string(&self) -> String {
//         format!(
//             "{}{}",
//             if self.is_japanese { "J" } else { "U" },
//             if self.is_virtual_console() {
//                 "VC"
//             } else {
//                 "GB"
//             }
//         )
//     }

//     fn unpack_box(
//         &mut self,
//         box_start_offset: usize,
//         current_box_offset: usize,
//         box_size: u16,
//         box_index: u8,
//         box_capacity: u16,
//     ) {
//         todo!()
//     }

//     fn initialize(&mut self) {
//         self.base.offsets.box_start = Some(self.base.data.len());
//         self.base.data
//             .resize(self.base.data.len() + DATA_RESERVED_SIZE as usize, u8::default());
//         self.base.offsets.party = Some(self.get_party_slot_offset(0));

//         // SAV1.cs:64
//         for current_box_index in 0..self.base.constants.box_count {
//             let current_box_offset = 0; // TODO: SAV1.cs:70
//             self.unpack_box(
//                 current_box_offset,
//                 self.base.offsets.box_start.unwrap() as usize,
//                 self.base.constants.pokemon_size_when_stored,
//                 current_box_index as u8,
//                 self.base.constants.box_size,
//             );
//         }

//         if (self.current_box as u32) < self.base.constants.box_count {
//             self.unpack_box(
//                 self.specific_offsets.current_box,
//                 self.base.offsets.box_start.unwrap() as usize,
//                 self.base.constants.pokemon_size_when_stored,
//                 self.current_box,
//                 self.base.constants.box_size,
//             );
//         }

//         let party_data = self.base.data.get_offset(
//             self.base.offsets.party.unwrap() as usize,
//             super::utils::get_data_length(
//                 SIZE_GENERATION_1_PARTY,
//                 self.is_japanese,
//                 self.base.constants.max_party_count.into(),
//             ),
//         );
//         let party_pokemon_list = Generation1PokemonList::new(
//             Some(party_data.to_vec()),
//             self.base.constants.max_party_count,
//             self.is_japanese,
//             self.base.constants.max_party_count.into(),
//         );

//         (0..(party_pokemon_list.get_base().pokemon.len() as u8)).for_each(|i| {
//             let detination_offset = self.get_party_slot_offset(i);
//         })
//     }
// }

// impl SaveFileTrait for RedBlueYellowSaveFile {
//     type Pokemon = pokemon::Generation1;

//     fn initialize_data(&mut self) {
//         todo!()
//     }

//     fn get_base(&self) -> &SaveFile {
//         &self.base
//     }

//     fn mutate_base(&mut self) -> &mut SaveFile {
//         &mut self.base
//     }

//     fn short_summary(&self) -> String {
//         format!(
//             "{} ({}) - {}",
//             self.base.trainer_info.name,
//             self.base.game,
//             self.playtime_string()
//         )
//     }

//     fn validate_checksum(&self) -> bool {
//         todo!()
//     }

//     fn set_checksum(&mut self) {
//         todo!()
//     }

//     fn get_final_data(&mut self) -> Vec<u8> {
//         todo!()
//     }

//     fn write(&self) -> Vec<u8> {
//         todo!()
//     }

//     fn load(data: &[u8]) -> Self {
//         todo!()
//     }

//     fn get_party_slot_offset(&self, slot: u8) -> usize {
//         self.base.data.len() - DATA_RESERVED_SIZE as usize
//             + (self.base.constants.box_count as usize * self.base.constants.box_size as usize)
//             + (slot as usize * self.base.constants.pokemon_size_when_stored as usize)
//     }

//     fn get_party_slot(&self, data: &[u8], offset: usize) -> pokemon::Generation1 {
//         todo!()
//     }

//     fn is_pokemon_present(&self, data: &[u8]) -> bool {
//         todo!()
//     }

//     fn get_pokemon(data: &[u8]) -> pokemon::Generation1 {
//         todo!()
//     }

//     fn decrypt_pokemon(data: &[u8]) -> Vec<u8> {
//         todo!()
//     }
// }

// pub fn get_correct_version(
//     data: Vec<u8>,
//     version_override: GameVersion,
// ) -> Option<RedBlueYellowSaveFile> {
//     let is_japanese = super::utils::is_generation_1_japanese(&data);
//     let offsets = if is_japanese {
//         Generation1Offsets::japanese()
//     } else {
//         Generation1Offsets::international()
//     };

//     // Try to get the version from the save data
//     let version = if version_override == GameVersion::Any {
//         super::utils::is_generation_1(&data)
//     } else {
//         version_override.clone()
//     };

//     if version == GameVersion::Invalid {
//         return None;
//     }

//     // Create the base save file
//     let mut result = RedBlueYellowSaveFile {
//         base: Default::default(),
//         is_japanese,
//         specific_offsets: offsets,
//         is_yellow: false,
//         current_box: 0,
//     };
//     result.base.data = data;
//     result.base.species_table = SpeciesTable::red_or_blue();
//     result.base.constants = SaveFileConstants::generation_1_constants(is_japanese);

//     // Get the starter to check for Yellow version
//     let starter = result.base.data[result.specific_offsets.starter];

//     // See if Red, Blue and Yellow can be differentiated
//     if starter != 0
//         && (version_override != GameVersion::GroupRedBlueGreen
//             || version_override != GameVersion::GroupRedBlueYellow)
//         && starter == YELLOW_PIKACHU_STARTER
//     {
//         result.is_yellow = true;
//         result.base.species_table = SpeciesTable::yellow();
//     }

//     Some(result)
// }
